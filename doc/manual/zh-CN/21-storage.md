## 21. 数据存储与配置

### 21.1 本地数据存储

所有笔记数据默认存储在本地 SQLite 数据库中，数据完全保存在用户本地设备上，不经过任何网络传输，保障数据隐私安全。

应用配置文件存储在操作系统的应用数据目录中。使用默认 SQLite 数据库时，数据库文件也位于该目录下；使用 MySQL 或 PostgreSQL 时，数据存储在对应的数据库服务器上，该目录下仅保存配置文件。不同操作系统的应用数据目录路径如下：

| 操作系统 | 应用数据目录 | 示例 |
|----------|-------------|------|
| **Windows** | `{FOLDERID_RoamingAppData}\net.easycloudcn.enote\` | `C:\Users\Alice\AppData\Roaming\net.easycloudcn.enote\` |
| **macOS** | `$HOME/Library/Application Support/net.easycloudcn.enote/` | `/Users/Alice/Library/Application Support/net.easycloudcn.enote/` |
| **Linux** | `$XDG_DATA_HOME/net.easycloudcn.enote/` 或 `$HOME/.local/share/net.easycloudcn.enote/` | `/home/alice/.local/share/net.easycloudcn.enote/` |

> **快速访问：**
> - **Windows：** 在文件资源管理器地址栏输入 `%APPDATA%\net.easycloudcn.enote` 即可快速打开。
> - **macOS：** 在 Finder 中按 `Cmd+Shift+G`，输入 `~/Library/Application Support/net.easycloudcn.enote` 即可打开。
> - **Linux：** 在终端中执行 `xdg-open "${XDG_DATA_HOME:-$HOME/.local/share}/net.easycloudcn.enote"` 即可打开。

该目录下包含以下文件和子目录：

| 文件/目录 | 说明 |
|-----------|------|
| `profiles.yml` | Profile 索引文件，记录当前活跃的配置和自动连接设置 |
| `profiles/` | Profile 配置目录，每个数据库配置对应一个 YAML 文件 |
| `application.yml` | 旧版配置文件（兼容模式，使用 `--config` 参数时使用） |
| `enote.db` | SQLite 数据库文件，仅使用 SQLite 配置时存在 |
| `enote.db-wal` | SQLite WAL 日志文件（自动生成），仅 SQLite 时存在 |
| `enote.db-shm` | SQLite 共享内存文件（自动生成），仅 SQLite 时存在 |
| `images/` | 图片本地存储目录，保存编辑器中插入的图片文件 |
| `backups/` | 自动备份目录，保存定时备份的 SQL 文件 |

### 21.2 配置文件

应用配置文件为 `application.yml`，采用 YAML 格式，默认位于上述应用数据目录中。首次启动时系统会自动生成包含 SQLite 默认配置的文件，用户无需手动创建。也可通过命令行参数 `--config` 指定自定义配置文件路径（详见 [19.1 命令行参数](19-startup.md#191-命令行参数)）。

**默认生成的配置文件内容如下：**

```yaml
# ENote 应用配置文件
# 此文件由应用自动生成

datasource:
  # SQLite 数据库连接 URL
  # mode=rwc 表示读写模式，如果文件不存在则创建
  url: "sqlite:/path/to/net.easycloudcn.enote/enote.db?mode=rwc"

  # 连接池配置
  # 最大连接数：SQLite 推荐 5 以内
  max-connections: 5
  # 最小连接数：保持常驻连接减少重连开销
  min-connections: 1
  # 连接超时（秒）：建立新连接的超时时间
  connect_timeout: 10
  # 获取连接超时（秒）：从池中获取连接的超时时间
  acquire-timeout: 5
  # 空闲超时（秒）：空闲连接保持时间
  idle-time: 300
  # 最大生命周期（秒）：连接最长存活时间
  max-lifetime: 1800

# MCP Server 配置
# 启用后可通过 AI 工具（如 Claude Desktop）操作笔记
mcp:
  enabled: false
```

> **注意：**
> - 配置文件中 `url` 字段的实际路径会根据操作系统自动填入完整的绝对路径。SQLite URL 中支持使用 `~` 表示用户主目录（如 `sqlite://~/data/enote.db?mode=rwc`），系统会自动展开为完整路径。
> - 修改配置文件后需**重启应用**使配置生效。

### 21.3 配置项说明

以下为 `datasource` 节点下所有可配置项的详细说明：

| 配置项 | 类型 | 默认值 | 有效范围 | 说明 |
|--------|------|--------|----------|------|
| `url` | 字符串 | （自动生成） | — | **必填**。数据库连接 URL，格式取决于数据库类型（见 21.4 节） |
| `max-connections` | 整数 | SQLite: 5，其他: 20 | 1 - 1000 | 连接池最大连接数。SQLite 建议不超过 5 |
| `min-connections` | 整数 | 1 | 0 - max-connections | 连接池最小保持连接数，设为 0 表示不保持常驻连接 |
| `connect_timeout` | 整数 | 10 | 1 - 300 | 建立新数据库连接的超时时间（秒） |
| `acquire-timeout` | 整数 | 5 | 1 - 300 | 从连接池获取可用连接的超时时间（秒） |
| `idle-time` | 整数 | 300 | 1 - 86400 | 空闲连接保持时间（秒），超过此时间未使用的连接将被回收 |
| `max-lifetime` | 整数 | 1800 | 1 - 86400 | 连接最大存活时间（秒），超过此时间的连接将被强制关闭并重建 |

以下为 `mcp` 节点下的配置项：

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `enabled` | 布尔值 | false | MCP Server 总开关。设为 true 后 MCP Server 才会接受连接 |

### 21.4 数据库配置示例

ENote 支持三种数据库，通过修改 `datasource.url` 即可切换。除默认的 SQLite 外，还支持 MySQL 和 PostgreSQL，满足高级用户的数据管理需求。

**SQLite（默认）：**

```yaml
datasource:
  url: "sqlite:/path/to/enote.db?mode=rwc"
  max-connections: 5
  min-connections: 1
```

- `mode=rwc`：读写模式，数据库文件不存在时自动创建。
- SQLite 为单文件数据库，无需安装额外服务，适合个人使用。

**MySQL：**

```yaml
datasource:
  url: "mysql://用户名:密码@服务器地址:3306/数据库名"
  max-connections: 20
  min-connections: 2
  connect_timeout: 10
  acquire-timeout: 5
  idle-time: 300
  max-lifetime: 1800
```

- 需预先安装 MySQL 5.7+ 或 MariaDB 10.3+ 并创建目标数据库。
- 示例：`url: "mysql://root:password@127.0.0.1:3306/enote"`

**PostgreSQL：**

```yaml
datasource:
  url: "postgres://用户名:密码@服务器地址:5432/数据库名"
  max-connections: 20
  min-connections: 2
  connect_timeout: 10
  acquire-timeout: 5
  idle-time: 300
  max-lifetime: 1800
```

- 需预先安装 PostgreSQL 12+ 并创建目标数据库。
- 示例：`url: "postgres://enote:password@127.0.0.1:5432/enote"`

> **注意：** 切换数据库类型后，原有数据库中的数据不会自动迁移。如需保留数据，请先使用导出功能备份笔记，切换后再导入。首次连接新数据库时，系统会自动创建所需的表结构。

### 21.5 Profile 配置管理

ENote 使用 Profile 模式管理多个数据库配置，配置文件存储在应用数据目录的 `profiles/` 子目录中。

**索引文件 `profiles.yml`：**

```yaml
active: local-sqlite    # 当前活跃的 Profile ID
auto-connect: false      # 是否自动连接上次使用的配置
```

**单个 Profile 配置文件示例（`profiles/local-sqlite.yml`）：**

```yaml
name: 本地笔记
icon: ''
datasource:
  type: sqlite
  path: /path/to/enote.db
security:
  content-encryption: false
```

**MySQL Profile 示例（`profiles/work-mysql.yml`）：**

```yaml
name: 工作数据库
icon: ''
datasource:
  type: mysql
  host: 192.168.1.100
  port: 3306
  database: enote
  username: enote_user
  auth-method: password
  ssl:
    mode: ''
security:
  content-encryption: true
```

**ENote Server Profile 示例（`profiles/company-server.yml`）：**

```yaml
name: 公司服务器
icon: ''
backend: server
server:
  url: https://enote.company.com
  auth-method:
    type: bearer
  timeout: 30
datasource:
  type: sqlite
security:
  content-encryption: false
```

**ENote Server Profile 示例 — JWT 认证（`profiles/cloud-jwt.yml`）：**

```yaml
name: 云端笔记
icon: ''
backend: server
server:
  url: https://api.enote-cloud.com
  auth-method:
    type: jwt
    refresh-url: /auth/refresh
    username: user@example.com
  timeout: 30
datasource:
  type: sqlite
security:
  content-encryption: false
```

**ENote Server Profile 示例 — OAuth 2.0（`profiles/sso-oauth.yml`）：**

```yaml
name: 企业 SSO
icon: ''
backend: server
server:
  url: https://enote.company.com
  auth-method:
    type: oauth2
    token-url: https://sso.company.com/oauth/token
    client-id: enote-desktop
    scopes: notes.read notes.write
  timeout: 30
datasource:
  type: sqlite
security:
  content-encryption: false
```

> **注意：**
> - 数据库密码、加密密钥和服务器认证信息（Token、密码、Client Secret 等）均不保存在配置文件中，而是安全存储在操作系统钥匙串中。
> - `backend` 字段为 `"server"` 时表示使用远程服务器后端，省略或为 `"database"` 时使用数据库后端。
> - Server 后端的 `datasource` 字段保留默认值即可，不会被使用。

### 21.5.1 Server 后端功能说明

使用 ENote Server 后端时，以下功能的行为与数据库后端有所不同：

| 功能 | 数据库后端 | Server 后端 |
|------|-----------|------------|
| 笔记 CRUD | 本地数据库操作 | 通过 HTTP API 调用服务端 |
| 全文搜索 | 本地 FTS 索引 | 由服务端实现 |
| 内容加密 | 客户端 AES-256 加密 | 由服务端处理 |
| 数据备份 | 支持（SQL/Excel/CSV） | 不支持（由服务端管理） |
| 跨 Profile 同步 | 支持 | 不支持 |
| 自动备份 | 支持 | 不支持 |
| 操作日志 | 本地记录 | 通过 API 记录到服务端 |

### 21.6 内容加密与密钥管理

启用内容加密后，系统使用 AES-256-GCM 算法对笔记内容进行加密保护。

**密钥存储：**

加密密钥存储在操作系统原生安全存储中：

| 操作系统 | 存储位置 | 管理方式 |
|----------|----------|----------|
| **macOS** | Keychain（钥匙串） | 可通过「钥匙串访问」应用查看 |
| **Windows** | Credential Store（凭据管理器） | 可通过「控制面板 > 凭据管理器」查看 |
| **Linux** | Secret Service（如 GNOME Keyring） | 可通过 `seahorse` 等工具查看 |

密钥条目使用应用标识符 `net.easycloudcn.enote` 作为服务名，格式为 `{profile_id}.encryption_key`。

**加密行为：**

- 笔记内容在保存到数据库前自动加密，读取时自动解密。
- 加密对用户完全透明，无需手动操作。
- 笔记标题**不加密**，仍可用于搜索。
- 笔记内容在数据库中以密文存储，即使直接打开数据库文件也无法读取原文。
- 笔记历史记录中的内容同样受加密保护。

### 21.7 SSL/TLS 证书认证

使用 MySQL 或 PostgreSQL 时，可通过 SSL/TLS 证书进行安全认证，保护数据传输安全。

**MySQL SSL 模式：**

| 模式 | 说明 |
|------|------|
| `disabled` | 不使用 SSL |
| `preferred` | 优先使用 SSL，不可用时回退 |
| `required` | 必须使用 SSL |
| `verify-ca` | 验证服务器 CA 证书 |
| `verify-identity` | 验证服务器身份（最严格） |

**PostgreSQL SSL 模式：**

| 模式 | 说明 |
|------|------|
| `disabled` | 不使用 SSL |
| `preferred` | 优先使用 SSL |
| `required` | 必须使用 SSL |
| `verify-ca` | 验证服务器 CA 证书 |
| `verify-full` | 完全验证（最严格） |

**所需证书文件（PEM 格式）：**

- **CA 证书：** 服务器 CA 根证书（`.pem` 或 `.crt`）
- **客户端证书：** 客户端身份证书（`.pem` 或 `.crt`）
- **客户端私钥：** 客户端私钥文件（`.pem` 或 `.key`）

证书文件路径在设置向导中通过文件选择器指定，保存在 Profile 配置文件中。

### 21.8 历史版本保护

系统自动记录笔记的每一次修改和删除操作的历史版本，包括操作前后的完整内容，支持随时追溯和对比，有效防止数据意外丢失。历史记录包含以下信息：

- 操作类型（新增、修改、删除）
- 操作来源（用户操作、AI 工具）
- 操作前的旧内容
- 操作后的新内容
- 操作发生时间
- 关联的笔记本和标签信息
