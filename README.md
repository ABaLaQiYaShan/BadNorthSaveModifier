# BadNorth 存档修改器 (BadNorthSaveModifier)

[English](#english) | [中文](#中文)

## 中文

### 项目介绍

**BadNorthSaveModifier** 是一个功能强大的 GUI 应用程序，专为 *Bad North* 游戏设计，用于快速、方便地修改游戏存档数据。该工具提供了直观的用户界面，使玩家能够轻松管理英雄、升级、背包物品和其他游戏数据。

### 主要功能

- **英雄管理**
  - 查看所有已招募的英雄信息
  - 编辑英雄属性（等级、经验值等）
  - 管理英雄状态

- **升级系统**
  - 圣杯升级 (Grail Upgrade)
  - 炸弹升级 (Bomb Upgrade)
  - 地雷升级 (Mine Upgrade)
  - 哲学家之石升级 (Philosopher's Stone Upgrade)
  - 体型升级 (Size Upgrade)
  - 战锤升级 (Warhammer Upgrade)
  - 丰收角升级 (Cornucopia Upgrade)
  - 战号升级 (War Horn Upgrade)

- **背包管理**
  - 查看和编辑背包物品数量
  - 自动背包容量检查（最大 20 个物品）
  - 快速增加/减少物品

- **界面特性**
  - 中文与英文支持
  - 亮色和黑暗主题
  - 设置保存和恢复
  - 友好的错误提示

### 技术栈

- **语言**: Rust
- **UI 框架**: egui 0.24
- **序列化**: serde, serde_json
- **其他**:
  - walkdir (文件遍历)
  - rfd (文件选择对话框)
  - anyhow, thiserror (错误处理)
  - log, env_logger (日志记录)

### 项目依赖

本项目依赖于 **[BadNorthSaveConverter](https://github.com/ABaLaQiYaShanMaiI/BadNorthSaveConverter)**，该项目提供了 Bad North 存档文件的转换和序列化功能。

### 项目结构

```
BadNorthSaveModifier/
├── src/
│   ├── main.rs                 # 应用入口和主逻辑
│   ├── lib.rs                  # 库入口
│   ├── models.rs               # 数据模型
│   ├── save_manager.rs         # 存档读写管理 (~2100 行)
│   ├── settings.rs             # 应用设置
│   ├── class_dictionary.rs     # 类型字典
│   ├── upgrade_dictionary.rs   # 升级字典
│   └── ui/
│       ├── mod.rs              # UI 模块入口
│       ├── styles.rs           # UI 样式定义
│       └── components/
│           └── mod.rs          # UI 组件
├── Cargo.toml                  # 项目配置和依赖
└── README.md                   # 项目说明文档
```

### 安装与编译

#### 前置要求
- Rust 1.56+ (推荐最新稳定版)
- Cargo

#### 编译步骤

1. **克隆或下载项目**
   ```bash
   git clone <repository-url>
   cd BadNorthSaveModifier
   ```

2. **编译项目**
   ```bash
   cargo build --release
   ```

3. **运行应用**
   ```bash
   cargo run --release
   ```

编译后的可执行文件位于 `target/release/BadNorthSaveModifier.exe`

### 使用方法

1. **启动应用**
   - 运行编译后的 `BadNorthSaveModifier.exe`

2. **选择编辑器 (Unity Editor)**
   - 首次运行时，选择 Bad North 的游戏编辑器执行文件

3. **选择存档文件**
   - 导航到存档文件位置并选择要编辑的存档

4. **编辑存档**
   - 在 UI 中修改英雄、升级、物品等数据
   - 修改实时预览

5. **保存修改**
   - 点击保存按钮将修改后的数据写回存档文件

### 核心模块详解

#### `save_manager.rs` (~2100 行)
包含存档文件的读写操作，主要分为 10 个功能模块：

- **模块 1-4**: 基础操作
  - 文件 I/O 和序列化
  - 内部工具函数
  - 英雄升级
  - 属性管理

- **模块 5-7**: 数据查询
  - 存档数据查询
  - 货币查询
  - 圣杯查询

- **模块 8-10**: 背包管理
  - 通用背包操作
  - 快捷方法 (56 个宏生成的方法)
  - 统计摘要

#### `ui/` 目录
包含所有用户界面相关代码，使用 egui 框架提供跨平台支持。

### 配置与设置

应用支持以下设置项（通过 `settings.rs` 管理）：

- **语言设置** (Language)
  - 中文 (ZH_CN)
  - 英文 (EN)

- **主题设置** (ColorMode)
  - 亮色主题 (Light)
  - 黑暗主题 (Dark)

- **用户首选项**
  - 自动保存应用设置
  - 记忆上次选择的文件路径

### 故障排除

| 问题 | 解决方案 |
|------|---------|
| 无法加载存档文件 | 确保文件路径正确；检查文件是否损坏 |
| UI 显示不清晰 | 尝试更改主题设置；检查系统 DPI 设置 |
| 编辑后存档无效 | 确保在保存前未关闭游戏；尝试从备份恢复 |
| 中文显示乱码 | 检查系统字体设置；更新应用到最新版本 |

### 开发贡献

欢迎提交 Issue 和 Pull Request！

### 许可证

根据项目具体许可证要求填写（如 MIT、Apache 2.0 等）

### 作者

**ABaLaQiYaShanMai**

### 更新日志

- **v0.1.0** - 初始版本发布

---

## English

### Project Description

**BadNorthSaveModifier** is a powerful GUI application designed for the *Bad North* game to quickly and conveniently modify game save data. The tool provides an intuitive user interface that allows players to easily manage heroes, upgrades, inventory items, and other game data.

### Key Features

- **Hero Management**
  - View all recruited hero information
  - Edit hero attributes (level, experience, etc.)
  - Manage hero status

- **Upgrade System**
  - Grail Upgrade
  - Bomb Upgrade
  - Mine Upgrade
  - Philosopher's Stone Upgrade
  - Size Upgrade
  - Warhammer Upgrade
  - Cornucopia Upgrade
  - War Horn Upgrade

- **Inventory Management**
  - View and edit inventory item quantities
  - Automatic inventory capacity checks (max 20 items)
  - Quick add/remove items

- **UI Features**
  - Chinese and English support
  - Light and dark themes
  - Settings persistence
  - User-friendly error messages

### Tech Stack

- **Language**: Rust
- **UI Framework**: egui 0.24
- **Serialization**: serde, serde_json
- **Other**:
  - walkdir (directory traversal)
  - rfd (filDependencies

This project depends on **[BadNorthSaveConverter](https://github.com/ABaLaQiYaShanMaiI/BadNorthSaveConverter)**, which provides conversion and serialization functionality for Bad North save files.

### Project e picker dialogs)
  - anyhow, thiserror (error handling)
  - log, env_logger (logging)

### Project Structure

```
BadNorthSaveModifier/
├── src/
│   ├── main.rs                 # Application entry and main logic
│   ├── lib.rs                  # Library entry point
│   ├── models.rs               # Data models
│   ├── save_manager.rs         # Save file management (~2100 lines)
│   ├── settings.rs             # Application settings
│   ├── class_dictionary.rs     # Class type dictionary
│   ├── upgrade_dictionary.rs   # Upgrade dictionary
│   └── ui/
│       ├── mod.rs              # UI module entry
│       ├── styles.rs           # UI styles
│       └── components/
│           └── mod.rs          # UI components
├── Cargo.toml                  # Project configuration
└── README.md                   # Project documentation
```

### Installation & Compilation

#### Prerequisites
- Rust 1.56+ (latest stable recommended)
- Cargo

#### Build Steps

1. **Clone or download the project**
   ```bash
   git clone <repository-url>
   cd BadNorthSaveModifier
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the application**
   ```bash
   cargo run --release
   ```

The compiled executable will be at `target/release/BadNorthSaveModifier.exe`

### Usage Guide

1. **Launch the application**
   - Run the compiled `BadNorthSaveModifier.exe`

2. **Select the game editor**
   - On first run, select the Bad North game editor executable

3. **Select a save file**
   - Navigate to the save file location and select the save to edit

4. **Edit the save**
   - Modify heroes, upgrades, items, and other data in the UI
   - Changes are previewed in real-time

5. **Save changes**
   - Click the save button to write the modified data back to the save file

### Core Modules Overview

#### `save_manager.rs` (~2100 lines)
Handles save file read/write operations with 10 main functional modules:

- **Modules 1-4**: Basic operations
  - File I/O and serialization
  - Utility functions
  - Hero upgrades
  - Attribute management

- **Modules 5-7**: Data queries
  - Save data queries
  - Currency queries
  - Grail queries

- **Modules 8-10**: Inventory management
  - Generic inventory operations
  - Shortcut methods (56 macro-generated methods)
  - Summary statistics

#### `ui/` Directory
Contains all user interface code using the egui framework for cross-platform support.

### Configuration & Settings

The application supports the following settings (managed via `settings.rs`):

- **Language Settings** (Language)
  - Chinese (ZH_CN)
  - English (EN)

- **Theme Settings** (ColorMode)
  - Light Theme
  - Dark Theme

- **User Preferences**
  - Auto-save application settings
  - Remember last selected file paths

### Troubleshooting

| Issue | Solution |
|-------|----------|
| Cannot load save file | Verify file path is correct; check if file is corrupted |
| UI display not clear | Try changing theme settings; check system DPI settings |
| Save file invalid after editing | Ensure game is closed during saving; try restoring from backup |
| Chinese characters display incorrectly | Check system font settings; update app to latest version |

### Contributing

Issues and Pull Requests are welcome!

### License

[Specify the license according to your project requirements, e.g., MIT, Apache 2.0, etc.]

### Author

**ABaLaQiYaShanMai**

### Changelog

- **v0.1.0** - Initial release
