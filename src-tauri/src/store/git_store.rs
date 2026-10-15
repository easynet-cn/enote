//! Git 仓库后端实现
//!
//! 基于 Git 仓库的 [`NoteStore`] / [`TodoStore`] 实现，
//! 使用 YAML frontmatter + Markdown body 格式存储笔记和待办。
//! 每次写操作后自动 `git add` + `git commit`。
//!
//! # 文件格式
//! ```text
//! ---
//! id: 1700000000000
//! uuid: "f4a3b2c1-..."
//! title: "笔记标题"
//! ---
//! 笔记正文...
//! ```
//!
//! # 目录结构
//! ```text
//! 仓库根/
//! ├── notes/           # 笔记按笔记本分目录
//! │   ├── {notebook}/
//! │   │   ├── {uuid}.md
//! │   │   └── .notebook.yml
//! │   ├── _unlisted/    # 无笔记本的笔记
//! │   └── _trash/       # 软删除的笔记
//! ├── todos/           # 待办按清单分目录
//! │   ├── {list}/
//! │   │   ├── {uuid}.md
//! │   │   └── .todo-list.yml
//! │   ├── _unlisted/
//! │   └── _trash/
//! ├── templates/       # 模板
//! │   └── {uuid}.md
//! ├── attachments/     # 附件（sha256 去重）
//! │   └── {sha256}.ext
//! └── tags.yml         # 标签元数据索引
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock as StdRwLock;

use chrono::{Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::error::AppError;
use crate::model::*;
use crate::service::crypto;

use super::{NoteStore, TodoStore};

// ============================================================================
// Frontmatter 辅助结构
// ============================================================================

/// 笔记 frontmatter（YAML 部分）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct NoteFrontmatter {
    id: i64,
    #[serde(default)]
    uuid: Option<String>,
    #[serde(default)]
    title: String,
    #[serde(default)]
    content_type: i32,
    #[serde(default)]
    notebook_id: i64,
    #[serde(default)]
    notebook: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    is_pinned: i32,
    #[serde(default)]
    is_starred: i32,
    #[serde(default)]
    mcp_access: i32,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    updated_at: Option<String>,
    #[serde(default)]
    deleted_at: Option<String>,
    #[serde(default)]
    links: Vec<LinkEntry>,
    #[serde(default)]
    attachments: Vec<AttachmentEntry>,
}

/// 链接条目（存储在笔记 frontmatter 中）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct LinkEntry {
    #[serde(default)]
    id: i64,
    #[serde(default)]
    note_id: i64,
    #[serde(default)]
    note_title: String,
    #[serde(default)]
    create_time: Option<String>,
}

/// 附件条目（存储在笔记 frontmatter 中）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct AttachmentEntry {
    #[serde(default)]
    id: i64,
    #[serde(default)]
    note_id: i64,
    #[serde(default)]
    file_name: String,
    #[serde(default)]
    file_path: String,
    #[serde(default)]
    file_size: i64,
    #[serde(default)]
    mime_type: String,
    #[serde(default)]
    file_hash: String,
    #[serde(default)]
    ref_count: i32,
    #[serde(default)]
    create_time: Option<String>,
}

/// 待办 frontmatter（YAML 部分）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TodoFrontmatter {
    id: i64,
    #[serde(default)]
    uuid: Option<String>,
    #[serde(default)]
    title: String,
    // description 存在 body 中
    #[serde(default)]
    is_completed: i32,
    #[serde(default)]
    priority: i32,
    #[serde(default)]
    due_date: Option<String>,
    #[serde(default)]
    completed_at: Option<String>,
    #[serde(default)]
    list_id: Option<i64>,
    #[serde(default)]
    list_name: String,
    #[serde(default)]
    sort_order: i32,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    updated_at: Option<String>,
    #[serde(default)]
    deleted_at: Option<String>,
    #[serde(default)]
    start_date: Option<String>,
    #[serde(default)]
    remind_at: Option<String>,
    #[serde(default)]
    is_reminded: i32,
    #[serde(default)]
    parent_id: i64,
    #[serde(default)]
    note_id: Option<i64>,
    #[serde(default)]
    recurrence_type: i32,
    #[serde(default)]
    recurrence_interval: i32,
    #[serde(default)]
    recurrence_end_date: Option<String>,
    #[serde(default)]
    mcp_access: i32,
    #[serde(default)]
    tags: Vec<i64>,
}

/// 模板 frontmatter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TemplateFrontmatter {
    id: i64,
    #[serde(default)]
    uuid: Option<String>,
    #[serde(default)]
    name: String,
    #[serde(default)]
    content_type: i32,
    #[serde(default)]
    sort_order: i32,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    updated_at: Option<String>,
}

/// 笔记本元数据（.notebook.yml）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct NotebookMeta {
    id: i64,
    #[serde(default)]
    parent_id: i64,
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    icon: String,
    #[serde(default)]
    cls: String,
    #[serde(default)]
    sort_order: i32,
    #[serde(default)]
    mcp_access: i32,
    #[serde(default)]
    create_time: Option<String>,
    #[serde(default)]
    update_time: Option<String>,
    /// 笔记本目录名（用于定位目录）
    #[serde(default)]
    dir_name: String,
}

/// 待办清单元数据（.todo-list.yml）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TodoListMeta {
    id: i64,
    name: String,
    #[serde(default)]
    icon: String,
    #[serde(default)]
    color: String,
    #[serde(default)]
    sort_order: i32,
    #[serde(default)]
    mcp_access: i32,
    #[serde(default)]
    create_time: Option<String>,
    #[serde(default)]
    update_time: Option<String>,
    #[serde(default)]
    dir_name: String,
}

/// tags.yml 文件结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TagsFile {
    #[serde(default)]
    tags: Vec<TagEntry>,
}

/// 标签条目
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TagEntry {
    id: i64,
    name: String,
    #[serde(default)]
    icon: String,
    #[serde(default)]
    cls: String,
    #[serde(default)]
    sort_order: i32,
    #[serde(default)]
    mcp_access: i32,
    #[serde(default)]
    create_time: Option<String>,
    #[serde(default)]
    update_time: Option<String>,
}

// ============================================================================
// GitStore
// ============================================================================

/// Git 仓库后端
pub struct GitStore {
    /// 仓库根路径
    repo_path: PathBuf,
    /// 内容加密密钥
    enc_key: Option<String>,
    /// 笔记索引：id -> 文件路径
    note_index: StdRwLock<HashMap<i64, PathBuf>>,
    /// 待办索引：id -> 文件路径
    todo_index: StdRwLock<HashMap<i64, PathBuf>>,
    /// 模板索引：id -> 文件路径
    template_index: StdRwLock<HashMap<i64, PathBuf>>,
    /// 笔记本索引：id -> 元数据
    notebook_index: StdRwLock<HashMap<i64, NotebookMeta>>,
    /// 待办清单索引：id -> 元数据
    todo_list_index: StdRwLock<HashMap<i64, TodoListMeta>>,
    /// 标签列表
    tags: StdRwLock<Vec<TagEntry>>,
    /// ID 计数器（确保唯一）
    next_id: StdRwLock<i64>,
    /// 链接 ID 计数器
    next_link_id: StdRwLock<i64>,
    /// 附件 ID 计数器
    next_attachment_id: StdRwLock<i64>,
}

impl GitStore {
    /// 创建 GitStore，初始化仓库并构建索引
    pub fn new(repo_path: PathBuf, enc_key: Option<String>) -> Self {
        let store = Self {
            repo_path: repo_path.clone(),
            enc_key,
            note_index: StdRwLock::new(HashMap::new()),
            todo_index: StdRwLock::new(HashMap::new()),
            template_index: StdRwLock::new(HashMap::new()),
            notebook_index: StdRwLock::new(HashMap::new()),
            todo_list_index: StdRwLock::new(HashMap::new()),
            tags: StdRwLock::new(Vec::new()),
            next_id: StdRwLock::new(0),
            next_link_id: StdRwLock::new(0),
            next_attachment_id: StdRwLock::new(0),
        };

        // 创建目录结构
        store.ensure_directories();

        // 初始化 git 仓库
        store.git_init();

        // 构建索引
        store.rebuild_indexes();

        store
    }

    // ========= 目录与路径 =========

    /// 确保目录结构存在
    fn ensure_directories(&self) {
        let dirs = [
            self.repo_path.join("notes"),
            self.repo_path.join("notes").join("_unlisted"),
            self.repo_path.join("notes").join("_trash"),
            self.repo_path.join("todos"),
            self.repo_path.join("todos").join("_unlisted"),
            self.repo_path.join("todos").join("_trash"),
            self.repo_path.join("templates"),
            self.repo_path.join("attachments"),
        ];
        for dir in &dirs {
            let _ = std::fs::create_dir_all(dir);
        }
    }

    /// notes 目录
    fn notes_dir(&self) -> PathBuf {
        self.repo_path.join("notes")
    }

    /// todos 目录
    fn todos_dir(&self) -> PathBuf {
        self.repo_path.join("todos")
    }

    /// templates 目录
    fn templates_dir(&self) -> PathBuf {
        self.repo_path.join("templates")
    }

    /// attachments 目录
    fn attachments_dir(&self) -> PathBuf {
        self.repo_path.join("attachments")
    }

    /// tags.yml 路径
    fn tags_file_path(&self) -> PathBuf {
        self.repo_path.join("tags.yml")
    }

    /// 笔记本目录名（用于文件系统安全）
    fn safe_dir_name(name: &str) -> String {
        if name.is_empty() {
            return "_unlisted".to_string();
        }
        // 替换文件系统不安全字符
        name.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                _ => c,
            })
            .collect()
    }

    /// 笔记文件路径
    fn note_file_path(&self, uuid: &str, notebook_name: &str, deleted: bool) -> PathBuf {
        if deleted {
            self.notes_dir().join("_trash").join(format!("{}.md", uuid))
        } else {
            let dir_name = Self::safe_dir_name(notebook_name);
            self.notes_dir()
                .join(&dir_name)
                .join(format!("{}.md", uuid))
        }
    }

    /// 待办文件路径
    fn todo_file_path(&self, uuid: &str, list_name: &str, deleted: bool) -> PathBuf {
        if deleted {
            self.todos_dir().join("_trash").join(format!("{}.md", uuid))
        } else {
            let dir_name = if list_name.is_empty() {
                "_unlisted".to_string()
            } else {
                Self::safe_dir_name(list_name)
            };
            self.todos_dir()
                .join(&dir_name)
                .join(format!("{}.md", uuid))
        }
    }

    /// 模板文件路径
    fn template_file_path(&self, uuid: &str) -> PathBuf {
        self.templates_dir().join(format!("{}.md", uuid))
    }

    // ========= Git 操作 =========

    /// 初始化 git 仓库（失败静默忽略）
    fn git_init(&self) {
        let _ = super::git_sync::init_repo(&self.repo_path);
    }

    /// git add + commit（失败静默忽略）
    fn git_commit(&self, message: &str) {
        let _ = super::git_sync::commit_all(&self.repo_path, message);
    }

    // ========= ID 生成 =========

    /// 生成唯一 i64 ID
    fn gen_id(&self) -> i64 {
        let mut counter = self.next_id.write().unwrap();
        let now = Utc::now().timestamp_micros();
        let id = if *counter >= now { *counter + 1 } else { now };
        *counter = id;
        id
    }

    /// 生成 UUID 字符串
    fn gen_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    // ========= 日期时间转换 =========

    /// NaiveDateTime → "yyyy-MM-dd HH:mm:ss"
    fn dt_to_string(dt: &NaiveDateTime) -> String {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// "yyyy-MM-dd HH:mm:ss" → NaiveDateTime
    fn string_to_dt(s: &str) -> Option<NaiveDateTime> {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
    }

    /// Option<NaiveDateTime> → Option<String>
    fn opt_dt_to_string(dt: &Option<NaiveDateTime>) -> Option<String> {
        dt.map(|d| Self::dt_to_string(&d))
    }

    /// Option<String> → Option<NaiveDateTime>
    fn string_to_opt_dt(s: &Option<String>) -> Option<NaiveDateTime> {
        s.as_ref().and_then(|s| Self::string_to_dt(s))
    }

    // ========= 加密 =========

    /// 加密内容（如果密钥存在且内容未加密）
    fn encrypt_content(&self, content: &str) -> String {
        if let Some(ref key) = self.enc_key {
            if !crypto::is_encrypted(content) {
                if let Ok(encrypted) = crypto::encrypt(content, key) {
                    return encrypted;
                }
            }
        }
        content.to_string()
    }

    /// 解密内容（如果已加密），用于搜索匹配
    fn decrypt_content(&self, content: &str) -> String {
        if let Some(ref key) = self.enc_key {
            if crypto::is_encrypted(content) {
                if let Ok(decrypted) = crypto::decrypt(content, key) {
                    return decrypted;
                }
            }
        }
        content.to_string()
    }

    // ========= Frontmatter 解析 =========

    /// 将内容拆分为 (YAML 部分, body)
    ///
    /// 基于原始字符串切片定位分隔符，body 原样保留（包括末尾换行），确保写读往返无损。
    fn split_frontmatter(content: &str) -> (Option<String>, String) {
        let rest = content.trim_start();

        // 首行必须是 ---
        let Some(first_line_end) = rest.find('\n') else {
            return (None, rest.to_string());
        };
        if rest[..first_line_end].trim_end_matches('\r') != "---" {
            return (None, rest.to_string());
        }

        // 逐行查找结束分隔符 ---
        let yaml_start = first_line_end + 1;
        let mut line_start = yaml_start;
        while line_start < rest.len() {
            let line_end = rest[line_start..]
                .find('\n')
                .map_or(rest.len(), |p| line_start + p);
            if rest[line_start..line_end].trim_end_matches('\r') == "---" {
                let yaml = rest[yaml_start..line_start].to_string();
                let body = rest[(line_end + 1).min(rest.len())..].to_string();
                return (Some(yaml), body);
            }
            line_start = line_end + 1;
        }

        (None, rest.to_string())
    }

    /// 将 YAML 和 body 合并为文件内容
    fn join_frontmatter(yaml: &str, body: &str) -> String {
        // 去掉 yaml_serde 可能添加的前导 ---
        let yaml = yaml
            .trim_start_matches("---\n")
            .trim_start_matches("---\r\n")
            .trim();
        format!("---\n{}\n---\n{}", yaml, body)
    }

    // ========= 索引操作 =========

    fn index_note(&self, id: i64, path: PathBuf) {
        self.note_index.write().unwrap().insert(id, path);
    }

    fn unindex_note(&self, id: i64) {
        self.note_index.write().unwrap().remove(&id);
    }

    fn get_note_path(&self, id: i64) -> Option<PathBuf> {
        self.note_index.read().unwrap().get(&id).cloned()
    }

    fn index_todo(&self, id: i64, path: PathBuf) {
        self.todo_index.write().unwrap().insert(id, path);
    }

    fn unindex_todo(&self, id: i64) {
        self.todo_index.write().unwrap().remove(&id);
    }

    fn get_todo_path(&self, id: i64) -> Option<PathBuf> {
        self.todo_index.read().unwrap().get(&id).cloned()
    }

    fn index_template(&self, id: i64, path: PathBuf) {
        self.template_index.write().unwrap().insert(id, path);
    }

    fn unindex_template(&self, id: i64) {
        self.template_index.write().unwrap().remove(&id);
    }

    fn get_template_path(&self, id: i64) -> Option<PathBuf> {
        self.template_index.read().unwrap().get(&id).cloned()
    }

    // ========= 重建索引 =========

    /// 重建所有索引（初始化时调用）
    fn rebuild_indexes(&self) {
        // 笔记本
        let notebooks = self.load_notebooks_sync();
        {
            let mut idx = self.notebook_index.write().unwrap();
            idx.clear();
            for (id, meta) in notebooks {
                idx.insert(id, meta);
            }
        }

        // 待办清单
        let todo_lists = self.load_todo_lists_sync();
        {
            let mut idx = self.todo_list_index.write().unwrap();
            idx.clear();
            for (id, meta) in todo_lists {
                idx.insert(id, meta);
            }
        }

        // 标签
        let tags = self.load_tags_sync();
        {
            let mut t = self.tags.write().unwrap();
            *t = tags;
        }

        // 笔记索引
        let note_index = self.scan_note_index();
        {
            let mut idx = self.note_index.write().unwrap();
            idx.clear();
            idx.extend(note_index);
        }

        // 待办索引
        let todo_index = self.scan_todo_index();
        {
            let mut idx = self.todo_index.write().unwrap();
            idx.clear();
            idx.extend(todo_index);
        }

        // 模板索引
        let template_index = self.scan_template_index();
        {
            let mut idx = self.template_index.write().unwrap();
            idx.clear();
            idx.extend(template_index);
        }

        // 更新 ID 计数器
        let max_id = {
            let notes = self.note_index.read().unwrap();
            let todos = self.todo_index.read().unwrap();
            let templates = self.template_index.read().unwrap();
            let notebooks = self.notebook_index.read().unwrap();
            let todo_lists = self.todo_list_index.read().unwrap();
            let tags = self.tags.read().unwrap();

            let mut max_id = 0i64;
            for &id in notes.keys() {
                max_id = max_id.max(id);
            }
            for &id in todos.keys() {
                max_id = max_id.max(id);
            }
            for &id in templates.keys() {
                max_id = max_id.max(id);
            }
            for &id in notebooks.keys() {
                max_id = max_id.max(id);
            }
            for &id in todo_lists.keys() {
                max_id = max_id.max(id);
            }
            for tag in tags.iter() {
                max_id = max_id.max(tag.id);
            }
            max_id
        };
        {
            let mut counter = self.next_id.write().unwrap();
            *counter = max_id;
        }

        // 链接 ID 计数器
        let max_link_id = self.compute_max_link_id();
        {
            let mut counter = self.next_link_id.write().unwrap();
            *counter = max_link_id;
        }

        // 附件 ID 计数器
        let max_attachment_id = self.compute_max_attachment_id();
        {
            let mut counter = self.next_attachment_id.write().unwrap();
            *counter = max_attachment_id;
        }
    }

    /// 扫描笔记索引
    fn scan_note_index(&self) -> HashMap<i64, PathBuf> {
        let mut index = HashMap::new();
        let notes_dir = self.notes_dir();
        Self::scan_md_files(&notes_dir, &mut index);
        index
    }

    /// 扫描待办索引
    fn scan_todo_index(&self) -> HashMap<i64, PathBuf> {
        let mut index = HashMap::new();
        let todos_dir = self.todos_dir();
        Self::scan_md_files(&todos_dir, &mut index);
        index
    }

    /// 扫描模板索引
    fn scan_template_index(&self) -> HashMap<i64, PathBuf> {
        let mut index = HashMap::new();
        let templates_dir = self.templates_dir();
        // 模板直接在该目录下
        if let Ok(entries) = std::fs::read_dir(&templates_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let (yaml, _) = Self::split_frontmatter(&content);
                        if let Some(y) = yaml {
                            if let Ok(fm) = yaml_serde::from_str::<TemplateFrontmatter>(&y) {
                                index.insert(fm.id, path);
                            }
                        }
                    }
                }
            }
        }
        index
    }

    /// 递归扫描 .md 文件，解析 frontmatter 中的 id 构建索引
    fn scan_md_files(dir: &Path, index: &mut HashMap<i64, PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::scan_md_files(&path, index);
            } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let (yaml, _) = Self::split_frontmatter(&content);
                    if let Some(y) = yaml {
                        // 尝试解析为 NoteFrontmatter
                        if let Ok(fm) = yaml_serde::from_str::<NoteFrontmatter>(&y) {
                            if fm.id > 0 {
                                index.insert(fm.id, path);
                            }
                        }
                    }
                }
            }
        }
    }

    // ========= 笔记本/清单加载 =========

    /// 同步加载所有笔记本
    fn load_notebooks_sync(&self) -> HashMap<i64, NotebookMeta> {
        let mut result = HashMap::new();
        let notes_dir = self.notes_dir();

        let Ok(entries) = std::fs::read_dir(&notes_dir) else {
            return result;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let dir_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            // 跳过特殊目录
            if dir_name == "_trash" {
                continue;
            }

            let meta_path = path.join(".notebook.yml");
            if meta_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&meta_path) {
                    if let Ok(mut meta) = yaml_serde::from_str::<NotebookMeta>(&content) {
                        if meta.dir_name.is_empty() {
                            meta.dir_name = dir_name;
                        }
                        result.insert(meta.id, meta);
                    }
                }
            } else if dir_name != "_unlisted" {
                // 无元数据文件的目录，自动创建
                let id = self.gen_id();
                let now = Utc::now().naive_local();
                let meta = NotebookMeta {
                    id,
                    parent_id: 0,
                    name: dir_name.clone(),
                    description: String::new(),
                    icon: String::new(),
                    cls: String::new(),
                    sort_order: 0,
                    mcp_access: 0,
                    create_time: Some(Self::dt_to_string(&now)),
                    update_time: Some(Self::dt_to_string(&now)),
                    dir_name: dir_name.clone(),
                };
                let yaml = yaml_serde::to_string(&meta).unwrap_or_default();
                let _ = std::fs::write(&meta_path, yaml);
                result.insert(id, meta);
            }
        }

        result
    }

    /// 同步加载所有待办清单
    fn load_todo_lists_sync(&self) -> HashMap<i64, TodoListMeta> {
        let mut result = HashMap::new();
        let todos_dir = self.todos_dir();

        let Ok(entries) = std::fs::read_dir(&todos_dir) else {
            return result;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let dir_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            if dir_name == "_trash" || dir_name == "_unlisted" {
                continue;
            }

            let meta_path = path.join(".todo-list.yml");
            if meta_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&meta_path) {
                    if let Ok(mut meta) = yaml_serde::from_str::<TodoListMeta>(&content) {
                        if meta.dir_name.is_empty() {
                            meta.dir_name = dir_name;
                        }
                        result.insert(meta.id, meta);
                    }
                }
            } else {
                // 自动创建
                let id = self.gen_id();
                let now = Utc::now().naive_local();
                let meta = TodoListMeta {
                    id,
                    name: dir_name.clone(),
                    icon: String::new(),
                    color: String::new(),
                    sort_order: 0,
                    mcp_access: 0,
                    create_time: Some(Self::dt_to_string(&now)),
                    update_time: Some(Self::dt_to_string(&now)),
                    dir_name: dir_name.clone(),
                };
                let yaml = yaml_serde::to_string(&meta).unwrap_or_default();
                let _ = std::fs::write(&meta_path, yaml);
                result.insert(id, meta);
            }
        }

        result
    }

    // ========= 标签加载/保存 =========

    /// 同步加载标签
    fn load_tags_sync(&self) -> Vec<TagEntry> {
        let path = self.tags_file_path();
        if !path.exists() {
            return Vec::new();
        }
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(tags_file) = yaml_serde::from_str::<TagsFile>(&content) {
                return tags_file.tags;
            }
        }
        Vec::new()
    }

    /// 同步保存标签
    fn save_tags_sync(&self, tags: &[TagEntry]) {
        let tags_file = TagsFile {
            tags: tags.to_vec(),
        };
        let yaml = yaml_serde::to_string(&tags_file).unwrap_or_default();
        let _ = std::fs::write(self.tags_file_path(), yaml);
    }

    // ========= 辅助：计算最大 ID =========

    /// 计算最大链接 ID
    fn compute_max_link_id(&self) -> i64 {
        let note_index = self.note_index.read().unwrap();
        let mut max_id = 0i64;
        for path in note_index.values() {
            if let Ok(content) = std::fs::read_to_string(path) {
                let (yaml, _) = Self::split_frontmatter(&content);
                if let Some(y) = yaml {
                    if let Ok(fm) = yaml_serde::from_str::<NoteFrontmatter>(&y) {
                        for link in &fm.links {
                            max_id = max_id.max(link.id);
                        }
                    }
                }
            }
        }
        max_id
    }

    /// 计算最大附件 ID
    fn compute_max_attachment_id(&self) -> i64 {
        let note_index = self.note_index.read().unwrap();
        let mut max_id = 0i64;
        for path in note_index.values() {
            if let Ok(content) = std::fs::read_to_string(path) {
                let (yaml, _) = Self::split_frontmatter(&content);
                if let Some(y) = yaml {
                    if let Ok(fm) = yaml_serde::from_str::<NoteFrontmatter>(&y) {
                        for att in &fm.attachments {
                            max_id = max_id.max(att.id);
                        }
                    }
                }
            }
        }
        max_id
    }

    // ========= 笔记本/清单名称查找 =========

    /// 根据 notebook_id 获取笔记本名称
    fn get_notebook_name(&self, notebook_id: i64) -> String {
        if notebook_id == 0 {
            return String::new();
        }
        let idx = self.notebook_index.read().unwrap();
        idx.get(&notebook_id)
            .map(|m| m.name.clone())
            .unwrap_or_default()
    }

    /// 根据 list_id 获取清单名称
    fn get_todo_list_name(&self, list_id: Option<i64>) -> String {
        match list_id {
            Some(id) => {
                let idx = self.todo_list_index.read().unwrap();
                idx.get(&id).map(|m| m.name.clone()).unwrap_or_default()
            }
            None => String::new(),
        }
    }

    // ========= 文件读写（异步） =========

    /// 读取笔记文件
    async fn read_note_file(&self, path: &Path) -> Result<Note, AppError> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取笔记文件失败: {}", e)))?;

        let (yaml, body) = Self::split_frontmatter(&content);

        let fm: NoteFrontmatter = if let Some(y) = yaml {
            yaml_serde::from_str(&y).map_err(|e| {
                AppError::Internal(anyhow::anyhow!("解析笔记 frontmatter 失败: {}", e))
            })?
        } else {
            NoteFrontmatter::default()
        };

        // 构建标签列表
        let tags = self.tags_from_names(&fm.tags);

        Ok(Note {
            id: fm.id,
            uuid: fm.uuid,
            notebook_id: fm.notebook_id,
            notebook_name: fm.notebook,
            title: fm.title,
            content: body,
            content_type: fm.content_type,
            is_pinned: fm.is_pinned,
            is_starred: fm.is_starred,
            mcp_access: fm.mcp_access,
            create_time: Self::string_to_opt_dt(&fm.created_at),
            update_time: Self::string_to_opt_dt(&fm.updated_at),
            deleted_at: Self::string_to_opt_dt(&fm.deleted_at),
            tags,
        })
    }

    /// 写入笔记文件
    async fn write_note_file(&self, path: &Path, note: &Note) -> Result<(), AppError> {
        // 读取已有文件以保留 links 和 attachments
        let existing_links_attachments = if path.exists() {
            let content = tokio::fs::read_to_string(path)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("读取已有笔记文件失败: {}", e)))?;
            let (yaml, _) = Self::split_frontmatter(&content);
            if let Some(y) = yaml {
                yaml_serde::from_str::<NoteFrontmatter>(&y)
                    .map(|fm| (fm.links, fm.attachments))
                    .unwrap_or((Vec::new(), Vec::new()))
            } else {
                (Vec::new(), Vec::new())
            }
        } else {
            (Vec::new(), Vec::new())
        };

        let fm = NoteFrontmatter {
            id: note.id,
            uuid: note.uuid.clone(),
            title: note.title.clone(),
            content_type: note.content_type,
            notebook_id: note.notebook_id,
            notebook: note.notebook_name.clone(),
            tags: note.tags.iter().map(|t| t.name.clone()).collect(),
            is_pinned: note.is_pinned,
            is_starred: note.is_starred,
            mcp_access: note.mcp_access,
            created_at: Self::opt_dt_to_string(&note.create_time),
            updated_at: Self::opt_dt_to_string(&note.update_time),
            deleted_at: Self::opt_dt_to_string(&note.deleted_at),
            links: existing_links_attachments.0,
            attachments: existing_links_attachments.1,
        };

        // 加密内容
        let body = self.encrypt_content(&note.content);

        let yaml = yaml_serde::to_string(&fm).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("序列化笔记 frontmatter 失败: {}", e))
        })?;
        let file_content = Self::join_frontmatter(&yaml, &body);

        // 确保目录存在
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("创建目录失败: {}", e)))?;
        }

        tokio::fs::write(path, file_content)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入笔记文件失败: {}", e)))?;

        Ok(())
    }

    /// 读取待办文件
    async fn read_todo_file(&self, path: &Path) -> Result<Todo, AppError> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取待办文件失败: {}", e)))?;

        let (yaml, body) = Self::split_frontmatter(&content);

        let fm: TodoFrontmatter = if let Some(y) = yaml {
            yaml_serde::from_str(&y).map_err(|e| {
                AppError::Internal(anyhow::anyhow!("解析待办 frontmatter 失败: {}", e))
            })?
        } else {
            TodoFrontmatter::default()
        };

        Ok(Todo {
            id: fm.id,
            uuid: fm.uuid,
            title: fm.title,
            description: body,
            is_completed: fm.is_completed,
            priority: fm.priority,
            due_date: Self::string_to_opt_dt(&fm.due_date),
            completed_at: Self::string_to_opt_dt(&fm.completed_at),
            list_id: fm.list_id,
            sort_order: fm.sort_order,
            create_time: Self::string_to_opt_dt(&fm.created_at),
            update_time: Self::string_to_opt_dt(&fm.updated_at),
            deleted_at: Self::string_to_opt_dt(&fm.deleted_at),
            start_date: Self::string_to_opt_dt(&fm.start_date),
            remind_at: Self::string_to_opt_dt(&fm.remind_at),
            is_reminded: fm.is_reminded,
            parent_id: fm.parent_id,
            note_id: fm.note_id,
            recurrence_type: fm.recurrence_type,
            recurrence_interval: fm.recurrence_interval,
            recurrence_end_date: Self::string_to_opt_dt(&fm.recurrence_end_date),
            mcp_access: fm.mcp_access,
        })
    }

    /// 写入待办文件
    async fn write_todo_file(&self, path: &Path, todo: &Todo) -> Result<(), AppError> {
        // 读取已有文件以保留 tags
        let existing_tags = if path.exists() {
            let content = tokio::fs::read_to_string(path)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("读取已有待办文件失败: {}", e)))?;
            let (yaml, _) = Self::split_frontmatter(&content);
            if let Some(y) = yaml {
                yaml_serde::from_str::<TodoFrontmatter>(&y)
                    .map(|fm| fm.tags)
                    .unwrap_or_default()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let fm = TodoFrontmatter {
            id: todo.id,
            uuid: todo.uuid.clone(),
            title: todo.title.clone(),
            is_completed: todo.is_completed,
            priority: todo.priority,
            due_date: Self::opt_dt_to_string(&todo.due_date),
            completed_at: Self::opt_dt_to_string(&todo.completed_at),
            list_id: todo.list_id,
            list_name: self.get_todo_list_name(todo.list_id),
            sort_order: todo.sort_order,
            created_at: Self::opt_dt_to_string(&todo.create_time),
            updated_at: Self::opt_dt_to_string(&todo.update_time),
            deleted_at: Self::opt_dt_to_string(&todo.deleted_at),
            start_date: Self::opt_dt_to_string(&todo.start_date),
            remind_at: Self::opt_dt_to_string(&todo.remind_at),
            is_reminded: todo.is_reminded,
            parent_id: todo.parent_id,
            note_id: todo.note_id,
            recurrence_type: todo.recurrence_type,
            recurrence_interval: todo.recurrence_interval,
            recurrence_end_date: Self::opt_dt_to_string(&todo.recurrence_end_date),
            mcp_access: todo.mcp_access,
            tags: existing_tags,
        };

        let yaml = yaml_serde::to_string(&fm).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("序列化待办 frontmatter 失败: {}", e))
        })?;
        let file_content = Self::join_frontmatter(&yaml, &todo.description);

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("创建目录失败: {}", e)))?;
        }

        tokio::fs::write(path, file_content)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入待办文件失败: {}", e)))?;

        Ok(())
    }

    /// 读取模板文件
    async fn read_template_file(&self, path: &Path) -> Result<NoteTemplate, AppError> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取模板文件失败: {}", e)))?;

        let (yaml, body) = Self::split_frontmatter(&content);

        let fm: TemplateFrontmatter = if let Some(y) = yaml {
            yaml_serde::from_str(&y).map_err(|e| {
                AppError::Internal(anyhow::anyhow!("解析模板 frontmatter 失败: {}", e))
            })?
        } else {
            TemplateFrontmatter::default()
        };

        Ok(NoteTemplate {
            id: fm.id,
            uuid: fm.uuid,
            name: fm.name,
            content: body,
            content_type: fm.content_type,
            sort_order: fm.sort_order,
            create_time: Self::string_to_opt_dt(&fm.created_at),
            update_time: Self::string_to_opt_dt(&fm.updated_at),
        })
    }

    /// 写入模板文件
    async fn write_template_file(
        &self,
        path: &Path,
        template: &NoteTemplate,
    ) -> Result<(), AppError> {
        let fm = TemplateFrontmatter {
            id: template.id,
            uuid: template.uuid.clone(),
            name: template.name.clone(),
            content_type: template.content_type,
            sort_order: template.sort_order,
            created_at: Self::opt_dt_to_string(&template.create_time),
            updated_at: Self::opt_dt_to_string(&template.update_time),
        };

        let yaml = yaml_serde::to_string(&fm).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("序列化模板 frontmatter 失败: {}", e))
        })?;
        let file_content = Self::join_frontmatter(&yaml, &template.content);

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("创建目录失败: {}", e)))?;
        }

        tokio::fs::write(path, file_content)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入模板文件失败: {}", e)))?;

        Ok(())
    }

    // ========= 加载所有数据 =========

    /// 加载所有笔记
    async fn load_all_notes(&self) -> Result<Vec<Note>, AppError> {
        let paths: Vec<PathBuf> = {
            let idx = self.note_index.read().unwrap();
            idx.values().cloned().collect()
        };

        let mut notes = Vec::with_capacity(paths.len());
        for path in paths {
            match self.read_note_file(&path).await {
                Ok(note) => notes.push(note),
                Err(e) => tracing::warn!("加载笔记失败 {:?}: {}", path, e),
            }
        }
        Ok(notes)
    }

    /// 加载所有待办
    async fn load_all_todos(&self) -> Result<Vec<Todo>, AppError> {
        let paths: Vec<PathBuf> = {
            let idx = self.todo_index.read().unwrap();
            idx.values().cloned().collect()
        };

        let mut todos = Vec::with_capacity(paths.len());
        for path in paths {
            match self.read_todo_file(&path).await {
                Ok(todo) => todos.push(todo),
                Err(e) => tracing::warn!("加载待办失败 {:?}: {}", path, e),
            }
        }
        Ok(todos)
    }

    /// 加载所有模板
    async fn load_all_templates(&self) -> Result<Vec<NoteTemplate>, AppError> {
        let paths: Vec<PathBuf> = {
            let idx = self.template_index.read().unwrap();
            idx.values().cloned().collect()
        };

        let mut templates = Vec::with_capacity(paths.len());
        for path in paths {
            match self.read_template_file(&path).await {
                Ok(template) => templates.push(template),
                Err(e) => tracing::warn!("加载模板失败 {:?}: {}", path, e),
            }
        }
        Ok(templates)
    }

    // ========= 标签辅助 =========

    /// 从标签名称列表构建 Tag 对象列表
    fn tags_from_names(&self, names: &[String]) -> Vec<Tag> {
        let tags = self.tags.read().unwrap();
        names
            .iter()
            .map(|name| {
                tags.iter()
                    .find(|t| &t.name == name)
                    .map(|t| self.tag_entry_to_tag(t))
                    .unwrap_or(Tag {
                        name: name.clone(),
                        ..Default::default()
                    })
            })
            .collect()
    }

    /// TagEntry → Tag
    fn tag_entry_to_tag(&self, entry: &TagEntry) -> Tag {
        Tag {
            id: entry.id,
            name: entry.name.clone(),
            icon: entry.icon.clone(),
            cls: entry.cls.clone(),
            sort_order: entry.sort_order,
            mcp_access: entry.mcp_access,
            create_time: Self::string_to_opt_dt(&entry.create_time),
            update_time: Self::string_to_opt_dt(&entry.update_time),
        }
    }

    // ========= 日期辅助 =========

    /// 判断给定日期时间是否为今天
    fn is_today(dt: &NaiveDateTime) -> bool {
        let today = Local::now().date_naive();
        let dt_date = dt.date();
        today == dt_date
    }

    /// 判断给定日期时间是否已过期（早于今天）
    fn is_overdue(dt: &NaiveDateTime) -> bool {
        let today = Local::now().date_naive();
        dt.date() < today
    }

    // ========= 附件辅助 =========

    /// 计算文件扩展名
    fn ext_from_mime(mime_type: &str, file_name: &str) -> String {
        // 优先从文件名提取
        if let Some(ext) = Path::new(file_name).extension() {
            return ext.to_string_lossy().to_string();
        }
        // 从 MIME 类型推断
        match mime_type {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/gif" => "gif",
            "image/webp" => "webp",
            "image/svg+xml" => "svg",
            "application/pdf" => "pdf",
            "text/plain" => "txt",
            "text/markdown" => "md",
            "application/zip" => "zip",
            "application/x-tar" => "tar",
            "application/gzip" => "gz",
            _ => "bin",
        }
        .to_string()
    }

    /// 扫描所有笔记的附件记录
    async fn scan_all_attachments(&self) -> Result<Vec<AttachmentEntry>, AppError> {
        let paths: Vec<PathBuf> = {
            let idx = self.note_index.read().unwrap();
            idx.values().cloned().collect()
        };

        let mut all = Vec::new();
        for path in &paths {
            let content = match tokio::fs::read_to_string(path).await {
                Ok(c) => c,
                Err(_) => continue,
            };
            let (yaml, _) = Self::split_frontmatter(&content);
            if let Some(y) = yaml {
                if let Ok(fm) = yaml_serde::from_str::<NoteFrontmatter>(&y) {
                    all.extend(fm.attachments);
                }
            }
        }
        Ok(all)
    }
}

// ============================================================================
// NoteStore 实现
// ============================================================================

#[async_trait::async_trait]
impl NoteStore for GitStore {
    // ======== 笔记 ========

    async fn create_note(&self, note: &Note) -> Result<Option<Note>, AppError> {
        let mut note = note.clone();
        if note.id == 0 {
            note.id = self.gen_id();
        }
        if note.uuid.is_none() {
            note.uuid = Some(Self::gen_uuid());
        }
        let now = Utc::now().naive_local();
        if note.create_time.is_none() {
            note.create_time = Some(now);
        }
        note.update_time = Some(now);

        let uuid = note.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let uuid = if uuid.is_empty() {
            Self::gen_uuid()
        } else {
            uuid
        };
        note.uuid = Some(uuid.clone());

        let notebook_name = if note.notebook_name.is_empty() {
            self.get_notebook_name(note.notebook_id)
        } else {
            note.notebook_name.clone()
        };
        note.notebook_name = notebook_name;

        let path = self.note_file_path(&uuid, &note.notebook_name, false);
        self.write_note_file(&path, &note).await?;
        self.index_note(note.id, path);
        self.git_commit(&format!("Create note: {}", note.title));

        Ok(Some(note))
    }

    async fn update_note(&self, note: &Note) -> Result<Option<Note>, AppError> {
        // 查找现有文件
        let old_path = self
            .get_note_path(note.id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("笔记不存在: id={}", note.id)))?;

        // 读取旧文件获取 uuid 和 links
        let old_content = tokio::fs::read_to_string(&old_path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取旧笔记失败: {}", e)))?;
        let (old_yaml, _) = Self::split_frontmatter(&old_content);
        let old_fm: NoteFrontmatter = if let Some(y) = old_yaml {
            yaml_serde::from_str(&y).unwrap_or_default()
        } else {
            NoteFrontmatter::default()
        };

        let mut note = note.clone();
        note.uuid = old_fm.uuid.clone().or_else(|| Some(Self::gen_uuid()));
        note.update_time = Some(Utc::now().naive_local());

        // 如果笔记本名称变化，需要移动文件
        let new_notebook_name = if note.notebook_name.is_empty() {
            self.get_notebook_name(note.notebook_id)
        } else {
            note.notebook_name.clone()
        };
        note.notebook_name = new_notebook_name;

        let deleted = old_fm.deleted_at.is_some();
        let uuid = note.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let new_path = self.note_file_path(&uuid, &note.notebook_name, deleted);

        // 写入新位置
        self.write_note_file(&new_path, &note).await?;

        // 如果路径变化，删除旧文件
        if new_path != old_path {
            let _ = tokio::fs::remove_file(&old_path).await;
            self.index_note(note.id, new_path.clone());
        }

        self.git_commit(&format!("Update note: {}", note.title));
        Ok(Some(note))
    }

    async fn delete_note_by_id(&self, id: i64) -> Result<(), AppError> {
        let path = self
            .get_note_path(id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("笔记不存在: id={}", id)))?;

        let mut note = self.read_note_file(&path).await?;
        note.deleted_at = Some(Utc::now().naive_local());

        let uuid = note.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let new_path = self.note_file_path(&uuid, &note.notebook_name, true);

        // 写入回收站
        self.write_note_file(&new_path, &note).await?;

        // 删除旧位置文件
        if new_path != path {
            let _ = tokio::fs::remove_file(&path).await;
            self.index_note(id, new_path);
        }

        self.git_commit(&format!("Delete note: {}", note.title));
        Ok(())
    }

    async fn search_page_notes(
        &self,
        param: &NoteSearchPageParam,
    ) -> Result<PageResult<Note>, AppError> {
        let mut notes = self.load_all_notes().await?;

        // 过滤：只显示未删除的
        notes.retain(|n| n.deleted_at.is_none());

        // 按笔记本过滤
        if param.notebook_id > 0 {
            notes.retain(|n| n.notebook_id == param.notebook_id);
        }

        // 按标签过滤
        if param.tag_id > 0 {
            let tag_name = {
                let tags = self.tags.read().unwrap();
                tags.iter()
                    .find(|t| t.id == param.tag_id)
                    .map(|t| t.name.clone())
            }
            .unwrap_or_default();
            if !tag_name.is_empty() {
                notes.retain(|n| n.tags.iter().any(|t| t.name == tag_name));
            }
        }

        // 按关键词过滤（搜索标题和内容）
        if !param.keyword.is_empty() {
            let kw = param.keyword.to_lowercase();
            notes.retain(|n| {
                let title_match = n.title.to_lowercase().contains(&kw);
                let content_match = if crypto::is_encrypted(&n.content) {
                    let decrypted = self.decrypt_content(&n.content);
                    decrypted.to_lowercase().contains(&kw)
                } else {
                    n.content.to_lowercase().contains(&kw)
                };
                title_match || content_match
            });
        }

        // 按星标过滤
        if param.is_starred {
            notes.retain(|n| n.is_starred == 1);
        }

        // 排序
        let sort_field = if param.sort_field.is_empty() {
            "update_time"
        } else {
            param.sort_field.as_str()
        };
        let sort_desc = param.sort_order != "asc";

        notes.sort_by(|a, b| {
            let ord = match sort_field {
                "title" => a.title.cmp(&b.title),
                "create_time" => a.create_time.cmp(&b.create_time),
                _ => a.update_time.cmp(&b.update_time),
            };
            if sort_desc { ord.reverse() } else { ord }
        });

        // 分页
        let total = notes.len() as i64;
        let start = param.page_param.start() as usize;
        let page_size = param.page_param.page_size as usize;
        if start < notes.len() {
            notes.truncate((start + page_size).min(notes.len()));
            notes.drain(..start);
        } else {
            notes.clear();
        }

        let mut result = PageResult::with_data(total, notes);
        result.total_pages(param.page_param.page_size);
        Ok(result)
    }

    async fn note_stats(&self, param: &NoteSearchPageParam) -> Result<NoteStatsResult, AppError> {
        let notes = self.load_all_notes().await?;

        let mut filtered: Vec<&Note> = notes.iter().filter(|n| n.deleted_at.is_none()).collect();

        if param.notebook_id > 0 {
            filtered.retain(|n| n.notebook_id == param.notebook_id);
        }
        if param.is_starred {
            filtered.retain(|n| n.is_starred == 1);
        }
        if !param.keyword.is_empty() {
            let kw = param.keyword.to_lowercase();
            filtered.retain(|n| {
                n.title.to_lowercase().contains(&kw)
                    || (!crypto::is_encrypted(&n.content) && n.content.to_lowercase().contains(&kw))
            });
        }

        let total = filtered.len() as i64;
        let mut notebook_counts = HashMap::new();
        for n in &filtered {
            *notebook_counts.entry(n.notebook_id).or_insert(0) += 1;
        }

        Ok(NoteStatsResult {
            total,
            notebook_counts,
        })
    }

    async fn batch_move_notes(&self, note_ids: &[i64], notebook_id: i64) -> Result<(), AppError> {
        let notebook_name = self.get_notebook_name(notebook_id);

        for &id in note_ids {
            if let Some(path) = self.get_note_path(id) {
                let mut note = self.read_note_file(&path).await?;
                let old_uuid = note.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
                note.notebook_id = notebook_id;
                note.notebook_name = notebook_name.clone();
                note.update_time = Some(Utc::now().naive_local());

                let new_path = self.note_file_path(&old_uuid, &notebook_name, false);
                self.write_note_file(&new_path, &note).await?;
                if new_path != path {
                    let _ = tokio::fs::remove_file(&path).await;
                    self.index_note(id, new_path);
                }
            }
        }

        self.git_commit(&format!(
            "Move {} notes to notebook {}",
            note_ids.len(),
            notebook_name
        ));
        Ok(())
    }

    async fn batch_delete_notes(&self, note_ids: &[i64]) -> Result<(), AppError> {
        for &id in note_ids {
            let _ = self.delete_note_by_id(id).await;
        }
        Ok(())
    }

    async fn toggle_note_star(&self, id: i64) -> Result<Option<Note>, AppError> {
        let path = self
            .get_note_path(id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("笔记不存在: id={}", id)))?;

        let mut note = self.read_note_file(&path).await?;
        note.is_starred = if note.is_starred == 1 { 0 } else { 1 };
        note.update_time = Some(Utc::now().naive_local());

        self.write_note_file(&path, &note).await?;
        self.git_commit(&format!("Toggle star: {}", note.title));
        Ok(Some(note))
    }

    async fn toggle_note_pin(&self, id: i64) -> Result<Option<Note>, AppError> {
        let path = self
            .get_note_path(id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("笔记不存在: id={}", id)))?;

        let mut note = self.read_note_file(&path).await?;
        note.is_pinned = if note.is_pinned == 1 { 0 } else { 1 };
        note.update_time = Some(Utc::now().naive_local());

        self.write_note_file(&path, &note).await?;
        self.git_commit(&format!("Toggle pin: {}", note.title));
        Ok(Some(note))
    }

    async fn restore_note(&self, id: i64) -> Result<(), AppError> {
        let path = self
            .get_note_path(id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("笔记不存在: id={}", id)))?;

        let mut note = self.read_note_file(&path).await?;
        note.deleted_at = None;
        note.update_time = Some(Utc::now().naive_local());

        let uuid = note.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let new_path = self.note_file_path(&uuid, &note.notebook_name, false);

        self.write_note_file(&new_path, &note).await?;
        if new_path != path {
            let _ = tokio::fs::remove_file(&path).await;
            self.index_note(id, new_path);
        }

        self.git_commit(&format!("Restore note: {}", note.title));
        Ok(())
    }

    async fn permanent_delete_note(&self, id: i64) -> Result<(), AppError> {
        let path = self
            .get_note_path(id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("笔记不存在: id={}", id)))?;

        let _ = tokio::fs::remove_file(&path).await;
        self.unindex_note(id);
        self.git_commit(&format!("Permanently delete note: {}", id));
        Ok(())
    }

    async fn empty_trash(&self) -> Result<(), AppError> {
        let trash_dir = self.notes_dir().join("_trash");
        if let Ok(mut entries) = tokio::fs::read_dir(&trash_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    // 解析 ID 并从索引中移除
                    if let Ok(content) = tokio::fs::read_to_string(&path).await {
                        let (yaml, _) = Self::split_frontmatter(&content);
                        if let Some(y) = yaml {
                            if let Ok(fm) = yaml_serde::from_str::<NoteFrontmatter>(&y) {
                                self.unindex_note(fm.id);
                            }
                        }
                    }
                    let _ = tokio::fs::remove_file(&path).await;
                }
            }
        }
        self.git_commit("Empty trash");
        Ok(())
    }

    async fn find_deleted_notes(
        &self,
        page_param: &PageParam,
    ) -> Result<PageResult<Note>, AppError> {
        let trash_dir = self.notes_dir().join("_trash");
        let mut notes = Vec::new();

        if let Ok(mut entries) = tokio::fs::read_dir(&trash_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    if let Ok(note) = self.read_note_file(&path).await {
                        notes.push(note);
                    }
                }
            }
        }

        // 按删除时间倒序
        notes.sort_by(|a, b| b.deleted_at.cmp(&a.deleted_at));

        let total = notes.len() as i64;
        let start = page_param.start() as usize;
        let page_size = page_param.page_size as usize;
        if start < notes.len() {
            notes.truncate((start + page_size).min(notes.len()));
            notes.drain(..start);
        } else {
            notes.clear();
        }

        let mut result = PageResult::with_data(total, notes);
        result.total_pages(page_param.page_size);
        Ok(result)
    }

    async fn search_page_note_histories(
        &self,
        _param: &NoteHistorySearchPageParam,
    ) -> Result<PageResult<NoteHistory>, AppError> {
        // GitStore 不支持结构化历史，用 git log 替代（未来实现）
        Ok(PageResult::default())
    }

    // ======== 笔记本 ========

    async fn find_all_notebooks(&self) -> Result<Vec<Notebook>, AppError> {
        let idx = self.notebook_index.read().unwrap();
        let mut notebooks: Vec<Notebook> = idx
            .values()
            .map(|m| Notebook {
                id: m.id,
                parent_id: m.parent_id,
                name: m.name.clone(),
                description: m.description.clone(),
                icon: m.icon.clone(),
                cls: m.cls.clone(),
                sort_order: m.sort_order,
                mcp_access: m.mcp_access,
                create_time: Self::string_to_opt_dt(&m.create_time),
                update_time: Self::string_to_opt_dt(&m.update_time),
            })
            .collect();
        notebooks.sort_by(|a, b| a.sort_order.cmp(&b.sort_order).then(a.id.cmp(&b.id)));
        Ok(notebooks)
    }

    async fn create_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError> {
        let mut notebook = notebook.clone();
        if notebook.id == 0 {
            notebook.id = self.gen_id();
        }
        let now = Utc::now().naive_local();
        if notebook.create_time.is_none() {
            notebook.create_time = Some(now);
        }
        notebook.update_time = Some(now);

        let dir_name = Self::safe_dir_name(&notebook.name);
        let dir_path = self.notes_dir().join(&dir_name);
        tokio::fs::create_dir_all(&dir_path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("创建笔记本目录失败: {}", e)))?;

        let meta = NotebookMeta {
            id: notebook.id,
            parent_id: notebook.parent_id,
            name: notebook.name.clone(),
            description: notebook.description.clone(),
            icon: notebook.icon.clone(),
            cls: notebook.cls.clone(),
            sort_order: notebook.sort_order,
            mcp_access: notebook.mcp_access,
            create_time: Self::opt_dt_to_string(&notebook.create_time),
            update_time: Self::opt_dt_to_string(&notebook.update_time),
            dir_name,
        };

        let meta_path = dir_path.join(".notebook.yml");
        let yaml = yaml_serde::to_string(&meta)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("序列化笔记本元数据失败: {}", e)))?;
        tokio::fs::write(&meta_path, yaml)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入笔记本元数据失败: {}", e)))?;

        {
            let mut idx = self.notebook_index.write().unwrap();
            idx.insert(notebook.id, meta);
        }

        self.git_commit(&format!("Create notebook: {}", notebook.name));
        Ok(Some(notebook))
    }

    async fn update_notebook(&self, notebook: &Notebook) -> Result<Option<Notebook>, AppError> {
        let mut notebook = notebook.clone();
        notebook.update_time = Some(Utc::now().naive_local());

        // 查找旧元数据
        let old_meta = {
            let idx = self.notebook_index.read().unwrap();
            idx.get(&notebook.id).cloned()
        }
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("笔记本不存在: id={}", notebook.id)))?;

        let new_dir_name = Self::safe_dir_name(&notebook.name);
        let old_dir_path = self.notes_dir().join(&old_meta.dir_name);
        let new_dir_path = self.notes_dir().join(&new_dir_name);

        // 如果目录名变化，重命名目录
        if old_meta.dir_name != new_dir_name {
            if old_dir_path.exists() {
                tokio::fs::rename(&old_dir_path, &new_dir_path)
                    .await
                    .map_err(|e| {
                        AppError::Internal(anyhow::anyhow!("重命名笔记本目录失败: {}", e))
                    })?;
            } else {
                tokio::fs::create_dir_all(&new_dir_path)
                    .await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("创建目录失败: {}", e)))?;
            }

            // 更新该目录下所有笔记的 notebook 字段
            if let Ok(mut entries) = tokio::fs::read_dir(&new_dir_path).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.extension().map(|e| e == "md").unwrap_or(false) {
                        if let Ok(mut note) = self.read_note_file(&path).await {
                            note.notebook_name = notebook.name.clone();
                            note.update_time = Some(Utc::now().naive_local());
                            let _ = self.write_note_file(&path, &note).await;
                        }
                    }
                }
            }

            // 更新 note_index 路径
            {
                let mut idx = self.note_index.write().unwrap();
                for path in idx.values_mut() {
                    if let Some(rel) = path.strip_prefix(&old_dir_path).ok() {
                        *path = new_dir_path.join(rel);
                    }
                }
            }
        }

        let meta = NotebookMeta {
            id: notebook.id,
            parent_id: notebook.parent_id,
            name: notebook.name.clone(),
            description: notebook.description.clone(),
            icon: notebook.icon.clone(),
            cls: notebook.cls.clone(),
            sort_order: notebook.sort_order,
            mcp_access: notebook.mcp_access,
            create_time: Self::opt_dt_to_string(&notebook.create_time),
            update_time: Self::opt_dt_to_string(&notebook.update_time),
            dir_name: new_dir_name,
        };

        let meta_path = new_dir_path.join(".notebook.yml");
        let yaml = yaml_serde::to_string(&meta)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("序列化笔记本元数据失败: {}", e)))?;
        tokio::fs::write(&meta_path, yaml)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入笔记本元数据失败: {}", e)))?;

        {
            let mut idx = self.notebook_index.write().unwrap();
            idx.insert(notebook.id, meta);
        }

        self.git_commit(&format!("Update notebook: {}", notebook.name));
        Ok(Some(notebook))
    }

    async fn delete_notebook_by_id(&self, id: i64) -> Result<(), AppError> {
        let meta = {
            let idx = self.notebook_index.read().unwrap();
            idx.get(&id).cloned()
        }
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("笔记本不存在: id={}", id)))?;

        let dir_path = self.notes_dir().join(&meta.dir_name);

        // 将该目录下的笔记移入回收站
        if let Ok(mut entries) = tokio::fs::read_dir(&dir_path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    if let Ok(mut note) = self.read_note_file(&path).await {
                        note.deleted_at = Some(Utc::now().naive_local());
                        note.notebook_name = meta.name.clone();
                        let uuid = note.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
                        let trash_path = self.note_file_path(&uuid, &note.notebook_name, true);
                        self.write_note_file(&trash_path, &note).await?;
                        self.index_note(note.id, trash_path);
                        let _ = tokio::fs::remove_file(&path).await;
                    }
                }
            }
        }

        // 删除目录
        let _ = tokio::fs::remove_dir_all(&dir_path).await;

        {
            let mut idx = self.notebook_index.write().unwrap();
            idx.remove(&id);
        }

        self.git_commit(&format!("Delete notebook: {}", meta.name));
        Ok(())
    }

    async fn reorder_notebooks(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        let mut idx = self.notebook_index.write().unwrap();
        for (id, sort_order) in orders {
            if let Some(meta) = idx.get_mut(&id) {
                meta.sort_order = sort_order;
                meta.update_time = Some(Self::dt_to_string(&Utc::now().naive_local()));

                // 写回元数据文件
                let dir_path = self.notes_dir().join(&meta.dir_name);
                let meta_path = dir_path.join(".notebook.yml");
                if let Ok(yaml) = yaml_serde::to_string(&*meta) {
                    let _ = std::fs::write(&meta_path, yaml);
                }
            }
        }
        drop(idx);

        self.git_commit("Reorder notebooks");
        Ok(())
    }

    // ======== 标签 ========

    async fn find_all_tags(&self) -> Result<Vec<Tag>, AppError> {
        let tags = self.tags.read().unwrap();
        let mut result: Vec<Tag> = tags.iter().map(|t| self.tag_entry_to_tag(t)).collect();
        result.sort_by(|a, b| a.sort_order.cmp(&b.sort_order).then(a.id.cmp(&b.id)));
        Ok(result)
    }

    async fn create_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError> {
        let mut tag = tag.clone();
        if tag.id == 0 {
            tag.id = self.gen_id();
        }
        let now = Utc::now().naive_local();
        if tag.create_time.is_none() {
            tag.create_time = Some(now);
        }
        tag.update_time = Some(now);

        let entry = TagEntry {
            id: tag.id,
            name: tag.name.clone(),
            icon: tag.icon.clone(),
            cls: tag.cls.clone(),
            sort_order: tag.sort_order,
            mcp_access: tag.mcp_access,
            create_time: Self::opt_dt_to_string(&tag.create_time),
            update_time: Self::opt_dt_to_string(&tag.update_time),
        };

        {
            let mut tags = self.tags.write().unwrap();
            tags.push(entry);
            self.save_tags_sync(&tags);
        }

        self.git_commit(&format!("Create tag: {}", tag.name));
        Ok(Some(tag))
    }

    async fn update_tag(&self, tag: &Tag) -> Result<Option<Tag>, AppError> {
        let mut tag = tag.clone();
        tag.update_time = Some(Utc::now().naive_local());

        let entry = TagEntry {
            id: tag.id,
            name: tag.name.clone(),
            icon: tag.icon.clone(),
            cls: tag.cls.clone(),
            sort_order: tag.sort_order,
            mcp_access: tag.mcp_access,
            create_time: Self::opt_dt_to_string(&tag.create_time),
            update_time: Self::opt_dt_to_string(&tag.update_time),
        };

        {
            let mut tags = self.tags.write().unwrap();
            if let Some(existing) = tags.iter_mut().find(|t| t.id == tag.id) {
                *existing = entry;
            } else {
                tags.push(entry);
            }
            self.save_tags_sync(&tags);
        }

        self.git_commit(&format!("Update tag: {}", tag.name));
        Ok(Some(tag))
    }

    async fn delete_tag_by_id(&self, id: i64) -> Result<(), AppError> {
        // 先取出被删标签的名称（索引用于按名清理笔记 frontmatter，删除后查不到了）
        let removed_name: Option<String> = {
            let mut tags = self.tags.write().unwrap();
            let name = tags.iter().find(|t| t.id == id).map(|t| t.name.clone());
            tags.retain(|t| t.id != id);
            self.save_tags_sync(&tags);
            name
        };

        // 从所有笔记中移除该标签
        let note_paths: Vec<PathBuf> = {
            let idx = self.note_index.read().unwrap();
            idx.values().cloned().collect()
        };

        for path in note_paths {
            if let Ok(content) = tokio::fs::read_to_string(&path).await {
                let (yaml, body) = Self::split_frontmatter(&content);
                if let Some(y) = yaml {
                    if let Ok(mut fm) = yaml_serde::from_str::<NoteFrontmatter>(&y) {
                        let before_len = fm.tags.len();
                        if let Some(rm) = &removed_name {
                            fm.tags.retain(|name| name != rm);
                        }
                        if fm.tags.len() != before_len {
                            let new_yaml = yaml_serde::to_string(&fm).unwrap_or_default();
                            let new_content = Self::join_frontmatter(&new_yaml, &body);
                            let _ = tokio::fs::write(&path, new_content).await;
                        }
                    }
                }
            }
        }

        self.git_commit(&format!("Delete tag: {}", id));
        Ok(())
    }

    async fn reorder_tags(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        let mut tags = self.tags.write().unwrap();
        for (id, sort_order) in orders {
            if let Some(tag) = tags.iter_mut().find(|t| t.id == id) {
                tag.sort_order = sort_order;
                tag.update_time = Some(Self::dt_to_string(&Utc::now().naive_local()));
            }
        }
        self.save_tags_sync(&tags);
        drop(tags);

        self.git_commit("Reorder tags");
        Ok(())
    }

    // ======== 模板 ========

    async fn find_all_templates(&self) -> Result<Vec<NoteTemplate>, AppError> {
        let templates = self.load_all_templates().await?;
        let mut templates = templates;
        templates.sort_by(|a, b| a.sort_order.cmp(&b.sort_order).then(a.id.cmp(&b.id)));
        Ok(templates)
    }

    async fn create_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError> {
        let mut template = template.clone();
        if template.id == 0 {
            template.id = self.gen_id();
        }
        if template.uuid.is_none() || template.uuid.as_ref().map(|u| u.is_empty()).unwrap_or(true) {
            template.uuid = Some(Self::gen_uuid());
        }
        let now = Utc::now().naive_local();
        if template.create_time.is_none() {
            template.create_time = Some(now);
        }
        template.update_time = Some(now);

        let uuid = template.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let path = self.template_file_path(&uuid);
        self.write_template_file(&path, &template).await?;
        self.index_template(template.id, path);

        self.git_commit(&format!("Create template: {}", template.name));
        Ok(Some(template))
    }

    async fn update_template(
        &self,
        template: &NoteTemplate,
    ) -> Result<Option<NoteTemplate>, AppError> {
        let mut template = template.clone();
        template.update_time = Some(Utc::now().naive_local());

        // 查找现有路径
        let path = self
            .get_template_path(template.id)
            .or_else(|| {
                // 尝试用 uuid 查找
                template.uuid.as_ref().and_then(|uuid| {
                    let p = self.template_file_path(uuid);
                    if p.exists() { Some(p) } else { None }
                })
            })
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("模板不存在: id={}", template.id)))?;

        if template.uuid.is_none() {
            template.uuid = Some(Self::gen_uuid());
        }

        let uuid = template.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let new_path = self.template_file_path(&uuid);

        self.write_template_file(&new_path, &template).await?;
        if new_path != path {
            let _ = tokio::fs::remove_file(&path).await;
            self.index_template(template.id, new_path);
        }

        self.git_commit(&format!("Update template: {}", template.name));
        Ok(Some(template))
    }

    async fn delete_template_by_id(&self, id: i64) -> Result<(), AppError> {
        if let Some(path) = self.get_template_path(id) {
            let _ = tokio::fs::remove_file(&path).await;
            self.unindex_template(id);
        }
        self.git_commit(&format!("Delete template: {}", id));
        Ok(())
    }

    // ======== 笔记链接 ========

    async fn find_note_links(&self, note_id: i64) -> Result<Vec<NoteLink>, AppError> {
        let path = match self.get_note_path(note_id) {
            Some(p) => p,
            None => return Ok(Vec::new()),
        };

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取笔记失败: {}", e)))?;
        let (yaml, _) = Self::split_frontmatter(&content);
        let fm: NoteFrontmatter = if let Some(y) = yaml {
            yaml_serde::from_str(&y).unwrap_or_default()
        } else {
            NoteFrontmatter::default()
        };

        Ok(fm
            .links
            .iter()
            .map(|l| NoteLink {
                id: l.id,
                note_id: l.note_id,
                note_title: l.note_title.clone(),
                create_time: Self::string_to_opt_dt(&l.create_time),
            })
            .collect())
    }

    async fn create_note_link(
        &self,
        source_note_id: i64,
        target_note_id: i64,
    ) -> Result<(), AppError> {
        let source_path = self.get_note_path(source_note_id).ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!("源笔记不存在: id={}", source_note_id))
        })?;

        // 读取目标笔记标题
        let target_path = self.get_note_path(target_note_id);
        let target_title = if let Some(tp) = target_path {
            self.read_note_file(&tp)
                .await
                .map(|n| n.title)
                .unwrap_or_default()
        } else {
            String::new()
        };

        // 读取源笔记 frontmatter
        let content = tokio::fs::read_to_string(&source_path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取源笔记失败: {}", e)))?;
        let (yaml, body) = Self::split_frontmatter(&content);
        let mut fm: NoteFrontmatter = if let Some(y) = yaml {
            yaml_serde::from_str(&y).unwrap_or_default()
        } else {
            NoteFrontmatter::default()
        };

        let link_id = {
            let mut counter = self.next_link_id.write().unwrap();
            *counter += 1;
            *counter
        };

        fm.links.push(LinkEntry {
            id: link_id,
            note_id: target_note_id,
            note_title: target_title,
            create_time: Some(Self::dt_to_string(&Utc::now().naive_local())),
        });

        let new_yaml = yaml_serde::to_string(&fm)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("序列化失败: {}", e)))?;
        let new_content = Self::join_frontmatter(&new_yaml, &body);
        tokio::fs::write(&source_path, new_content)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入笔记失败: {}", e)))?;

        self.git_commit(&format!(
            "Create note link: {} -> {}",
            source_note_id, target_note_id
        ));
        Ok(())
    }

    async fn delete_note_link(&self, link_id: i64) -> Result<(), AppError> {
        // 扫描所有笔记查找该链接
        let paths: Vec<PathBuf> = {
            let idx = self.note_index.read().unwrap();
            idx.values().cloned().collect()
        };

        for path in paths {
            let content = match tokio::fs::read_to_string(&path).await {
                Ok(c) => c,
                Err(_) => continue,
            };
            let (yaml, body) = Self::split_frontmatter(&content);
            let mut fm: NoteFrontmatter = if let Some(y) = yaml {
                yaml_serde::from_str(&y).unwrap_or_default()
            } else {
                continue;
            };

            let before_len = fm.links.len();
            fm.links.retain(|l| l.id != link_id);
            if fm.links.len() != before_len {
                let new_yaml = yaml_serde::to_string(&fm).unwrap_or_default();
                let new_content = Self::join_frontmatter(&new_yaml, &body);
                let _ = tokio::fs::write(&path, new_content).await;
                self.git_commit(&format!("Delete note link: {}", link_id));
                return Ok(());
            }
        }

        Ok(())
    }

    async fn search_linkable_notes(
        &self,
        note_id: i64,
        keyword: &str,
    ) -> Result<Vec<NoteLink>, AppError> {
        let notes = self.load_all_notes().await?;
        let kw = keyword.to_lowercase();

        let mut result: Vec<NoteLink> = notes
            .iter()
            .filter(|n| n.id != note_id && n.deleted_at.is_none())
            .filter(|n| {
                if kw.is_empty() {
                    true
                } else {
                    n.title.to_lowercase().contains(&kw)
                }
            })
            .map(|n| NoteLink {
                id: 0,
                note_id: n.id,
                note_title: n.title.clone(),
                create_time: None,
            })
            .collect();

        result.sort_by(|a, b| a.note_title.cmp(&b.note_title));
        Ok(result)
    }

    // ======== 附件 ========

    async fn save_attachment(
        &self,
        note_id: i64,
        file_name: &str,
        file_data: &[u8],
        mime_type: &str,
    ) -> Result<NoteAttachment, AppError> {
        // 计算 sha256
        let mut hasher = Sha256::new();
        hasher.update(file_data);
        let hash = hasher.finalize();
        let file_hash: String = hash.iter().map(|b| format!("{:02x}", b)).collect();

        let ext = Self::ext_from_mime(mime_type, file_name);
        let attach_filename = format!("{}.{}", file_hash, ext);
        let attach_path = self.attachments_dir().join(&attach_filename);

        // 如果文件不存在则写入（去重）
        if !attach_path.exists() {
            tokio::fs::create_dir_all(self.attachments_dir())
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("创建附件目录失败: {}", e)))?;
            tokio::fs::write(&attach_path, file_data)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("写入附件文件失败: {}", e)))?;
        }

        let att_id = {
            let mut counter = self.next_attachment_id.write().unwrap();
            *counter += 1;
            *counter
        };
        let now = Utc::now().naive_local();

        let entry = AttachmentEntry {
            id: att_id,
            note_id,
            file_name: file_name.to_string(),
            file_path: attach_filename,
            file_size: file_data.len() as i64,
            mime_type: mime_type.to_string(),
            file_hash: file_hash.clone(),
            ref_count: 1,
            create_time: Some(Self::dt_to_string(&now)),
        };

        // 添加到笔记 frontmatter
        if let Some(path) = self.get_note_path(note_id) {
            let content = tokio::fs::read_to_string(&path)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("读取笔记失败: {}", e)))?;
            let (yaml, body) = Self::split_frontmatter(&content);
            let mut fm: NoteFrontmatter = if let Some(y) = yaml {
                yaml_serde::from_str(&y).unwrap_or_default()
            } else {
                NoteFrontmatter::default()
            };
            fm.attachments.push(entry.clone());
            let new_yaml = yaml_serde::to_string(&fm)
                .map_err(|e| AppError::Internal(anyhow::anyhow!("序列化失败: {}", e)))?;
            let new_content = Self::join_frontmatter(&new_yaml, &body);
            tokio::fs::write(&path, new_content)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("写入笔记失败: {}", e)))?;
        }

        self.git_commit(&format!("Save attachment: {}", file_name));

        Ok(NoteAttachment {
            id: entry.id,
            note_id: entry.note_id,
            file_name: entry.file_name,
            file_path: entry.file_path,
            file_size: entry.file_size,
            mime_type: entry.mime_type,
            file_hash: entry.file_hash,
            ref_count: entry.ref_count,
            create_time: Some(now),
        })
    }

    async fn find_attachments(&self, note_id: i64) -> Result<Vec<NoteAttachment>, AppError> {
        let path = match self.get_note_path(note_id) {
            Some(p) => p,
            None => return Ok(Vec::new()),
        };

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取笔记失败: {}", e)))?;
        let (yaml, _) = Self::split_frontmatter(&content);
        let fm: NoteFrontmatter = if let Some(y) = yaml {
            yaml_serde::from_str(&y).unwrap_or_default()
        } else {
            NoteFrontmatter::default()
        };

        Ok(fm
            .attachments
            .iter()
            .map(|a| NoteAttachment {
                id: a.id,
                note_id: a.note_id,
                file_name: a.file_name.clone(),
                file_path: a.file_path.clone(),
                file_size: a.file_size,
                mime_type: a.mime_type.clone(),
                file_hash: a.file_hash.clone(),
                ref_count: a.ref_count,
                create_time: Self::string_to_opt_dt(&a.create_time),
            })
            .collect())
    }

    async fn delete_attachment(&self, id: i64) -> Result<(), AppError> {
        let paths: Vec<PathBuf> = {
            let idx = self.note_index.read().unwrap();
            idx.values().cloned().collect()
        };

        let mut file_hash_to_delete: Option<String> = None;
        let mut file_path_to_delete: Option<String> = None;

        for path in &paths {
            let content = match tokio::fs::read_to_string(path).await {
                Ok(c) => c,
                Err(_) => continue,
            };
            let (yaml, body) = Self::split_frontmatter(&content);
            let mut fm: NoteFrontmatter = if let Some(y) = yaml {
                yaml_serde::from_str(&y).unwrap_or_default()
            } else {
                continue;
            };

            let before_len = fm.attachments.len();
            if let Some(att) = fm.attachments.iter().find(|a| a.id == id) {
                file_hash_to_delete = Some(att.file_hash.clone());
                file_path_to_delete = Some(att.file_path.clone());
            }
            fm.attachments.retain(|a| a.id != id);

            if fm.attachments.len() != before_len {
                let new_yaml = yaml_serde::to_string(&fm).unwrap_or_default();
                let new_content = Self::join_frontmatter(&new_yaml, &body);
                let _ = tokio::fs::write(path, new_content).await;
                break;
            }
        }

        // 检查是否还有其他笔记引用该文件
        if let Some(ref hash) = file_hash_to_delete {
            let all_attachments = self.scan_all_attachments().await?;
            let still_referenced = all_attachments.iter().any(|a| &a.file_hash == hash);
            if !still_referenced {
                if let Some(fp) = file_path_to_delete {
                    let full_path = self.attachments_dir().join(&fp);
                    let _ = tokio::fs::remove_file(&full_path).await;
                }
            }
        }

        self.git_commit(&format!("Delete attachment: {}", id));
        Ok(())
    }

    async fn get_attachment_stats(&self) -> Result<AttachmentStats, AppError> {
        let all = self.scan_all_attachments().await?;
        let total_count = all.len() as u64;
        let total_size: i64 = all.iter().map(|a| a.file_size).sum();
        let unique_hashes: std::collections::HashSet<&str> =
            all.iter().map(|a| a.file_hash.as_str()).collect();
        let unique_file_count = unique_hashes.len() as u64;

        // 扫描 attachments 目录查找孤立文件
        let mut orphan_count = 0u64;
        if let Ok(mut entries) = tokio::fs::read_dir(self.attachments_dir()).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_file() {
                    let filename = match path.file_name().and_then(|n| n.to_str()) {
                        Some(n) => n.to_string(),
                        None => continue,
                    };
                    // 检查是否有笔记引用
                    let referenced = all.iter().any(|a| a.file_path == filename);
                    if !referenced {
                        orphan_count += 1;
                    }
                }
            }
        }

        Ok(AttachmentStats {
            total_count,
            unique_file_count,
            total_size,
            orphan_count,
        })
    }

    async fn cleanup_orphan_attachments(&self) -> Result<u32, AppError> {
        let all = self.scan_all_attachments().await?;
        let referenced_files: std::collections::HashSet<String> =
            all.iter().map(|a| a.file_path.clone()).collect();

        let mut cleaned = 0u32;
        if let Ok(mut entries) = tokio::fs::read_dir(self.attachments_dir()).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_file() {
                    let filename = match path.file_name().and_then(|n| n.to_str()) {
                        Some(n) => n.to_string(),
                        None => continue,
                    };
                    if !referenced_files.contains(&filename) {
                        let _ = tokio::fs::remove_file(&path).await;
                        cleaned += 1;
                    }
                }
            }
        }

        if cleaned > 0 {
            self.git_commit(&format!("Clean {} orphan attachments", cleaned));
        }
        Ok(cleaned)
    }
}

// ============================================================================
// TodoStore 实现
// ============================================================================

#[async_trait::async_trait]
impl TodoStore for GitStore {
    async fn search_todos(&self, param: &TodoSearchParam) -> Result<Vec<Todo>, AppError> {
        let mut todos = self.load_all_todos().await?;

        // 回收站过滤
        if param.deleted {
            todos.retain(|t| t.deleted_at.is_some());
        } else {
            todos.retain(|t| t.deleted_at.is_none());
        }

        // 关键词过滤
        if !param.keyword.is_empty() {
            let kw = param.keyword.to_lowercase();
            todos.retain(|t| {
                t.title.to_lowercase().contains(&kw) || t.description.to_lowercase().contains(&kw)
            });
        }

        // 清单过滤
        if let Some(list_id) = param.list_id {
            todos.retain(|t| t.list_id == Some(list_id));
        }

        // 标签过滤
        if let Some(tag_id) = param.tag_id {
            let paths: Vec<PathBuf> = {
                let idx = self.todo_index.read().unwrap();
                idx.values().cloned().collect()
            };
            // 获取有该标签的 todo id 集合
            let mut matching_ids = std::collections::HashSet::new();
            for path in &paths {
                if let Ok(content) = tokio::fs::read_to_string(path).await {
                    let (yaml, _) = Self::split_frontmatter(&content);
                    if let Some(y) = yaml {
                        if let Ok(fm) = yaml_serde::from_str::<TodoFrontmatter>(&y) {
                            if fm.tags.contains(&tag_id) {
                                matching_ids.insert(fm.id);
                            }
                        }
                    }
                }
            }
            todos.retain(|t| matching_ids.contains(&t.id));
        }

        // 完成状态过滤
        if let Some(completed) = param.completed {
            todos.retain(|t| {
                if completed {
                    t.is_completed == 1
                } else {
                    t.is_completed == 0
                }
            });
        }

        // 优先级过滤
        if let Some(priority) = param.priority {
            todos.retain(|t| t.priority == priority);
        }

        // 视图过滤
        match param.view.as_str() {
            "today" => {
                todos.retain(|t| {
                    t.is_completed == 0 && t.due_date.map(|d| Self::is_today(&d)).unwrap_or(false)
                });
            }
            "planned" => {
                todos.retain(|t| t.due_date.is_some() && t.is_completed == 0);
            }
            "overdue" => {
                todos.retain(|t| {
                    t.is_completed == 0 && t.due_date.map(|d| Self::is_overdue(&d)).unwrap_or(false)
                });
            }
            _ => {}
        }

        // 排序
        let sort_field = if param.sort_field.is_empty() {
            "manual"
        } else {
            param.sort_field.as_str()
        };
        let sort_desc = param.sort_order != "asc";

        todos.sort_by(|a, b| {
            let ord = match sort_field {
                "dueDate" => a.due_date.cmp(&b.due_date),
                "priority" => a.priority.cmp(&b.priority),
                "createTime" => a.create_time.cmp(&b.create_time),
                _ => a.sort_order.cmp(&b.sort_order),
            };
            if sort_desc { ord.reverse() } else { ord }
        });

        Ok(todos)
    }

    async fn find_todo(&self, id: i64) -> Result<Option<Todo>, AppError> {
        let path = match self.get_todo_path(id) {
            Some(p) => p,
            None => return Ok(None),
        };
        let todo = self.read_todo_file(&path).await?;
        Ok(Some(todo))
    }

    async fn create_todo(&self, todo: &Todo) -> Result<Todo, AppError> {
        let mut todo = todo.clone();
        if todo.id == 0 {
            todo.id = self.gen_id();
        }
        if todo.uuid.is_none() || todo.uuid.as_ref().map(|u| u.is_empty()).unwrap_or(true) {
            todo.uuid = Some(Self::gen_uuid());
        }
        let now = Utc::now().naive_local();
        if todo.create_time.is_none() {
            todo.create_time = Some(now);
        }
        todo.update_time = Some(now);

        let uuid = todo.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let list_name = self.get_todo_list_name(todo.list_id);
        let path = self.todo_file_path(&uuid, &list_name, false);
        self.write_todo_file(&path, &todo).await?;
        self.index_todo(todo.id, path);

        self.git_commit(&format!("Create todo: {}", todo.title));
        Ok(todo)
    }

    async fn update_todo(&self, todo: &Todo) -> Result<Option<Todo>, AppError> {
        let old_path = self
            .get_todo_path(todo.id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("待办不存在: id={}", todo.id)))?;

        // 读取旧文件获取 uuid
        let old_content = tokio::fs::read_to_string(&old_path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取旧待办失败: {}", e)))?;
        let (old_yaml, _) = Self::split_frontmatter(&old_content);
        let old_fm: TodoFrontmatter = if let Some(y) = old_yaml {
            yaml_serde::from_str(&y).unwrap_or_default()
        } else {
            TodoFrontmatter::default()
        };

        let mut todo = todo.clone();
        todo.uuid = old_fm.uuid.clone().or_else(|| Some(Self::gen_uuid()));
        todo.update_time = Some(Utc::now().naive_local());

        let deleted = old_fm.deleted_at.is_some();
        let uuid = todo.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let list_name = self.get_todo_list_name(todo.list_id);
        let new_path = self.todo_file_path(&uuid, &list_name, deleted);

        self.write_todo_file(&new_path, &todo).await?;
        if new_path != old_path {
            let _ = tokio::fs::remove_file(&old_path).await;
            self.index_todo(todo.id, new_path);
        }

        self.git_commit(&format!("Update todo: {}", todo.title));
        Ok(Some(todo))
    }

    async fn toggle_todo(&self, id: i64) -> Result<Option<Todo>, AppError> {
        let path = self
            .get_todo_path(id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("待办不存在: id={}", id)))?;

        let mut todo = self.read_todo_file(&path).await?;
        todo.is_completed = if todo.is_completed == 1 { 0 } else { 1 };
        if todo.is_completed == 1 {
            todo.completed_at = Some(Utc::now().naive_local());
        } else {
            todo.completed_at = None;
        }
        todo.update_time = Some(Utc::now().naive_local());

        self.write_todo_file(&path, &todo).await?;
        self.git_commit(&format!("Toggle todo: {}", todo.title));
        Ok(Some(todo))
    }

    async fn delete_todo(&self, id: i64) -> Result<(), AppError> {
        let path = self
            .get_todo_path(id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("待办不存在: id={}", id)))?;

        let mut todo = self.read_todo_file(&path).await?;
        todo.deleted_at = Some(Utc::now().naive_local());

        let uuid = todo.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let list_name = self.get_todo_list_name(todo.list_id);
        let new_path = self.todo_file_path(&uuid, &list_name, true);

        self.write_todo_file(&new_path, &todo).await?;
        if new_path != path {
            let _ = tokio::fs::remove_file(&path).await;
            self.index_todo(id, new_path);
        }

        self.git_commit(&format!("Delete todo: {}", todo.title));
        Ok(())
    }

    async fn restore_todo(&self, id: i64) -> Result<(), AppError> {
        let path = self
            .get_todo_path(id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("待办不存在: id={}", id)))?;

        let mut todo = self.read_todo_file(&path).await?;
        todo.deleted_at = None;
        todo.update_time = Some(Utc::now().naive_local());

        let uuid = todo.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
        let list_name = self.get_todo_list_name(todo.list_id);
        let new_path = self.todo_file_path(&uuid, &list_name, false);

        self.write_todo_file(&new_path, &todo).await?;
        if new_path != path {
            let _ = tokio::fs::remove_file(&path).await;
            self.index_todo(id, new_path);
        }

        self.git_commit(&format!("Restore todo: {}", todo.title));
        Ok(())
    }

    async fn permanent_delete_todo(&self, id: i64) -> Result<(), AppError> {
        if let Some(path) = self.get_todo_path(id) {
            let _ = tokio::fs::remove_file(&path).await;
            self.unindex_todo(id);
        }
        self.git_commit(&format!("Permanently delete todo: {}", id));
        Ok(())
    }

    async fn empty_todo_trash(&self) -> Result<u64, AppError> {
        let trash_dir = self.todos_dir().join("_trash");
        let mut count = 0u64;

        if let Ok(mut entries) = tokio::fs::read_dir(&trash_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    if let Ok(content) = tokio::fs::read_to_string(&path).await {
                        let (yaml, _) = Self::split_frontmatter(&content);
                        if let Some(y) = yaml {
                            if let Ok(fm) = yaml_serde::from_str::<TodoFrontmatter>(&y) {
                                self.unindex_todo(fm.id);
                            }
                        }
                    }
                    let _ = tokio::fs::remove_file(&path).await;
                    count += 1;
                }
            }
        }

        if count > 0 {
            self.git_commit(&format!("Empty todo trash: {} items", count));
        }
        Ok(count)
    }

    async fn find_all_todo_lists(&self) -> Result<Vec<TodoList>, AppError> {
        let idx = self.todo_list_index.read().unwrap();
        let mut lists: Vec<TodoList> = idx
            .values()
            .map(|m| TodoList {
                id: m.id,
                name: m.name.clone(),
                icon: m.icon.clone(),
                color: m.color.clone(),
                sort_order: m.sort_order,
                mcp_access: m.mcp_access,
                create_time: Self::string_to_opt_dt(&m.create_time),
                update_time: Self::string_to_opt_dt(&m.update_time),
            })
            .collect();
        lists.sort_by(|a, b| a.sort_order.cmp(&b.sort_order).then(a.id.cmp(&b.id)));
        Ok(lists)
    }

    async fn create_todo_list(&self, list: &TodoList) -> Result<TodoList, AppError> {
        let mut list = list.clone();
        if list.id == 0 {
            list.id = self.gen_id();
        }
        let now = Utc::now().naive_local();
        if list.create_time.is_none() {
            list.create_time = Some(now);
        }
        list.update_time = Some(now);

        let dir_name = Self::safe_dir_name(&list.name);
        let dir_path = self.todos_dir().join(&dir_name);
        tokio::fs::create_dir_all(&dir_path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("创建清单目录失败: {}", e)))?;

        let meta = TodoListMeta {
            id: list.id,
            name: list.name.clone(),
            icon: list.icon.clone(),
            color: list.color.clone(),
            sort_order: list.sort_order,
            mcp_access: list.mcp_access,
            create_time: Self::opt_dt_to_string(&list.create_time),
            update_time: Self::opt_dt_to_string(&list.update_time),
            dir_name,
        };

        let meta_path = dir_path.join(".todo-list.yml");
        let yaml = yaml_serde::to_string(&meta)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("序列化清单元数据失败: {}", e)))?;
        tokio::fs::write(&meta_path, yaml)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入清单元数据失败: {}", e)))?;

        {
            let mut idx = self.todo_list_index.write().unwrap();
            idx.insert(list.id, meta);
        }

        self.git_commit(&format!("Create todo list: {}", list.name));
        Ok(list)
    }

    async fn update_todo_list(&self, list: &TodoList) -> Result<Option<TodoList>, AppError> {
        let mut list = list.clone();
        list.update_time = Some(Utc::now().naive_local());

        let old_meta = {
            let idx = self.todo_list_index.read().unwrap();
            idx.get(&list.id).cloned()
        }
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("清单不存在: id={}", list.id)))?;

        let new_dir_name = Self::safe_dir_name(&list.name);
        let old_dir_path = self.todos_dir().join(&old_meta.dir_name);
        let new_dir_path = self.todos_dir().join(&new_dir_name);

        // 如果目录名变化，重命名目录
        if old_meta.dir_name != new_dir_name {
            if old_dir_path.exists() {
                tokio::fs::rename(&old_dir_path, &new_dir_path)
                    .await
                    .map_err(|e| {
                        AppError::Internal(anyhow::anyhow!("重命名清单目录失败: {}", e))
                    })?;
            } else {
                tokio::fs::create_dir_all(&new_dir_path)
                    .await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("创建目录失败: {}", e)))?;
            }

            // 更新 todo_index 路径
            {
                let mut idx = self.todo_index.write().unwrap();
                for path in idx.values_mut() {
                    if let Some(rel) = path.strip_prefix(&old_dir_path).ok() {
                        *path = new_dir_path.join(rel);
                    }
                }
            }
        }

        let meta = TodoListMeta {
            id: list.id,
            name: list.name.clone(),
            icon: list.icon.clone(),
            color: list.color.clone(),
            sort_order: list.sort_order,
            mcp_access: list.mcp_access,
            create_time: Self::opt_dt_to_string(&list.create_time),
            update_time: Self::opt_dt_to_string(&list.update_time),
            dir_name: new_dir_name,
        };

        let meta_path = new_dir_path.join(".todo-list.yml");
        let yaml = yaml_serde::to_string(&meta)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("序列化清单元数据失败: {}", e)))?;
        tokio::fs::write(&meta_path, yaml)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入清单元数据失败: {}", e)))?;

        {
            let mut idx = self.todo_list_index.write().unwrap();
            idx.insert(list.id, meta);
        }

        self.git_commit(&format!("Update todo list: {}", list.name));
        Ok(Some(list))
    }

    async fn delete_todo_list(&self, id: i64) -> Result<(), AppError> {
        let meta = {
            let idx = self.todo_list_index.read().unwrap();
            idx.get(&id).cloned()
        }
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("清单不存在: id={}", id)))?;

        let dir_path = self.todos_dir().join(&meta.dir_name);

        // 将该目录下的待办移入回收站或 _unlisted
        if let Ok(mut entries) = tokio::fs::read_dir(&dir_path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    if let Ok(mut todo) = self.read_todo_file(&path).await {
                        let uuid = todo.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
                        // 移入 _unlisted
                        todo.list_id = None;
                        todo.update_time = Some(Utc::now().naive_local());
                        let new_path = self.todo_file_path(&uuid, "", false);
                        self.write_todo_file(&new_path, &todo).await?;
                        self.index_todo(todo.id, new_path);
                        let _ = tokio::fs::remove_file(&path).await;
                    }
                }
            }
        }

        // 删除目录
        let _ = tokio::fs::remove_dir_all(&dir_path).await;

        {
            let mut idx = self.todo_list_index.write().unwrap();
            idx.remove(&id);
        }

        self.git_commit(&format!("Delete todo list: {}", meta.name));
        Ok(())
    }

    async fn reorder_todos(&self, orders: Vec<(i64, i32)>) -> Result<(), AppError> {
        for (id, sort_order) in &orders {
            if let Some(path) = self.get_todo_path(*id) {
                let mut todo = self.read_todo_file(&path).await?;
                todo.sort_order = *sort_order;
                todo.update_time = Some(Utc::now().naive_local());
                self.write_todo_file(&path, &todo).await?;
            }
        }
        self.git_commit("Reorder todos");
        Ok(())
    }

    async fn find_todo_tags(&self, todo_id: i64) -> Result<Vec<i64>, AppError> {
        let path = match self.get_todo_path(todo_id) {
            Some(p) => p,
            None => return Ok(Vec::new()),
        };

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取待办失败: {}", e)))?;
        let (yaml, _) = Self::split_frontmatter(&content);
        let fm: TodoFrontmatter = if let Some(y) = yaml {
            yaml_serde::from_str(&y).unwrap_or_default()
        } else {
            TodoFrontmatter::default()
        };

        Ok(fm.tags)
    }

    async fn set_todo_tags(&self, todo_id: i64, tag_ids: Vec<i64>) -> Result<(), AppError> {
        let path = self
            .get_todo_path(todo_id)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("待办不存在: id={}", todo_id)))?;

        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("读取待办失败: {}", e)))?;
        let (yaml, body) = Self::split_frontmatter(&content);
        let mut fm: TodoFrontmatter = if let Some(y) = yaml {
            yaml_serde::from_str(&y).unwrap_or_default()
        } else {
            TodoFrontmatter::default()
        };

        fm.tags = tag_ids;

        let new_yaml = yaml_serde::to_string(&fm)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("序列化失败: {}", e)))?;
        let new_content = Self::join_frontmatter(&new_yaml, &body);
        tokio::fs::write(&path, new_content)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("写入待办失败: {}", e)))?;

        self.git_commit(&format!("Set todo tags: {}", todo_id));
        Ok(())
    }

    async fn batch_toggle_todos(&self, ids: &[i64]) -> Result<u64, AppError> {
        let mut count = 0u64;
        for &id in ids {
            if self.toggle_todo(id).await?.is_some() {
                count += 1;
            }
        }
        Ok(count)
    }

    async fn batch_delete_todos(&self, ids: &[i64]) -> Result<u64, AppError> {
        let mut count = 0u64;
        for &id in ids {
            if self.delete_todo(id).await.is_ok() {
                count += 1;
            }
        }
        Ok(count)
    }

    async fn batch_move_todos(&self, ids: &[i64], list_id: Option<i64>) -> Result<u64, AppError> {
        let list_name = self.get_todo_list_name(list_id);
        let mut count = 0u64;

        for &id in ids {
            if let Some(path) = self.get_todo_path(id) {
                let mut todo = self.read_todo_file(&path).await?;
                let uuid = todo.uuid.clone().unwrap_or_else(|| Self::gen_uuid());
                todo.list_id = list_id;
                todo.update_time = Some(Utc::now().naive_local());

                let deleted = todo.deleted_at.is_some();
                let new_path = self.todo_file_path(&uuid, &list_name, deleted);
                self.write_todo_file(&new_path, &todo).await?;
                if new_path != path {
                    let _ = tokio::fs::remove_file(&path).await;
                    self.index_todo(id, new_path);
                }
                count += 1;
            }
        }

        if count > 0 {
            self.git_commit(&format!("Move {} todos", count));
        }
        Ok(count)
    }

    async fn todo_stats(&self) -> Result<TodoStats, AppError> {
        let todos = self.load_all_todos().await?;
        let active: Vec<&Todo> = todos.iter().filter(|t| t.deleted_at.is_none()).collect();

        let total = active.len() as u64;
        let completed = active.iter().filter(|t| t.is_completed == 1).count() as u64;
        let pending = total - completed;

        let today = Local::now().date_naive();
        let today_count = active
            .iter()
            .filter(|t| {
                t.is_completed == 0 && t.due_date.map(|d| d.date() == today).unwrap_or(false)
            })
            .count() as u64;

        let overdue_count = active
            .iter()
            .filter(|t| {
                t.is_completed == 0 && t.due_date.map(|d| d.date() < today).unwrap_or(false)
            })
            .count() as u64;

        let completion_rate = if total > 0 {
            (completed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Ok(TodoStats {
            total_count: total,
            completed_count: completed,
            pending_count: pending,
            today_count,
            overdue_count,
            completion_rate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ======== 测试辅助 ========

    /// 为每个测试创建独立的临时仓库目录
    fn temp_repo(name: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "enote-git-store-test-{}-{}-{n}",
            std::process::id(),
            name
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn cleanup(dir: &Path) {
        let _ = std::fs::remove_dir_all(dir);
    }

    /// 构造分页参数齐全的搜索条件
    ///
    /// PageParam 的 derive(Default) 是全 0（serde 默认值仅在反序列化时生效），
    /// 直接用 default() 会导致 page_size=0、分页截取结果为空。
    fn search_param() -> NoteSearchPageParam {
        NoteSearchPageParam {
            page_param: PageParam {
                page_index: 1,
                page_size: 100,
            },
            ..Default::default()
        }
    }

    /// 递归收集目录下所有 .md 文件
    fn collect_md_files(dir: &Path, out: &mut Vec<PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    collect_md_files(&p, out);
                } else if p.extension().map(|x| x == "md").unwrap_or(false) {
                    out.push(p);
                }
            }
        }
    }

    async fn make_notebook(store: &GitStore, name: &str) -> Notebook {
        store
            .create_notebook(&Notebook {
                name: name.to_string(),
                ..Default::default()
            })
            .await
            .unwrap()
            .unwrap()
    }

    async fn make_note(store: &GitStore, nb_id: i64, title: &str, content: &str) -> Note {
        store
            .create_note(&Note {
                notebook_id: nb_id,
                title: title.to_string(),
                content: content.to_string(),
                content_type: 1,
                ..Default::default()
            })
            .await
            .unwrap()
            .unwrap()
    }

    async fn make_todo(store: &GitStore, title: &str) -> Todo {
        store
            .create_todo(&Todo {
                title: title.to_string(),
                ..Default::default()
            })
            .await
            .unwrap()
    }

    // ======== 初始化 ========

    #[tokio::test]
    async fn init_creates_directory_structure() {
        let dir = temp_repo("init");
        let _store = GitStore::new(dir.clone(), None);

        for sub in [
            "notes",
            "notes/_unlisted",
            "notes/_trash",
            "todos",
            "todos/_unlisted",
            "todos/_trash",
            "templates",
            "attachments",
        ] {
            assert!(dir.join(sub).is_dir(), "missing dir: {sub}");
        }
        assert!(dir.join(".git").is_dir(), "git repo should be initialized");
        cleanup(&dir);
    }

    #[tokio::test]
    async fn git_commits_are_created() {
        let dir = temp_repo("git-commit");
        let store = GitStore::new(dir.clone(), None);
        let nb = make_notebook(&store, "G").await;
        make_note(&store, nb.id, "git", "body").await;
        drop(store);

        // git 不可用时跳过（git 操作在实现中静默失败，不影响功能）
        let out = match std::process::Command::new("git")
            .args(["rev-list", "--count", "HEAD"])
            .current_dir(&dir)
            .output()
        {
            Ok(o) if o.status.success() => o,
            _ => {
                cleanup(&dir);
                return;
            }
        };
        let count = String::from_utf8_lossy(&out.stdout)
            .trim()
            .parse::<u32>()
            .unwrap();
        assert!(count >= 2, "expect at least 2 commits, got {count}");
        cleanup(&dir);
    }

    #[tokio::test]
    async fn index_rebuild_persists_across_reopen() {
        let dir = temp_repo("persist");
        {
            let store = GitStore::new(dir.clone(), None);
            let nb = make_notebook(&store, "持久").await;
            make_note(&store, nb.id, "持久笔记", "data").await;
            make_todo(&store, "持久待办").await;
            store
                .create_tag(&Tag {
                    name: "tag1".to_string(),
                    ..Default::default()
                })
                .await
                .unwrap();
            store
                .create_template(&NoteTemplate {
                    name: "tpl".to_string(),
                    content: "c".to_string(),
                    ..Default::default()
                })
                .await
                .unwrap();
        }

        // 重新打开后索引重建，数据仍可读
        let store = GitStore::new(dir.clone(), None);
        assert_eq!(store.find_all_notebooks().await.unwrap().len(), 1);
        assert_eq!(
            store
                .search_page_notes(&search_param())
                .await
                .unwrap()
                .total,
            1
        );
        assert_eq!(
            store
                .search_todos(&TodoSearchParam::default())
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(store.find_all_tags().await.unwrap().len(), 1);
        assert_eq!(store.find_all_templates().await.unwrap().len(), 1);
        cleanup(&dir);
    }

    // ======== 笔记本 ========

    #[tokio::test]
    async fn notebook_crud_and_rename() {
        let dir = temp_repo("notebook");
        let store = GitStore::new(dir.clone(), None);

        let nb = make_notebook(&store, "工作").await;
        assert!(nb.id > 0);

        let all = store.find_all_notebooks().await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name, "工作");

        // 重命名
        store
            .update_notebook(&Notebook {
                id: nb.id,
                name: "工作台".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
        let all = store.find_all_notebooks().await.unwrap();
        assert_eq!(all[0].name, "工作台");

        // 删除
        store.delete_notebook_by_id(nb.id).await.unwrap();
        assert!(store.find_all_notebooks().await.unwrap().is_empty());
        cleanup(&dir);
    }

    // ======== 笔记 ========

    #[tokio::test]
    async fn note_crud_roundtrip() {
        let dir = temp_repo("note");
        let store = GitStore::new(dir.clone(), None);
        let nb = make_notebook(&store, "笔记").await;

        let note = make_note(&store, nb.id, "Hello", "# 正文\n内容").await;
        assert!(note.id > 0);
        assert!(note.uuid.is_some());
        assert!(note.create_time.is_some());
        assert_eq!(note.notebook_name, "笔记");

        let page = store.search_page_notes(&search_param()).await.unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.data[0].id, note.id);
        assert_eq!(page.data[0].title, "Hello");
        assert_eq!(page.data[0].content, "# 正文\n内容");

        // 富文本（HTML）内容原样保存
        let html = make_note(&store, nb.id, "HTML", "<p>富文本 <b>加粗</b></p>").await;
        assert_eq!(html.content_type, 1);
        let page = store.search_page_notes(&search_param()).await.unwrap();
        let loaded = page.data.iter().find(|n| n.id == html.id).unwrap();
        assert_eq!(loaded.content, "<p>富文本 <b>加粗</b></p>");

        // 更新
        let mut updated = note.clone();
        updated.title = "Hello2".to_string();
        updated.content = "new body".to_string();
        store.update_note(&updated).await.unwrap();
        let page = store.search_page_notes(&search_param()).await.unwrap();
        let loaded = page.data.iter().find(|n| n.id == note.id).unwrap();
        assert_eq!(loaded.title, "Hello2");
        assert_eq!(loaded.content, "new body");
        cleanup(&dir);
    }

    #[tokio::test]
    async fn note_file_format_uses_frontmatter() {
        let dir = temp_repo("format");
        let store = GitStore::new(dir.clone(), None);
        let nb = make_notebook(&store, "默认").await;
        let note = make_note(&store, nb.id, "格式", "body").await;

        let mut files = Vec::new();
        collect_md_files(&dir.join("notes"), &mut files);
        assert_eq!(files.len(), 1);

        let raw = std::fs::read_to_string(&files[0]).unwrap();
        assert!(raw.starts_with("---\n"), "file should use YAML frontmatter");
        assert!(
            raw.contains(note.uuid.as_deref().unwrap()),
            "frontmatter should contain uuid"
        );
        assert!(raw.contains("title:"));
        assert!(raw.trim_end().ends_with("body"), "body should be preserved");
        cleanup(&dir);
    }

    #[tokio::test]
    async fn note_search_keyword_filter_pagination() {
        let dir = temp_repo("search");
        let store = GitStore::new(dir.clone(), None);
        let nb = make_notebook(&store, "全部").await;
        make_note(&store, nb.id, "Rust 入门", "学习 rust").await;
        make_note(&store, nb.id, "Java 指南", "学习 java").await;
        make_note(&store, nb.id, "随想", "今天天气不错").await;

        // 关键词匹配标题（大小写不敏感）
        let param = NoteSearchPageParam {
            keyword: "rust".to_string(),
            ..search_param()
        };
        let page = store.search_page_notes(&param).await.unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.data[0].title, "Rust 入门");

        // 关键词匹配内容
        let param = NoteSearchPageParam {
            keyword: "JAVA".to_string(),
            ..search_param()
        };
        assert_eq!(store.search_page_notes(&param).await.unwrap().total, 1);

        // 按笔记本过滤
        let nb2 = make_notebook(&store, "其他").await;
        make_note(&store, nb2.id, "别的", "x").await;
        let param = NoteSearchPageParam {
            notebook_id: nb2.id,
            ..search_param()
        };
        let page = store.search_page_notes(&param).await.unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.data[0].notebook_id, nb2.id);

        // 分页
        let param = NoteSearchPageParam {
            page_param: PageParam {
                page_index: 1,
                page_size: 2,
                ..Default::default()
            },
            ..Default::default()
        };
        let page = store.search_page_notes(&param).await.unwrap();
        assert_eq!(page.total, 4);
        assert_eq!(page.data.len(), 2);
        cleanup(&dir);
    }

    #[tokio::test]
    async fn note_trash_flow() {
        let dir = temp_repo("trash");
        let store = GitStore::new(dir.clone(), None);
        let nb = make_notebook(&store, "工作").await;
        let n1 = make_note(&store, nb.id, "A", "a").await;
        let n2 = make_note(&store, nb.id, "B", "b").await;

        // 软删除后不在列表中
        store.delete_note_by_id(n1.id).await.unwrap();
        let page = store.search_page_notes(&search_param()).await.unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.data[0].title, "B");

        // 回收站可见
        let trash = store
            .find_deleted_notes(&PageParam {
                page_index: 1,
                page_size: 100,
            })
            .await
            .unwrap();
        assert_eq!(trash.total, 1);
        assert!(trash.data[0].deleted_at.is_some());

        // 恢复
        store.restore_note(n1.id).await.unwrap();
        assert_eq!(
            store
                .search_page_notes(&search_param())
                .await
                .unwrap()
                .total,
            2
        );

        // 彻底删除
        store.delete_note_by_id(n1.id).await.unwrap();
        store.permanent_delete_note(n1.id).await.unwrap();
        assert_eq!(
            store
                .find_deleted_notes(&PageParam {
                    page_index: 1,
                    page_size: 100,
                })
                .await
                .unwrap()
                .total,
            0
        );

        // 清空回收站
        store.delete_note_by_id(n2.id).await.unwrap();
        store.empty_trash().await.unwrap();
        assert_eq!(
            store
                .find_deleted_notes(&PageParam {
                    page_index: 1,
                    page_size: 100,
                })
                .await
                .unwrap()
                .total,
            0
        );
        cleanup(&dir);
    }

    #[tokio::test]
    async fn note_toggle_pin_star_and_stats() {
        let dir = temp_repo("toggle");
        let store = GitStore::new(dir.clone(), None);
        let nb = make_notebook(&store, "T").await;
        let n1 = make_note(&store, nb.id, "1", "a").await;
        let n2 = make_note(&store, nb.id, "2", "b").await;
        make_note(&store, nb.id, "3", "c").await;

        let starred = store.toggle_note_star(n1.id).await.unwrap().unwrap();
        assert_eq!(starred.is_starred, 1);
        let pinned = store.toggle_note_pin(n2.id).await.unwrap().unwrap();
        assert_eq!(pinned.is_pinned, 1);

        // 再 toggle 回来
        let unstarred = store.toggle_note_star(n1.id).await.unwrap().unwrap();
        assert_eq!(unstarred.is_starred, 0);

        // 统计
        let stats = store
            .note_stats(&NoteSearchPageParam::default())
            .await
            .unwrap();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.notebook_counts.get(&nb.id), Some(&3));
        cleanup(&dir);
    }

    #[tokio::test]
    async fn note_batch_move_and_delete() {
        let dir = temp_repo("batch");
        let store = GitStore::new(dir.clone(), None);
        let nb1 = make_notebook(&store, "甲").await;
        let nb2 = make_notebook(&store, "乙").await;
        let n1 = make_note(&store, nb1.id, "1", "x").await;
        let n2 = make_note(&store, nb1.id, "2", "y").await;
        let n3 = make_note(&store, nb1.id, "3", "z").await;

        store
            .batch_move_notes(&[n1.id, n2.id], nb2.id)
            .await
            .unwrap();
        let param = NoteSearchPageParam {
            notebook_id: nb2.id,
            ..search_param()
        };
        assert_eq!(store.search_page_notes(&param).await.unwrap().total, 2);

        store.batch_delete_notes(&[n1.id, n3.id]).await.unwrap();
        let page = store.search_page_notes(&search_param()).await.unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.data[0].id, n2.id);
        cleanup(&dir);
    }

    // ======== 标签 ========

    #[tokio::test]
    async fn tag_crud_and_note_tag_roundtrip() {
        let dir = temp_repo("tag");
        let store = GitStore::new(dir.clone(), None);

        let tag = store
            .create_tag(&Tag {
                name: "rust".to_string(),
                ..Default::default()
            })
            .await
            .unwrap()
            .unwrap();
        assert!(tag.id > 0);
        assert_eq!(store.find_all_tags().await.unwrap().len(), 1);

        // 笔记带标签 → 往返保留
        let nb = make_notebook(&store, "N").await;
        let mut note = Note {
            notebook_id: nb.id,
            title: "t".to_string(),
            content: "c".to_string(),
            content_type: 1,
            ..Default::default()
        };
        note.tags.push(Tag {
            id: tag.id,
            name: "rust".to_string(),
            ..Default::default()
        });
        store.create_note(&note).await.unwrap();

        let page = store.search_page_notes(&search_param()).await.unwrap();
        assert_eq!(page.data[0].tags.len(), 1);
        assert_eq!(page.data[0].tags[0].name, "rust");

        // 按标签过滤
        let param = NoteSearchPageParam {
            tag_id: tag.id,
            ..search_param()
        };
        assert_eq!(store.search_page_notes(&param).await.unwrap().total, 1);

        // 删除标签（同时从笔记中移除）
        store.delete_tag_by_id(tag.id).await.unwrap();
        assert!(store.find_all_tags().await.unwrap().is_empty());
        let page = store.search_page_notes(&search_param()).await.unwrap();
        assert!(page.data[0].tags.is_empty());
        cleanup(&dir);
    }

    // ======== 模板 ========

    #[tokio::test]
    async fn template_crud() {
        let dir = temp_repo("template");
        let store = GitStore::new(dir.clone(), None);

        let t = store
            .create_template(&NoteTemplate {
                name: "日记模板".to_string(),
                content: "# 日记\n".to_string(),
                content_type: 1,
                ..Default::default()
            })
            .await
            .unwrap()
            .unwrap();
        assert!(t.id > 0);
        assert!(t.uuid.is_some());

        let all = store.find_all_templates().await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name, "日记模板");
        assert_eq!(all[0].content, "# 日记\n");
        assert_eq!(all[0].content_type, 1);

        store
            .update_template(&NoteTemplate {
                id: t.id,
                uuid: t.uuid.clone(),
                name: "周报模板".to_string(),
                content: "## 本周".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
        let all = store.find_all_templates().await.unwrap();
        assert_eq!(all[0].name, "周报模板");
        assert_eq!(all[0].content, "## 本周");

        store.delete_template_by_id(t.id).await.unwrap();
        assert!(store.find_all_templates().await.unwrap().is_empty());
        cleanup(&dir);
    }

    // ======== 笔记链接 ========

    #[tokio::test]
    async fn note_link_flow() {
        let dir = temp_repo("link");
        let store = GitStore::new(dir.clone(), None);
        let nb = make_notebook(&store, "L").await;
        let n1 = make_note(&store, nb.id, "源笔记", "source").await;
        let n2 = make_note(&store, nb.id, "目标笔记", "target").await;
        make_note(&store, nb.id, "无关笔记", "other").await;

        store.create_note_link(n1.id, n2.id).await.unwrap();

        let links = store.find_note_links(n1.id).await.unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].note_id, n2.id);
        assert_eq!(links[0].note_title, "目标笔记");

        // 可链接笔记搜索：排除自己，按关键词过滤
        let result = store.search_linkable_notes(n1.id, "").await.unwrap();
        assert_eq!(result.len(), 2);
        assert!(!result.iter().any(|l| l.note_id == n1.id));
        let result = store.search_linkable_notes(n1.id, "目标").await.unwrap();
        assert!(result.iter().any(|l| l.note_id == n2.id));

        // 删除链接
        store.delete_note_link(links[0].id).await.unwrap();
        assert!(store.find_note_links(n1.id).await.unwrap().is_empty());
        cleanup(&dir);
    }

    // ======== 附件 ========

    #[tokio::test]
    async fn attachment_dedup_and_stats() {
        let dir = temp_repo("attach");
        let store = GitStore::new(dir.clone(), None);
        let nb = make_notebook(&store, "A").await;
        let n1 = make_note(&store, nb.id, "有附件", "body").await;

        let a1 = store
            .save_attachment(n1.id, "a.txt", b"hello world", "text/plain")
            .await
            .unwrap();
        assert_eq!(a1.file_size, 11);

        // 相同内容 → sha256 去重，同一物理文件
        let a2 = store
            .save_attachment(n1.id, "a-copy.txt", b"hello world", "text/plain")
            .await
            .unwrap();
        assert_eq!(a1.file_hash, a2.file_hash);

        let list = store.find_attachments(n1.id).await.unwrap();
        assert_eq!(list.len(), 2);

        let stats = store.get_attachment_stats().await.unwrap();
        assert_eq!(stats.total_count, 2);
        assert_eq!(stats.unique_file_count, 1);

        store.delete_attachment(a1.id).await.unwrap();
        assert_eq!(store.find_attachments(n1.id).await.unwrap().len(), 1);
        cleanup(&dir);
    }

    // ======== 加密 ========

    #[tokio::test]
    async fn encrypted_content_stored_and_searchable() {
        let dir = temp_repo("enc");
        let store = GitStore::new(dir.clone(), Some("test-enc-key".to_string()));
        let nb = make_notebook(&store, "机密").await;
        let note = make_note(&store, nb.id, "秘密", "绝密内容 secret-plans").await;

        // 磁盘上的文件 body 是密文
        let mut files = Vec::new();
        collect_md_files(&dir.join("notes"), &mut files);
        assert_eq!(files.len(), 1);
        let raw = std::fs::read_to_string(&files[0]).unwrap();
        let (_, body) = GitStore::split_frontmatter(&raw);
        assert!(
            crypto::is_encrypted(&body),
            "stored body should be encrypted"
        );

        // 搜索时按明文匹配（解密后检索）
        let param = NoteSearchPageParam {
            keyword: "secret-plans".to_string(),
            ..search_param()
        };
        let page = store.search_page_notes(&param).await.unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.data[0].id, note.id);
        cleanup(&dir);
    }

    // ======== 待办 ========

    #[tokio::test]
    async fn todo_crud_toggle_update_search() {
        let dir = temp_repo("todo");
        let store = GitStore::new(dir.clone(), None);

        let t1 = make_todo(&store, "买牛奶").await;
        assert!(t1.id > 0);
        assert!(t1.uuid.is_some());
        let t2 = make_todo(&store, "写周报").await;

        let found = store.find_todo(t1.id).await.unwrap().unwrap();
        assert_eq!(found.title, "买牛奶");

        // 关键字搜索
        let param = TodoSearchParam {
            keyword: "牛奶".to_string(),
            ..Default::default()
        };
        let todos = store.search_todos(&param).await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].id, t1.id);

        // toggle 完成 / 取消
        let done = store.toggle_todo(t1.id).await.unwrap().unwrap();
        assert_eq!(done.is_completed, 1);
        assert!(done.completed_at.is_some());
        let undone = store.toggle_todo(t1.id).await.unwrap().unwrap();
        assert_eq!(undone.is_completed, 0);
        assert!(undone.completed_at.is_none());

        // 更新
        let mut t = t1.clone();
        t.title = "买两瓶牛奶".to_string();
        t.priority = 2;
        store.update_todo(&t).await.unwrap();
        let found = store.find_todo(t1.id).await.unwrap().unwrap();
        assert_eq!(found.title, "买两瓶牛奶");
        assert_eq!(found.priority, 2);
        let _ = t2;
        cleanup(&dir);
    }

    #[tokio::test]
    async fn todo_list_crud_and_move() {
        let dir = temp_repo("todo-list");
        let store = GitStore::new(dir.clone(), None);

        let list = store
            .create_todo_list(&TodoList {
                name: "工作清单".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
        assert!(list.id > 0);
        assert_eq!(store.find_all_todo_lists().await.unwrap().len(), 1);

        store
            .create_todo(&Todo {
                title: "任务A".to_string(),
                list_id: Some(list.id),
                ..Default::default()
            })
            .await
            .unwrap();
        let t2 = make_todo(&store, "任务B").await;

        // 按清单过滤
        let param = TodoSearchParam {
            list_id: Some(list.id),
            ..Default::default()
        };
        let todos = store.search_todos(&param).await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "任务A");

        // 批量移动到清单
        let moved = store
            .batch_move_todos(&[t2.id], Some(list.id))
            .await
            .unwrap();
        assert_eq!(moved, 1);
        assert_eq!(store.search_todos(&param).await.unwrap().len(), 2);

        // 清单重命名
        store
            .update_todo_list(&TodoList {
                id: list.id,
                name: "工作2".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
        let lists = store.find_all_todo_lists().await.unwrap();
        assert_eq!(lists[0].name, "工作2");

        // 移出待办后删除清单
        let ids: Vec<i64> = store
            .search_todos(&param)
            .await
            .unwrap()
            .iter()
            .map(|t| t.id)
            .collect();
        store.batch_move_todos(&ids, None).await.unwrap();
        store.delete_todo_list(list.id).await.unwrap();
        assert!(store.find_all_todo_lists().await.unwrap().is_empty());
        cleanup(&dir);
    }

    #[tokio::test]
    async fn todo_tags_and_batch_ops_and_stats() {
        let dir = temp_repo("todo-batch");
        let store = GitStore::new(dir.clone(), None);
        let t1 = make_todo(&store, "A").await;
        let t2 = make_todo(&store, "B").await;
        let t3 = make_todo(&store, "C").await;

        // 待办标签（ID 列表全量替换）
        store.set_todo_tags(t1.id, vec![1, 2]).await.unwrap();
        assert_eq!(store.find_todo_tags(t1.id).await.unwrap(), vec![1, 2]);
        store.set_todo_tags(t1.id, vec![3]).await.unwrap();
        assert_eq!(store.find_todo_tags(t1.id).await.unwrap(), vec![3]);

        // 批量完成
        let n = store.batch_toggle_todos(&[t1.id, t2.id]).await.unwrap();
        assert_eq!(n, 2);
        assert_eq!(
            store.find_todo(t1.id).await.unwrap().unwrap().is_completed,
            1
        );
        assert_eq!(
            store.find_todo(t2.id).await.unwrap().unwrap().is_completed,
            1
        );

        // 统计
        let stats = store.todo_stats().await.unwrap();
        assert_eq!(stats.total_count, 3);
        assert_eq!(stats.completed_count, 2);
        assert_eq!(stats.pending_count, 1);

        // 批量删除
        let n = store.batch_delete_todos(&[t3.id]).await.unwrap();
        assert_eq!(n, 1);
        assert_eq!(store.todo_stats().await.unwrap().total_count, 2);
        cleanup(&dir);
    }

    #[tokio::test]
    async fn todo_trash_flow() {
        let dir = temp_repo("todo-trash");
        let store = GitStore::new(dir.clone(), None);
        let t1 = make_todo(&store, "A").await;
        let t2 = make_todo(&store, "B").await;

        // 软删除
        store.delete_todo(t1.id).await.unwrap();
        assert_eq!(
            store
                .search_todos(&TodoSearchParam::default())
                .await
                .unwrap()
                .len(),
            1
        );

        // 回收站可见
        let param = TodoSearchParam {
            deleted: true,
            ..Default::default()
        };
        let trashed = store.search_todos(&param).await.unwrap();
        assert_eq!(trashed.len(), 1);
        assert!(trashed[0].deleted_at.is_some());

        // 恢复
        store.restore_todo(t1.id).await.unwrap();
        assert_eq!(
            store
                .search_todos(&TodoSearchParam::default())
                .await
                .unwrap()
                .len(),
            2
        );

        // 永久删除
        store.delete_todo(t1.id).await.unwrap();
        store.permanent_delete_todo(t1.id).await.unwrap();
        let param = TodoSearchParam {
            deleted: true,
            ..Default::default()
        };
        assert!(store.search_todos(&param).await.unwrap().is_empty());

        // 清空回收站
        store.delete_todo(t2.id).await.unwrap();
        assert_eq!(store.empty_todo_trash().await.unwrap(), 1);
        cleanup(&dir);
    }
}
