# myapp

myapp 是一个基于 Tauri 框架构建的跨平台桌面应用程序，目前处于非常早期的开发阶段。本项目旨在提供一个高性能、低资源占用的聊天应用。

### Attention
- 此项目版本目前未经审计
- 功能不完善，无完整社区

## License
This project is licensed under the [GNU Affero General Public License v3.0](LICENSE).
出于对通信软件安全性的考虑，暂定为传染性开源，转发服务等未来计划功能包含不会传染性

## 项目状态
version:0.0.1
🚧 此项目目前处于**非常早期的开发阶段**。功能尚不完善。欢迎提供建议！

## 特性

- **跨平台支持**：可在 Windows、macOS 和 Linux 上运行
- **条码扫描**：集成摄像头扫码功能
- **命令行接口**：// TODO::支持命令行交互操作 
- **现代 UI**：使用 React 19 构建的现代化用户界面


## 技术栈

- **前端**: React 19, TypeScript, Vite
- **框架**: Tauri 2.9.6
- **后端**: Rust
- **路由**: React Router v7
- **条码扫描**: @zxing/library, @zxing/browser
- **二维码生成**: qrcode

## 安装与运行

### 环境要求

- Node.js (>=18)
- Yarn
- Rust 工具链

### 开发环境搭建

```bash
# 克隆项目后进入目录
cd myapp

# 安装依赖
yarn install

# 启动开发服务器
yarn tauri dev
```

### 开发构建

```bash
# 构建
yarn tauri build
```

---
## HISTORY
- 2025 创建项目
- 2026 摸鱼中
## PLAN
先搭建基础框架

## 吐槽
在上学，无时间😭
希望有大佬帮忙一起开发，我太菜了