# AI Skills Manager

一个中心化的 AI IDE skills 管理器，解决配置碎片化问题。通过强大的软链接同步机制，实现一处编写，多处使用。

## 功能特性

- **Skills 管理**：统一管理本地 skills，支持创建、编辑、删除、搜索、批量管理
- **工具检测**：自动检测已安装的 AI IDE 工具（Claude Code、Codex、OpenCode、Cursor、Trae）
- **软链接同步**：编辑 skill 后自动同步到所有启用的工具
- **市场集成**：从 awesome-claude-skills 和 skills.sh 安装 skills
- **项目绑定**：支持将 skill 绑定到特定项目
- **主题支持**：浅色/深色/跟随系统

## 技术栈

- **桌面框架**：Tauri 2.0
- **前端**：Vue 3 + TypeScript + TailwindCSS V4
- **编辑器**：Monaco Editor
- **状态管理**：Pinia
- **国际化**：vue-i18n

## 项目结构

```
ai-skills-manager/
├── src/                    # Vue 前端源码
│   ├── components/         # Vue 组件
│   ├── views/             # 页面视图
│   ├── stores/            # Pinia 状态管理
│   ├── router/            # Vue Router
│   ├── i18n/              # 国际化
│   └── types/             # TypeScript 类型
├── src-tauri/             # Tauri Rust 后端
│   ├── src/
│   │   ├── commands/      # Tauri 命令
│   │   ├── models/        # 数据模型
│   │   └── services/      # 业务服务
│   └── Cargo.toml
├── docs/                  # 设计文档
└── .trae/                 # 项目规范
```

## 开发

### 环境要求

- Node.js 18+
- Rust 1.95+
- Windows 10/11

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
# 前端开发
npm run dev

# Tauri 开发
npm run tauri dev
```

### 构建

```bash
# 构建前端
npm run build

# 构建 Tauri 应用
npm run tauri build
```

## 默认存储目录

```
C:\Users\<用户名>\.ai-skills-manager\skills
```

## 配置位置

```
C:\Users\<用户名>\.ai-skills-manager\config.json
```

## 快捷键

| 功能 | 快捷键 |
|------|--------|
| 新建 Skill | Ctrl+N |
| 保存 | Ctrl+S |
| 刷新 | Ctrl+R |
| 搜索 | Ctrl+F |

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
