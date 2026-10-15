//! Git 仓库底层操作（基于系统 `git` CLI）
//!
//! git 后端依赖用户机器已安装 git。加载 git 类型的 Profile 前应通过
//! [`is_available`] 检测可用性，不可用时 Profile 置为不可用状态。
//!
//! 调用方对初始化/提交类操作保持「失败静默忽略」的既有语义；
//! 网络操作（clone/fetch/push）在后续阶段接入远程配置后实现。

use std::path::Path;
use std::process::Command;

use anyhow::{bail, Result};

/// 检测系统是否安装了 git（`git --version` 可正常执行）
pub fn is_available() -> bool {
    Command::new("git")
        .arg("--version")
        .env("LC_ALL", "C")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// 若目标目录尚未初始化 git 仓库，则创建一个本地仓库并配置提交身份。
///
/// `.git` 目录已存在时直接返回，符合 `git init` 的幂等语义。
pub fn init_repo(repo_path: &Path) -> Result<()> {
    if repo_path.join(".git").exists() {
        return Ok(());
    }
    run_git(repo_path, &["init"])?;
    run_git(repo_path, &["config", "user.name", "ENote"])?;
    run_git(repo_path, &["config", "user.email", "enote@local"])?;
    Ok(())
}

/// `git add -A` + `git commit`，将工作区全部变更提交到本地仓库。
///
/// 无变更时 `git commit` 返回非零退出码，由调用方决定静默忽略。
pub fn commit_all(repo_path: &Path, message: &str) -> Result<()> {
    run_git(repo_path, &["add", "-A"])?;
    run_git(repo_path, &["commit", "-m", message])?;
    Ok(())
}

/// 执行 git 子命令，非零退出码时返回带 stderr 的错误。
///
/// 固定 `LC_ALL=C`，保证错误信息为英文，便于日志排查与匹配。
fn run_git(dir: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("LC_ALL", "C")
        .output()?;
    if !output.status.success() {
        bail!(
            "git {} 执行失败: {}",
            args[0],
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}
