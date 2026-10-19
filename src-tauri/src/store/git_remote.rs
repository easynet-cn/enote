//! Git 远程同步编排层
//!
//! 在 [`git_sync`](super::git_sync) 的原子命令之上，组织「连接时同步」与
//! 「提交后推送」两条流程，覆盖三类场景：
//!
//! 1. 首次新建：本地目录不存在 / 为空 + 有远程地址 → `clone`；
//! 2. 重新设置：已有本地仓库 → 校正 origin 地址 → `fetch` + `pull --rebase` + `push`；
//! 3. 目录非空但不是 git 仓库 → 明确报错，不强行初始化。
//!
//! 设计约定：
//! - 离线优先：远程操作失败由调用方告警（emit `git-sync-result` 事件），
//!   不阻塞本地使用；
//! - 所有网络操作与本地提交共用一把进程内锁，串行化同一进程内的 git 调用，
//!   避免并发触发 `.git/index.lock` 冲突；
//! - push 被拒绝时不自动 force，冲突时先 rebase --abort 回滚。

use std::path::Path;
use std::sync::{Mutex, MutexGuard, OnceLock};

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use super::git_sync::{self, GitCredentials};

/// 推送给前端的同步结果事件名
pub const SYNC_EVENT: &str = "git-sync-result";

/// Git 远程运行时配置（密码仅运行时持有，来自 Keychain，不落配置文件）
#[derive(Debug, Clone, Default)]
pub struct GitRemoteConfig {
    /// 远程仓库地址
    pub url: String,
    /// 认证方式
    pub credentials: GitCredentials,
}

/// 同步结果负载（camelCase，直接供前端消费）
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitSyncPayload {
    /// 是否成功
    pub ok: bool,
    /// 阶段标识：clone / fetch / pull / push / sync
    pub operation: String,
    /// 错误码（成功时为 None），前端按 errorCodes.* 翻译
    pub code: Option<String>,
    /// 详情（git 原始 stderr 或成功描述）
    pub message: String,
}

/// 同步错误：携带前端可翻译的错误码与 git 原始信息
#[derive(Debug)]
pub struct GitSyncError {
    /// 错误码（GIT_*）
    pub code: String,
    /// 详情（一般为 git stderr 原文）
    pub message: String,
}

impl GitSyncError {
    fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
        }
    }

    /// 从 anyhow 错误（git_sync 命令失败）按 stderr 归类
    fn from_git(err: &anyhow::Error) -> Self {
        let msg = err.to_string();
        Self::new(git_sync::classify_git_error(&msg), msg)
    }
}

impl std::fmt::Display for GitSyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for GitSyncError {}

/// 进程内 git 操作全局锁：所有网络同步与本地提交均串行执行
static GIT_OP_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// 获取 git 操作锁（中毒时直接恢复，单进程内不会有跨线程数据损坏）
pub fn lock_git_ops() -> MutexGuard<'static, ()> {
    GIT_OP_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// 发送同步结果事件到前端（发送失败仅忽略，不影响主流程）
fn emit_result(app_handle: Option<&AppHandle>, payload: GitSyncPayload) {
    if let Some(handle) = app_handle {
        let _ = handle.emit(SYNC_EVENT, payload);
    }
}

fn emit_success(app_handle: Option<&AppHandle>, operation: &str, message: &str) {
    emit_result(
        app_handle,
        GitSyncPayload {
            ok: true,
            operation: operation.to_string(),
            code: None,
            message: message.to_string(),
        },
    )
}

fn emit_failure(app_handle: Option<&AppHandle>, operation: &str, err: &GitSyncError) {
    emit_result(
        app_handle,
        GitSyncPayload {
            ok: false,
            operation: operation.to_string(),
            code: Some(err.code.clone()),
            message: err.message.clone(),
        },
    )
}

/// 判断目录是否存在且非空
fn is_non_empty_dir(path: &Path) -> bool {
    std::fs::read_dir(path)
        .map(|mut it| it.next().is_some())
        .unwrap_or(false)
}

/// 连接 Profile 时确保本地仓库就绪并与远程完成一次双向同步。
///
/// - `remote.url` 为空：纯本地模式，直接跳过；
/// - 目录不存在或为空：clone 远程仓库（场景 1）；
/// - 已有 `.git`：校正 origin、fetch、rebase 整合、push 本地提交（场景 2）；
/// - 非空目录但非 git 仓库：返回 `GIT_DIRECTORY_NOT_EMPTY`，由调用方决定是否继续。
///
/// 成功/失败都会在有 `app_handle` 时推送 `git-sync-result` 事件。
pub fn ensure_and_sync(
    repo_path: &Path,
    remote: &GitRemoteConfig,
    app_handle: Option<&AppHandle>,
) -> Result<(), GitSyncError> {
    if remote.url.is_empty() {
        return Ok(());
    }

    let _guard = lock_git_ops();

    let result = sync_locked(repo_path, remote);
    match &result {
        Ok(()) => emit_success(app_handle, "sync", "远程仓库同步成功"),
        Err(e) => {
            tracing::warn!("Git 远程同步失败（{}）：{}", e.code, e.message);
            emit_failure(app_handle, "sync", e);
        }
    }
    result
}

/// 持锁状态下的实际同步流程
fn sync_locked(repo_path: &Path, remote: &GitRemoteConfig) -> Result<(), GitSyncError> {
    let git_dir = repo_path.join(".git");

    if !repo_path.exists() || !is_non_empty_dir(repo_path) {
        // 场景 1：首次新建，直接克隆
        git_sync::clone_repo(&remote.url, repo_path, &remote.credentials)
            .map_err(|e| GitSyncError::from_git(&e))?;
        return Ok(());
    }

    if !git_dir.exists() {
        // 非空目录但不是 git 仓库：不猜测用户意图，明确报错
        return Err(GitSyncError::new(
            "GIT_DIRECTORY_NOT_EMPTY",
            format!("目录已存在且非空，但不是 git 仓库：{}", repo_path.display()),
        ));
    }

    // 场景 2：已有仓库，校正远程地址
    let current = git_sync::get_remote_url(repo_path)
        .map_err(|e| GitSyncError::from_git(&e))?;
    if current.as_deref() != Some(remote.url.as_str()) {
        git_sync::set_remote_url(repo_path, &remote.url)
            .map_err(|e| GitSyncError::from_git(&e))?;
    }

    // fetch 远程更新
    if let Err(e) = git_sync::fetch(repo_path, &remote.credentials) {
        return Err(GitSyncError::from_git(&e));
    }

    integrate_and_push(repo_path, remote)
}

/// fetch 之后整合远程更新并推送：本地有提交走 rebase；空仓库尝试快进拉取。
fn integrate_and_push(repo_path: &Path, remote: &GitRemoteConfig) -> Result<(), GitSyncError> {
    if git_sync::has_head(repo_path) {
        // 本地已有提交：rebase 整合（冲突立即 abort 回滚，autostash 随之恢复）
        if let Err(e) = git_sync::pull_rebase(repo_path, &remote.credentials) {
            let err = GitSyncError::from_git(&e);
            if err.code == "GIT_CONFLICT" {
                let _ = git_sync::rebase_abort(repo_path);
            }
            return Err(err);
        }
        push_local(repo_path, remote)
    } else {
        // 本地尚无提交（克隆空仓库后或新 init）：远程有内容则快进到远程 HEAD
        match git_sync::pull_rebase(repo_path, &remote.credentials) {
            Ok(()) => Ok(()),
            Err(e) => {
                let err = GitSyncError::from_git(&e);
                if err.code == "GIT_REMOTE_EMPTY" {
                    // 双方都为空仓库，等首次本地提交后再推
                    Ok(())
                } else {
                    Err(err)
                }
            }
        }
    }
}

/// 推送本地提交（被拒绝不自动 force）
fn push_local(repo_path: &Path, remote: &GitRemoteConfig) -> Result<(), GitSyncError> {
    git_sync::push(repo_path, &remote.credentials).map_err(|e| GitSyncError::from_git(&e))
}

/// 本地提交后的后台同步：fetch + rebase 整合 + push。
///
/// 由 GitStore 在去抖线程中调用；失败仅告警并返回错误，不改动本地状态，
/// 下一次提交会自然重试。
pub fn sync_after_commit(
    repo_path: &Path,
    remote: &GitRemoteConfig,
    app_handle: Option<&AppHandle>,
) {
    if remote.url.is_empty() {
        return;
    }

    let _guard = lock_git_ops();

    // 先 fetch 再 rebase 整合，降低 push 被拒概率
    let result = (|| -> Result<(), GitSyncError> {
        git_sync::fetch(repo_path, &remote.credentials).map_err(|e| GitSyncError::from_git(&e))?;
        if git_sync::has_head(repo_path) {
            if let Err(e) = git_sync::pull_rebase(repo_path, &remote.credentials) {
                let err = GitSyncError::from_git(&e);
                if err.code == "GIT_CONFLICT" {
                    let _ = git_sync::rebase_abort(repo_path);
                }
                return Err(err);
            }
            push_local(repo_path, remote)?;
        }
        Ok(())
    })();

    match result {
        Ok(()) => emit_success(app_handle, "push", "本地更改已推送到远程"),
        Err(e) => {
            tracing::warn!("Git 提交后推送失败（{}）：{}", e.code, e.message);
            emit_failure(app_handle, "push", &e);
        }
    }
}

/// 测试远程地址是否可访问（向导「测试连接」使用，不读写本地仓库）
pub fn test_remote(url: &str, credentials: &GitCredentials) -> Result<(), GitSyncError> {
    let _guard = lock_git_ops();
    git_sync::ls_remote(url, credentials).map_err(|e| GitSyncError::from_git(&e))
}
