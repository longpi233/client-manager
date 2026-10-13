# 维康诊所管理系统

<p align="center">
  <img src="./src-tauri/icons/128x128.png" alt="Logo" width="96" height="96" />
</p>

<p align="center">
  <b>面向基层门诊、中医馆、针灸理疗中心的一站式现代化桌面管理系统</b>
</p>

<p align="center">
  <a href="https://tauri.app/"><img src="https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri&amp;logoColor=white" alt="Tauri 官网" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-DEA584?logo=rust&amp;logoColor=white" alt="Rust 官网" /></a>
  <a href="https://vuejs.org/"><img src="https://img.shields.io/badge/Vue-3-4FC08D?logo=vuedotjs&amp;logoColor=white" alt="Vue 官网" /></a>
  <a href="https://www.typescriptlang.org/"><img src="https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&amp;logoColor=white" alt="TypeScript 官网" /></a>
  <a href="https://vite.dev/"><img src="https://img.shields.io/badge/Vite-646CFF?logo=vite&amp;logoColor=white" alt="Vite 官网" /></a>
  <a href="https://element-plus.org/"><img src="https://img.shields.io/badge/Element_Plus-409EFF?logo=elementplus&amp;logoColor=white" alt="Element Plus 官网" /></a>
  <a href="https://www.sqlite.org/"><img src="https://img.shields.io/badge/SQLite-003B57?logo=sqlite&amp;logoColor=white" alt="SQLite 官网" /></a>
</p>

---

Client Manager 是一个用于记录诊所日常就诊、针灸理疗和中药处方的桌面应用，使用 Vue 3、TypeScript、Tauri 2 和 SQLite 开发。

业务数据保存在本机，支持记录查询、门诊统计、打印和备份。项目以单机使用为主，不包含独立的后端服务或多端数据同步服务。

## 功能

- **就诊登记**：记录患者信息、就诊时间、治疗项目、费用和结算类型。
- **针灸理疗**：录入病历和治疗方案，选择理疗项目，生成病历、治疗处方及费用清单。
- **中药处方**：按名称或拼音检索饮片，录入用量、单价和付数，计算费用并打印处方。
- **查询与统计**：按条件查询就诊记录，按日、月、年或自定义日期范围查看门诊日志，导出 Excel 表格。
- **基础信息维护**：管理机构信息、医师、中药饮片和理疗项目。
- **数据管理**：创建和恢复数据库备份，查看数据统计，压缩数据库，导入、导出 JSON 数据。

## 本地开发

### 环境要求

- Node.js 22.x 或 24.x
- pnpm
- Rust stable 工具链
- 对应操作系统的 Tauri 2 构建依赖；Windows 需要 Visual Studio C++ 构建工具和 Microsoft Edge WebView2 Runtime

### 安装依赖

```bash
git clone https://github.com/longpi233/client-manager.git
cd client-manager
pnpm install
```

### 启动桌面应用

在项目根目录执行：

```bash
pnpm exec tauri dev
```

Tauri 会自动启动 Vite 开发服务器并编译桌面应用。首次运行需要下载和编译 Rust 依赖，耗时取决于网络和本机环境。

项目的前端依赖中已包含 Tauri CLI，无需单独安装。若本机已安装 Tauri 2 的 Cargo CLI，也可以使用 `cargo tauri dev`。

### 仅调试前端

```bash
pnpm dev
```

开发服务器默认地址为 `http://localhost:3006`。浏览器模式适合调试页面和样式；数据库、备份等功能需要在 Tauri 桌面窗口中使用。

## 构建

生成桌面应用和安装包：

```bash
pnpm exec tauri build
```

该命令会先执行前端类型检查和构建，再编译 Rust 程序。安装包默认输出到 `src-tauri/target/release/bundle/`，格式取决于构建平台。

仅构建前端：

```bash
pnpm build
```

前端产物位于 `dist/`。它是桌面应用的界面资源，不是可以独立使用数据库功能的 Web 服务。

在 Windows x64 上，可使用 PowerShell 7 一次生成 EXE 安装包、MSI 安装包和免安装 ZIP：

```powershell
pwsh -NoProfile -File scripts/build-release.ps1
```

发布文件、SHA-256 校验和及构建日志会保存在 `artifacts/release-<版本>/`。该目录不会提交到 Git；打包脚本不执行安装、签名或 GitHub 上传。免安装 ZIP 需完整解压，并安装 Microsoft Edge WebView2 Runtime 后运行其中的 EXE；请保留随包附带的许可证。生成的安装包未签名，发布前请在干净的 Windows 环境验证安装、启动和卸载。

## 数据与备份

数据库文件名为 `health_clinic.db`，首次启动时自动创建在系统的应用数据目录中，不存放在项目仓库内。可通过应用的「数据管理」页面打开数据库文件所在位置和备份目录。

- 需要完整备份或迁移时，使用数据库备份功能，并将备份复制到另一块磁盘或其他安全位置。
- 恢复备份会覆盖当前数据库；导入 JSON 也可能替换已有记录。操作前请先创建备份，并在操作后核对数据。
- 数据库、备份和导出文件没有加密。涉及患者信息时，请自行限制文件访问权限，不要将真实数据上传到公开仓库或 Issue。

## 项目结构

前端通过 Tauri 的 `invoke` 接口调用 Rust 业务模块，Rust 负责 SQLite 数据读写和文件操作。

```text
client-manager/
├── src/
│   ├── api/                 # Tauri 命令调用封装
│   ├── components/          # 通用组件和业务组件
│   ├── router/              # 路由与菜单
│   ├── store/               # 前端状态管理
│   ├── types/               # TypeScript 类型
│   ├── utils/               # 打印、拼音检索等工具
│   └── views/               # 业务页面
├── src-tauri/
│   ├── src/modules/         # 数据库、就诊记录、基础信息及备份模块
│   ├── src/lib.rs           # 应用初始化与命令注册
│   ├── Cargo.toml           # Rust 依赖
│   └── tauri.conf.json      # 桌面应用与打包配置
├── package.json             # 前端依赖与开发脚本
├── pnpm-lock.yaml           # 前端依赖锁文件
└── vite.config.ts           # 前端构建配置
```

## 参与贡献

欢迎通过 [Issue](https://github.com/longpi233/client-manager/issues) 报告问题或提出建议，也欢迎提交 Pull Request。

报告问题时，请附上操作系统、应用版本、复现步骤和相关日志。截图、日志及示例数据请先脱敏。

提交修改前，可按涉及的代码运行检查：

```bash
# 前端类型检查与代码检查
pnpm exec vue-tsc --noEmit
pnpm exec eslint src

# Rust 格式与编译检查
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo check --manifest-path src-tauri/Cargo.toml --locked
```

请在 Pull Request 中说明修改目的和验证方式；涉及数据库结构或导入、恢复逻辑的变更，应说明对现有数据的影响。

## 致谢

前端界面基于 [Art Design Pro](https://github.com/Daymychen/art-design-pro) 开发，感谢原项目及其贡献者。

## 许可证

本项目采用 [MIT License](./LICENSE)。
