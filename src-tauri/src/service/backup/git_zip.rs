//! Git 仓库备份：将整个仓库目录打包为 zip（含 .git 历史、笔记、待办与附件）
//!
//! 与数据库三格式（SQL/Excel/CSV）不同，Git 后端的数据本身就是文件，
//! 备份即打包目录，恢复即解压目录，不依赖 git CLI。

use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

/// 将 Git 仓库目录打包为 zip 备份文件
///
/// 包含 .git 目录（若有），恢复后即可得到完整仓库（含提交历史与附件）。
/// 空目录不会写入 zip；GitStore 打开仓库时会自动重建目录结构，无影响。
pub fn export_git_repo_zip(repo_path: &Path, zip_path: &str) -> Result<()> {
    if !repo_path.exists() {
        anyhow::bail!("仓库路径不存在: {:?}", repo_path);
    }
    if !repo_path.is_dir() {
        anyhow::bail!("仓库路径不是目录: {:?}", repo_path);
    }

    let file = std::fs::File::create(zip_path)
        .with_context(|| format!("创建备份文件失败: {:?}", zip_path))?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let mut files = Vec::new();
    collect_files(repo_path, repo_path, &mut files)?;

    for (full, rel) in files {
        zip.start_file(rel.to_string_lossy().to_string(), opts)
            .with_context(|| format!("写入 zip 条目失败: {:?}", rel))?;
        let data = std::fs::read(&full).with_context(|| format!("读取文件失败: {:?}", full))?;
        zip.write_all(&data)?;
    }

    zip.finish()?;
    Ok(())
}

/// 递归收集目录下所有文件（含隐藏文件/目录如 .git），rel 为相对 root 的路径
///
/// 使用 DirEntry::file_type（不跟随符号链接），避免符号链接导致的路径逃逸。
fn collect_files(root: &Path, dir: &Path, out: &mut Vec<(PathBuf, PathBuf)>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();
        let rel = path.strip_prefix(root)?.to_path_buf();
        if file_type.is_dir() {
            collect_files(root, &path, out)?;
        } else if file_type.is_file() {
            out.push((path, rel));
        }
        // 符号链接等其他类型跳过
    }
    Ok(())
}

/// 将 Git 仓库 zip 备份解压恢复到目标目录
///
/// 目标目录必须为空或不存在，避免覆盖现有数据。
pub fn restore_git_repo_zip(zip_path: &str, target_dir: &Path) -> Result<()> {
    if !std::path::Path::new(zip_path).exists() {
        anyhow::bail!("备份文件不存在: {:?}", zip_path);
    }
    if target_dir.exists() {
        let not_empty = std::fs::read_dir(target_dir)?.next().is_some();
        if not_empty {
            anyhow::bail!("目标目录非空，拒绝恢复: {:?}", target_dir);
        }
    } else {
        std::fs::create_dir_all(target_dir)
            .with_context(|| format!("创建目标目录失败: {:?}", target_dir))?;
    }

    let file = std::fs::File::open(zip_path)
        .with_context(|| format!("打开备份文件失败: {:?}", zip_path))?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        // enclosed_name 已做 zip-slip 防护，None 表示路径逃逸条目，跳过
        let Some(rel) = entry.enclosed_name() else {
            continue;
        };
        let out_path = target_dir.join(rel);
        if entry.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out = std::fs::File::create(&out_path)?;
            std::io::copy(&mut entry, &mut out)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn temp_dir(name: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "enote-git-zip-test-{}-{}-{n}",
            std::process::id(),
            name
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn export_and_restore_roundtrip() {
        let repo = temp_dir("repo");
        let zip_dir = temp_dir("zip");
        let target = temp_dir("target");

        // 构造仓库结构：含隐藏目录 .git、多级子目录、中文文件名
        std::fs::create_dir_all(repo.join(".git/objects")).unwrap();
        std::fs::create_dir_all(repo.join("notes/工作台")).unwrap();
        std::fs::write(repo.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        std::fs::write(repo.join(".git/objects/abc"), "object-data").unwrap();
        std::fs::write(repo.join("notes/工作台/笔记.md"), "---\nid: 1\n---\n正文").unwrap();
        std::fs::create_dir_all(repo.join("todos")).unwrap();
        std::fs::write(repo.join("todos/默认.md"), "---\nid: 2\n---\n任务").unwrap();
        std::fs::write(repo.join("tags.yml"), "[]").unwrap();

        let zip_path = zip_dir.join("enote_backup_test.git.zip");
        export_git_repo_zip(&repo, &zip_path.to_string_lossy()).unwrap();
        assert!(zip_path.exists());
        assert!(zip_path.metadata().unwrap().len() > 0);

        // 恢复到新目录
        restore_git_repo_zip(&zip_path.to_string_lossy(), &target).unwrap();
        assert_eq!(
            std::fs::read_to_string(target.join(".git/HEAD")).unwrap(),
            "ref: refs/heads/main\n"
        );
        assert_eq!(
            std::fs::read_to_string(target.join("notes/工作台/笔记.md")).unwrap(),
            "---\nid: 1\n---\n正文"
        );
        assert_eq!(
            std::fs::read_to_string(target.join("todos/默认.md")).unwrap(),
            "---\nid: 2\n---\n任务"
        );
        assert_eq!(
            std::fs::read_to_string(target.join("tags.yml")).unwrap(),
            "[]"
        );
    }

    #[test]
    fn restore_rejects_non_empty_target() {
        let repo = temp_dir("repo2");
        let zip_dir = temp_dir("zip2");
        let target = temp_dir("target2");

        std::fs::write(repo.join("a.md"), "content").unwrap();
        let zip_path = zip_dir.join("backup.zip");
        export_git_repo_zip(&repo, &zip_path.to_string_lossy()).unwrap();

        // 目标目录非空 → 拒绝
        std::fs::write(target.join("existing.txt"), "x").unwrap();
        let err = restore_git_repo_zip(&zip_path.to_string_lossy(), &target);
        assert!(err.is_err());

        // 目标目录为空 → 允许
        std::fs::remove_file(target.join("existing.txt")).unwrap();
        restore_git_repo_zip(&zip_path.to_string_lossy(), &target).unwrap();
        assert_eq!(
            std::fs::read_to_string(target.join("a.md")).unwrap(),
            "content"
        );
    }

    #[test]
    fn export_rejects_missing_repo() {
        let zip_dir = temp_dir("zip3");
        let zip_path = zip_dir.join("backup.zip");
        let result = export_git_repo_zip(
            Path::new("/nonexistent/repo-path-xyz"),
            &zip_path.to_string_lossy(),
        );
        assert!(result.is_err());
    }
}
