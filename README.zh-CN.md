# ENote

[English](README.md) | 中文

一个基于 Tauri 的跨平台桌面笔记应用，支持富文本编辑和 Markdown 模式。

## 功能特性

### 编辑器
- **富文本编辑** - 基于 TipTap 的所见即所得编辑器，支持多种格式
- **Markdown 模式** - 可切换为 Markdown 纯文本编辑模式，支持分屏预览
- **查找替换** - 编辑器内全局查找与替换
- **数学公式** - KaTeX 数学公式渲染
- **代码高亮** - 基于 lowlight 的代码块语法高亮
- **目录大纲** - 自动生成文档目录结构
- **拖拽排版** - 拖拽手柄调整内容块顺序
- **智能粘贴** - 上下文感知的粘贴格式处理
- **图片懒加载** - 图片延迟加载优化性能

### 笔记管理
- **笔记本管理** - 按笔记本分类组织笔记，拖拽排序
- **标签系统** - 灵活的标签管理，支持多标签筛选
- **笔记置顶** - 重要笔记置顶显示
- **历史版本** - 自动保存笔记历史，支持来源追踪（用户/同步/导入）
- **全文搜索** - FTS5 全文搜索，支持中文子串匹配
- **笔记模板** - 模板管理，快速创建笔记
- **双向链接** - 笔记间建立关联引用
- **回收站** - 软删除，支持恢复已删除笔记
- **命令面板** - Ctrl+P 快速执行操作
- **自定义快捷键** - 可配置的键盘快捷键

### 安全与加密
- **内容加密** - AES-256-GCM 透明加密笔记内容，密钥安全存储在系统钥匙串
- **笔记加密** - 单篇笔记密码保护
- **锁屏安全** - 密码保护（Argon2id）、超时锁定、最小化锁定
- **钥匙串集成** - 密码和密钥存储在 OS 钥匙串（macOS Keychain / Windows Credential Store / Linux Secret Service）

### 数据管理
- **多配置管理** - 支持多个数据库配置（Profile），启动时选择或自动连接
- **设置向导** - 首次启动引导配置数据库，支持 SQLite/MySQL/PostgreSQL
- **SSL/TLS 认证** - MySQL/PostgreSQL 支持证书登录
- **数据备份** - SQL/Excel/CSV 导出导入，自动定时备份
- **导入导出** - 支持印象笔记、有道笔记、Notion 导入，Word/Markdown/JSON/XML 导出
- **数据同步** - 同步基础设施，支持同步日志和详细追踪
- **图片本地存储** - 图片保存到本地，通过 Tauri asset 协议访问

### AI 集成
- **MCP 集成** - AI 工具通过 MCP 协议操作笔记，三层访问控制（笔记/标签/笔记本级别）

### 界面与体验
- **深色模式** - 浅色/深色/跟随系统主题切换
- **多语言** - 支持中文简体和英文
- **系统托盘** - 最小化到托盘运行
- **多窗口** - 支持在新窗口中打开笔记
- **帮助系统** - 内置可搜索的帮助文档
- **应用日志** - 审计日志，自动脱敏敏感数据
- **跨平台** - 支持 Windows、macOS 和 Linux

## 技术栈

### 前端
- **Vue 3** - 使用 Composition API 和 `<script setup>` 语法
- **TypeScript** - 类型安全的开发体验
- **Vite** - 快速的开发构建工具
- **Tailwind CSS v4** - 原子化 CSS 框架
- **TipTap** - 可扩展的富文本编辑器（8 个自定义扩展）
- **Pinia** - 状态管理
- **Vue I18n** - 国际化
- **Lucide Icons** - 图标库

### 后端
- **Rust**（Edition 2024）- 高性能系统编程语言
- **Tauri 2.x** - 轻量级跨平台桌面应用框架
- **SeaORM** - 异步 ORM 框架
- **SQLite/MySQL/PostgreSQL** - 多数据库支持
- **aes-gcm** - AES-256-GCM 加密
- **argon2** - 密码哈希
- **keyring** - 系统钥匙串集成
- **rmcp** - MCP 协议服务器

## 项目结构

```
enote/
├── src/                        # 前端源码
│   ├── api/                    # Tauri IPC 调用封装
│   ├── components/             # Vue 组件（60+ 组件）
│   │   ├── toolbar/            # 编辑器工具栏组件（17 个）
│   │   ├── ui/                 # 通用 UI 组件库（16 个）
│   │   ├── NoteEditor.vue      # 编辑器容器
│   │   ├── TipTapEditor.vue    # TipTap 富文本编辑器
│   │   ├── MarkdownSplitEditor.vue  # Markdown 分屏编辑器
│   │   ├── AppSidebar.vue      # 侧边栏（笔记本/标签）
│   │   ├── NoteList.vue        # 笔记列表
│   │   ├── LockScreen.vue      # 锁屏界面
│   │   ├── SetupWizard.vue     # 设置向导
│   │   ├── ProfileSelector.vue # 配置选择器
│   │   ├── CommandPalette.vue  # 命令面板
│   │   ├── HelpManual.vue      # 帮助文档
│   │   └── LogDialog.vue       # 应用日志
│   ├── composables/            # 组合式函数（11 个）
│   ├── extensions/             # TipTap 自定义扩展（8 个）
│   ├── utils/                  # 工具函数
│   │   └── import/             # 导入解析器（印象笔记/有道/Notion）
│   ├── stores/                 # Pinia 状态管理
│   ├── i18n/                   # 国际化（中/英）
│   ├── types/                  # TypeScript 类型定义
│   └── App.vue                 # 应用根组件
├── src-tauri/                  # Tauri 后端源码
│   └── src/
│       ├── command.rs          # IPC 命令处理
│       ├── service/            # 业务逻辑层（17 个服务）
│       ├── entity/             # 数据库实体（12 个）
│       ├── migration/          # 数据库迁移（22 个）
│       ├── config.rs           # 配置管理
│       ├── error.rs            # 错误处理
│       ├── model.rs            # 数据传输对象
│       └── i18n.rs             # 后端国际化
├── crates/
│   └── enote-mcp/              # MCP 服务器（独立二进制）
└── doc/                        # 文档和配置示例
```

## UI 组件库

项目包含一套自定义的 UI 组件：

- **BaseButton** - 按钮组件，支持多种类型和状态
- **BaseInput** - 输入框组件
- **BaseSelect** - 下拉选择组件
- **BaseDialog** - 对话框组件
- **BaseDropdown** - 下拉菜单组件
- **BasePagination** - 分页组件
- **BaseTooltip** - 提示框组件
- **BaseSkeleton** - 骨架屏加载组件
- **ColorPicker** - 颜色选择器
- **IconPicker** - 图标选择器
- **StylePicker** - 样式选择器
- **ConfirmDialog** - 确认对话框
- **ShortcutRecorder** - 快捷键录制器
- **Notification** - 通知提示

## 开发指南

### 环境要求

- Node.js 18+
- pnpm 10+
- Rust 1.85+（Edition 2024）
- Tauri CLI

### 安装依赖

```bash
# 安装前端依赖
pnpm install

# 安装 Tauri CLI（如未安装）
cargo install tauri-cli
```

### 开发模式

```bash
# 启动开发服务器（前端 + Tauri）
pnpm tauri dev

# 仅启动前端开发服务器（端口 1420）
pnpm dev
```

### 构建发布

```bash
# 构建生产版本
pnpm build

# 生成应用图标
pnpm tauri:icon
```

### 代码格式化与检查

```bash
# 格式化代码
pnpm format

# ESLint 检查
pnpm lint

# ESLint 自动修复
pnpm lint:fix

# 运行测试
pnpm test
```

## 数据库配置

支持 SQLite（默认）、MySQL 和 PostgreSQL。

**首次启动**时，设置向导将引导您完成数据库连接配置：

- **SQLite** - 本地文件数据库，开箱即用，无需安装额外服务
- **MySQL** - 支持密码登录和 SSL 证书认证
- **PostgreSQL** - 支持密码登录和 SSL 证书认证

配置以 Profile 形式管理，支持多个数据库配置随时切换。数据库密码和加密密钥安全存储在操作系统钥匙串中（macOS Keychain / Windows Credential Store / Linux Secret Service），不保存在配置文件里。

也可通过命令行参数使用传统配置文件：

```bash
enote --config /path/to/application.yml
```

配置文件示例参见 `doc/application.yml`、`doc/application-mysql.yml`、`doc/application-posgres.yml`。

## MCP 集成

ENote 内置 MCP（Model Context Protocol）服务器，允许 AI 工具通过标准化协议操作笔记。

### 三层访问控制

| 层级 | 范围 | 说明 |
|------|------|------|
| 笔记级别 | 单篇笔记 | 最高优先级 |
| 标签级别 | 按标签 | 多标签取最严格 |
| 笔记本级别 | 按笔记本 | 最低优先级 |

### 访问权限

- **继承** - 从上层继承（默认）
- **读写** - AI 可读可写
- **只读** - AI 仅可读取
- **拒绝** - 完全禁止 AI 访问
- 加密笔记始终强制拒绝 AI 访问

## 贡献指南

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/your-feature`)
3. 提交更改 (`git commit -m 'Add some feature'`)
4. 推送到分支 (`git push origin feature/your-feature`)
5. 创建 Pull Request

## 许可证

MIT License

## 相关链接

- [Tauri 官方文档](https://tauri.app)
- [Vue 3 文档](https://vuejs.org)
- [TipTap 文档](https://tiptap.dev)
- [Tailwind CSS 文档](https://tailwindcss.com)
- [SeaORM 文档](https://www.sea-ql.org/SeaORM)
