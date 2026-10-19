//! Git 仓库底层操作（基于系统 `git` CLI）
//!
//! git 后端依赖用户机器已安装 git。加载 git 类型的 Profile 前应通过
//! [`is_available`] 检测可用性，不可用时 Profile 置为不可用状态。
//!
//! 调用方对初始化/提交类操作保持「失败静默忽略」的既有语义；
//! 网络操作（clone/fetch/pull/push）的失败由上层编排区分错误码后告警，
//! 不阻塞本地使用（离线优先）。

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD};

/// 网络操作默认超时（秒）：常规命令 60s
const NETWORK_TIMEOUT_SECS: u64 = 60;
/// clone 超时（秒）：首次克隆可能较大，给 300s
const CLONE_TIMEOUT_SECS: u64 = 300;
/// 轮询子进程状态的间隔
const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Git 远程认证方式（密码/PAT 仅运行时传入，不落盘、不进进程参数）
#[derive(Debug, Clone, Default)]
pub enum GitCredentials {
    /// 不认证（公开仓库或纯本地）
    #[default]
    None,
    /// HTTPS 用户名 + 密码/访问令牌（PAT）
    Https { username: String, password: String },
    /// SSH（走系统 ssh，读取 ~/.ssh/config 与已有密钥）
    Ssh,
}

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

/// 执行本地 git 子命令，非零退出码时返回带 stderr 的错误。
///
/// 固定 `LC_ALL=C`，保证错误信息为英文，便于日志排查与匹配。
fn run_git(dir: &Path, args: &[&str]) -> Result<()> {
    run_git_with(dir, args, &[], None)?;
    Ok(())
}

/// 执行 git 子命令的统一入口，支持环境变量注入与超时控制，成功时返回 stdout。
///
/// - 固定 `LC_ALL=C`，保证错误信息为英文，便于 [`classify_git_error`] 匹配；
/// - `envs` 中的键值对追加到子进程环境（用于凭据注入与防交互挂起）；
/// - `timeout_secs` 到期后强杀子进程，避免认证提示/网络异常导致永久挂起。
fn run_git_with(
    dir: &Path,
    args: &[&str],
    envs: &[(&str, &str)],
    timeout_secs: Option<u64>,
) -> Result<String> {
    let mut child = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("LC_ALL", "C")
        .envs(envs.iter().copied())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // 轮询等待，超时则 kill（output() 无法处理挂起的认证提示）
    let deadline = timeout_secs.map(|s| Instant::now() + Duration::from_secs(s));
    loop {
        match child.try_wait()? {
            Some(status) => {
                let output = child.wait_with_output()?;
                if !status.success() {
                    bail!(
                        "git {} 执行失败: {}",
                        args.iter().find(|a| !a.starts_with('-')).copied().unwrap_or(""),
                        String::from_utf8_lossy(&output.stderr).trim()
                    );
                }
                return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
            None => {
                if let Some(deadline) = deadline
                    && Instant::now() > deadline
                {
                    let _ = child.kill();
                    let _ = child.wait();
                    bail!("git 命令超时（{} 秒）已终止", timeout_secs.unwrap_or(0));
                }
                std::thread::sleep(POLL_INTERVAL);
            }
        }
    }
}

// ============================================================================
// 远程操作（clone / fetch / pull / push / ls-remote）
// ============================================================================

/// 构造网络操作的防挂起环境变量与凭据注入。
///
/// - `GIT_TERMINAL_PROMPT=0`：禁止任何终端交互提示；
/// - HTTPS：通过 `GIT_CONFIG_COUNT/KEY_n/VALUE_n` 注入空 credential.helper
///   （重置系统 helper 链，防止系统凭据管理器弹窗）与 `http.extraHeader`
///   的 Basic 认证头。凭据只走环境变量，不进进程参数、不落盘；
/// - SSH：`GIT_SSH_COMMAND` 使用 BatchMode（禁止交互）+ accept-new（自动接受
///   首次主机指纹）+ 连接超时。
fn network_envs(creds: &GitCredentials) -> Vec<(&'static str, String)> {
    let mut envs: Vec<(&'static str, String)> = vec![
        ("GIT_TERMINAL_PROMPT", "0".to_string()),
        ("GIT_ASKPASS", "echo".to_string()),
        ("GCM_INTERACTIVE", "never".to_string()),
    ];
    match creds {
        GitCredentials::None => {}
        GitCredentials::Https { username, password } => {
            // GIT_CONFIG_* 按下标成对注入，索引 0/1 两个配置项
            envs.push(("GIT_CONFIG_COUNT", "2".to_string()));
            envs.push(("GIT_CONFIG_KEY_0", "credential.helper".to_string()));
            envs.push(("GIT_CONFIG_VALUE_0", String::new()));
            envs.push(("GIT_CONFIG_KEY_1", "http.extraHeader".to_string()));
            let token = STANDARD.encode(format!("{username}:{password}"));
            envs.push(("GIT_CONFIG_VALUE_1", format!("Authorization: Basic {token}")));
        }
        GitCredentials::Ssh => {
            envs.push((
                "GIT_SSH_COMMAND",
                "ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new -o ConnectTimeout=10"
                    .to_string(),
            ));
        }
    }
    envs
}

/// 网络命令统一的低速超时参数（1000 字节/秒以下持续 30 秒判定失败）
const SLOW_SPEED_ARGS: [&str; 4] = [
    "-c",
    "http.lowSpeedLimit=1000",
    "-c",
    "http.lowSpeedTime=30",
];

/// 克隆远程仓库到目标目录（父目录会自动创建）
pub fn clone_repo(
    url: &str,
    target_dir: &Path,
    creds: &GitCredentials,
) -> Result<()> {
    if let Some(parent) = target_dir.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let envs = network_envs(creds);
    let env_refs: Vec<(&str, &str)> = envs
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect();
    // target_dir 的父目录作为工作目录，目标以路径参数传入
    let work_dir = target_dir.parent().unwrap_or(Path::new("."));
    let target = target_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".to_string());
    let mut args: Vec<&str> = SLOW_SPEED_ARGS.to_vec();
    args.extend(["clone", url, &target]);
    run_git_with(work_dir, &args, &env_refs, Some(CLONE_TIMEOUT_SECS))?;
    Ok(())
}

/// 读取 origin 远程地址（未配置或读取失败时返回 None）
pub fn get_remote_url(repo_path: &Path) -> Result<Option<String>> {
    match run_git_with(repo_path, &["remote", "get-url", "origin"], &[], None) {
        Ok(url) => Ok(Some(url)),
        Err(_) => Ok(None),
    }
}

/// 设置 origin 远程地址（不存在则新增，存在则更新）
pub fn set_remote_url(repo_path: &Path, url: &str) -> Result<()> {
    if get_remote_url(repo_path)?.is_some() {
        run_git(repo_path, &["remote", "set-url", "origin", url])?;
    } else {
        run_git(repo_path, &["remote", "add", "origin", url])?;
    }
    Ok(())
}

/// 判断本地仓库是否已有提交（HEAD 是否存在，用于区分空仓库）
pub fn has_head(repo_path: &Path) -> bool {
    run_git(repo_path, &["rev-parse", "--verify", "HEAD"]).is_ok()
}

/// 拉取远程更新（fetch）
pub fn fetch(repo_path: &Path, creds: &GitCredentials) -> Result<()> {
    let envs = network_envs(creds);
    let env_refs: Vec<(&str, &str)> = envs
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect();
    let mut args: Vec<&str> = SLOW_SPEED_ARGS.to_vec();
    args.extend(["fetch", "origin"]);
    run_git_with(repo_path, &args, &env_refs, Some(NETWORK_TIMEOUT_SECS))?;
    Ok(())
}

/// 以 rebase 方式整合远程更新（含 autostash，冲突时由调用方 abort）
pub fn pull_rebase(repo_path: &Path, creds: &GitCredentials) -> Result<()> {
    let envs = network_envs(creds);
    let env_refs: Vec<(&str, &str)> = envs
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect();
    let mut args: Vec<&str> = SLOW_SPEED_ARGS.to_vec();
    args.extend(["pull", "--rebase", "--autostash", "origin", "HEAD"]);
    run_git_with(repo_path, &args, &env_refs, Some(NETWORK_TIMEOUT_SECS))?;
    Ok(())
}

/// 中止进行中的 rebase（冲突回滚，autostash 会随 abort 自动恢复）
pub fn rebase_abort(repo_path: &Path) -> Result<()> {
    run_git(repo_path, &["rebase", "--abort"])
}

/// 推送当前分支到 origin 并建立上游跟踪（`-u origin HEAD` 推同名分支）
pub fn push(repo_path: &Path, creds: &GitCredentials) -> Result<()> {
    let envs = network_envs(creds);
    let env_refs: Vec<(&str, &str)> = envs
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect();
    let mut args: Vec<&str> = SLOW_SPEED_ARGS.to_vec();
    args.extend(["push", "-u", "origin", "HEAD"]);
    run_git_with(repo_path, &args, &env_refs, Some(NETWORK_TIMEOUT_SECS))?;
    Ok(())
}

/// 测试远程仓库是否可访问（ls-remote，供向导「测试连接」使用）
pub fn ls_remote(url: &str, creds: &GitCredentials) -> Result<()> {
    let envs = network_envs(creds);
    let env_refs: Vec<(&str, &str)> = envs
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect();
    let mut args: Vec<&str> = SLOW_SPEED_ARGS.to_vec();
    args.extend(["ls-remote", "--heads", url]);
    run_git_with(Path::new("."), &args, &env_refs, Some(NETWORK_TIMEOUT_SECS))?;
    Ok(())
}

/// 将 git 命令的 stderr 文本归类为前端可翻译的错误码。
///
/// 依赖 `LC_ALL=C` 保证 git 输出英文。匹配顺序敏感：先匹配冲突/拒绝等
/// 具体场景，再匹配认证、仓库不存在、网络问题，最后兜底 GIT_SYNC_FAILED。
pub fn classify_git_error(stderr: &str) -> &'static str {
    let s = stderr.to_ascii_lowercase();
    if s.contains("couldn't find remote ref head") {
        // 空远程仓库 pull 时的预期情况，交由上层特殊处理，不当错误
        "GIT_REMOTE_EMPTY"
    } else if s.contains("could not apply")
        || s.contains("conflict (content)")
        || s.contains("merge conflict")
        || s.contains("automatic merge failed")
        || (s.contains("rebase") && s.contains("conflict"))
    {
        "GIT_CONFLICT"
    } else if s.contains("rejected") || s.contains("non-fast-forward") || s.contains("fetch first") {
        "GIT_PUSH_REJECTED"
    } else if s.contains("authentication failed")
        || s.contains("could not read username")
        || s.contains("invalid username or password")
        || s.contains("permission denied (publickey")
        || s.contains("403 forbidden")
        || s.contains("401 unauthorized")
        || s.contains("terminal prompts disabled")
    {
        "GIT_AUTH_FAILED"
    } else if s.contains("not found (repository)")
        || (s.contains("repository") && s.contains("' not found"))
        || s.contains("does not exist")
        || s.contains("could not read from remote repository")
    {
        "GIT_REPO_NOT_FOUND"
    } else if s.contains("could not resolve host")
        || s.contains("connection timed out")
        || s.contains("connection refused")
        || s.contains("network is unreachable")
        || s.contains("operation timed out")
        || s.contains("early eof")
        || s.contains("rpc failed")
        || s.contains("unable to access")
    {
        "GIT_NETWORK_FAILED"
    } else {
        "GIT_SYNC_FAILED"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_conflict() {
        assert_eq!(
            classify_git_error("error: could not apply abc123... fix\nResolve all conflicts manually"),
            "GIT_CONFLICT"
        );
        assert_eq!(
            classify_git_error("Automatic merge failed; fix conflicts and then commit the result."),
            "GIT_CONFLICT"
        );
    }

    #[test]
    fn test_classify_push_rejected() {
        assert_eq!(
            classify_git_error("! [rejected] main -> main (non-fast-forward)"),
            "GIT_PUSH_REJECTED"
        );
        assert_eq!(
            classify_git_error("error: failed to push some refs... Updates were rejected"),
            "GIT_PUSH_REJECTED"
        );
    }

    #[test]
    fn test_classify_auth_failed() {
        assert_eq!(
            classify_git_error("fatal: Authentication failed for 'https://github.com/x/y.git/'"),
            "GIT_AUTH_FAILED"
        );
        assert_eq!(
            classify_git_error("remote: Permission to x/y.git denied.\nfatal: unable to access: 403 Forbidden"),
            "GIT_AUTH_FAILED"
        );
        assert_eq!(
            classify_git_error("git@github.com: Permission denied (publickey)."),
            "GIT_AUTH_FAILED"
        );
    }

    #[test]
    fn test_classify_repo_not_found() {
        assert_eq!(
            classify_git_error("fatal: repository 'https://github.com/x/nope.git/' not found"),
            "GIT_REPO_NOT_FOUND"
        );
    }

    #[test]
    fn test_classify_network_failed() {
        assert_eq!(
            classify_git_error("fatal: unable to access 'https://x/': Could not resolve host: x"),
            "GIT_NETWORK_FAILED"
        );
        assert_eq!(
            classify_git_error("fatal: unable to access: Connection timed out after 30000 ms"),
            "GIT_NETWORK_FAILED"
        );
    }

    #[test]
    fn test_classify_fallback() {
        assert_eq!(classify_git_error("some weird unknown error"), "GIT_SYNC_FAILED");
    }

    #[test]
    fn test_remote_empty() {
        assert_eq!(
            classify_git_error("fatal: couldn't find remote ref HEAD"),
            "GIT_REMOTE_EMPTY"
        );
    }

    #[test]
    fn test_network_envs_none_and_ssh() {
        // None：只有防交互三件套
        assert_eq!(network_envs(&GitCredentials::None).len(), 3);
        // SSH：三件套 + GIT_SSH_COMMAND
        let ssh_envs = network_envs(&GitCredentials::Ssh);
        assert!(ssh_envs.iter().any(|(k, _)| *k == "GIT_SSH_COMMAND"));
        assert!(ssh_envs
            .iter()
            .any(|(_, v)| v.contains("BatchMode=yes")));
    }

    #[test]
    fn test_network_envs_https() {
        let envs = network_envs(&GitCredentials::Https {
            username: "alice".to_string(),
            password: "tok".to_string(),
        });
        let find = |key: &str| envs.iter().find(|(k, _)| *k == key).map(|(_, v)| v.as_str());
        assert_eq!(find("GIT_CONFIG_COUNT"), Some("2"));
        assert_eq!(find("GIT_CONFIG_KEY_0"), Some("credential.helper"));
        assert_eq!(find("GIT_CONFIG_VALUE_0"), Some(""));
        assert_eq!(find("GIT_CONFIG_KEY_1"), Some("http.extraHeader"));
        // Basic base64("alice:tok") = "YWxpY2U6dG9r"
        assert_eq!(
            find("GIT_CONFIG_VALUE_1"),
            Some("Authorization: Basic YWxpY2U6dG9r")
        );
    }
}
