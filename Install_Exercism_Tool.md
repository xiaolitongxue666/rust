# Exercism CLI 安装指南 

https://exercism.org/cli-walkthrough
https://exercism.org/docs/using/solving-exercises/working-locally

## 概述

Exercism CLI 是 Exercism 平台的命令行工具，允许你下载练习、提交解决方案并与社区互动。本指南将介绍在不同操作系统下如何安装和使用该工具。

## 系统要求

- 支持的操作系统：Windows、macOS、Linux
- 网络连接（用于下载练习和提交解决方案）
- 有效的 Exercism 账户

## Windows 安装

### 方法 1：直接下载（推荐）

1. 访问 [Exercism CLI GitHub 发布页面](https://github.com/exercism/cli/releases)
2. 下载最新版本的 `exercism-windows-x64.zip`
3. 解压到任意目录（如 `C:\tools\exercism`）
4. 将目录添加到系统 PATH 环境变量

**详细步骤：**
```powershell
# 创建目录
mkdir C:\tools\exercism

# 下载文件（手动从 GitHub 下载）
# 解压 exercism.exe 到 C:\tools\exercism\

# 添加到 PATH 环境变量
$env:PATH += ";C:\tools\exercism"
```

### 方法 2：使用 Scoop

```powershell
# 安装 Scoop（如果尚未安装）
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

# 安装 Exercism CLI
scoop install exercism
```

### 方法 3：使用 winget

```powershell
# 使用 Windows 包管理器
winget install exercism.exercism
```

### 方法 4：使用官方安装脚本

```powershell
# 使用官方安装脚本
curl -fsSL https://exercism.io/api/v1/install | powershell
```

## macOS 安装

### 方法 1：使用 Homebrew（推荐）

```bash
# 安装 Homebrew（如果尚未安装）
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装 Exercism CLI
brew install exercism
```

### 方法 2：使用 MacPorts

```bash
sudo port install exercism
```

### 方法 3：直接下载

1. 访问 [Exercism CLI 发布页面](https://github.com/exercism/cli/releases)
2. 下载最新版本的 `exercism-macos-x64.tar.gz`
3. 解压并移动到 `/usr/local/bin/`

```bash
tar -xzf exercism-macos-x64.tar.gz
sudo mv exercism /usr/local/bin/
```

## Linux 安装

### 方法 1：使用包管理器

#### Ubuntu/Debian
```bash
# 添加 Exercism 官方仓库
curl -fsSL https://exercism.io/api/v1/install | sh

# 或者使用 snap
sudo snap install exercism
```

#### CentOS/RHEL/Fedora
```bash
# 使用 dnf/yum
sudo dnf install exercism
# 或
sudo yum install exercism
```

#### Arch Linux
```bash
# 使用 AUR
yay -S exercism
# 或
paru -S exercism
```

### 方法 2：使用 snap（通用 Linux）

```bash
sudo snap install exercism
```

### 方法 3：直接下载

```bash
# 下载最新版本
curl -fsSL https://exercism.io/api/v1/install | sh

# 或者手动下载
wget https://github.com/exercism/cli/releases/latest/download/exercism-linux-x64.tar.gz
tar -xzf exercism-linux-x64.tar.gz
sudo mv exercism /usr/local/bin/
```

## 验证安装

安装完成后，验证 CLI 是否正确安装：

```bash
exercism version
```

如果显示版本信息，说明安装成功。

## 初始配置

### 1. 获取 API Token

1. 访问 [Exercism 网站](https://exercism.org)
2. 登录你的账户
3. 进入 [Settings](https://exercism.org/settings)
4. 复制你的 API Token

### 2. 配置 CLI

```bash
# 配置 API Token
exercism configure --token=YOUR_API_TOKEN

# 配置工作目录（重要：影响下载路径）
exercism configure --workspace=/path/to/your/exercism/workspace
```

**工作空间配置说明**：
- exercism CLI 会在工作空间下自动创建以轨道名命名的子目录
- 对于 Rust 轨道，会创建 `rust` 子目录
- 实际下载路径：`工作空间/rust/练习名/`
- 建议将工作空间设置为父目录，而不是 `rust` 目录本身

### 3. 验证配置

```bash
# 检查配置
exercism whoami
```

## 基本使用

### 1. 配置 CLI

首先需要配置你的 API Token：

```bash
# 配置 API Token（从 https://exercism.org/settings 获取）
exercism configure --token=YOUR_API_TOKEN

# 配置工作目录（重要：影响下载路径）
exercism configure --workspace=/path/to/your/exercism/workspace

# 查看当前配置
exercism configure --show

# 查看工作空间路径
exercism workspace
```

**注意**：在 Windows Git Bash 中，如果 PATH 未正确设置，需要使用完整路径：
```bash
C:/tools/exercism/exercism.exe configure --token=YOUR_API_TOKEN
```

**工作空间路径配置示例**：
```bash
# 正确配置：工作空间设置为父目录
exercism configure --workspace=E:/Code/my_code/Rust
# 实际下载路径：E:/Code/my_code/Rust/rust/flower-field

# 错误配置：工作空间设置为 rust 目录
exercism configure --workspace=E:/Code/my_code/Rust/rust
# 实际下载路径：E:/Code/my_code/Rust/rust/rust/flower-field (多了一层 rust)
```

### 2. 下载练习

```bash
# 下载特定练习
exercism download --exercise=hello-world --track=rust

# 下载所有练习
exercism download --track=rust

# 强制覆盖现有练习目录
exercism download --exercise=hello-world --track=rust --force

# 从练习页面获取具体命令（推荐）
# 每个练习页面都会显示正确的下载命令
```

**Windows Git Bash 示例**：
```bash
C:/tools/exercism/exercism.exe download --exercise=hello-world --track=rust
```

### 3. 解决练习

下载练习后，你会得到以下文件：
- `README.md`: 练习说明
- `HELP.md`: 如何运行测试和获取帮助
- `HINTS.md`: 提示信息（可选）
- 测试文件: 验证解决方案的正确性
- 存根文件: 提供起始点

**运行测试：**
```bash
# 进入练习目录（重要：必须在练习目录中运行）
cd hello-world

# 验证目录正确性（检查是否有 .exercism 文件夹）
ls -la .exercism

# 使用 exercism 运行测试
exercism test

# 或者使用 cargo test（Rust 项目）
cargo test

# 或者使用项目特定的测试脚本
./bin/test-exercise .
```

**Windows Git Bash 示例**：
```bash
# 验证目录正确性
ls -la .exercism

# 运行测试
C:/tools/exercism/exercism.exe test
```

### 4. 提交解决方案

```bash
# 确保在练习目录中（包含 .exercism 文件夹）
# 提交单个文件
exercism submit src/lib.rs

# 提交多个文件
exercism submit src/lib.rs tests/lib.rs

# 提交所有相关文件（推荐）
exercism submit .
```

**Windows Git Bash 示例**：
```bash
# 确保在练习目录中
C:/tools/exercism/exercism.exe submit src/lib.rs
# 或者提交所有文件
C:/tools/exercism/exercism.exe submit .
```

### 5. 查看练习状态

```bash
# 查看当前练习状态
exercism status

# 查看特定练习状态
exercism status --exercise=hello-world --track=rust
```

### 6. 在网站上打开练习

```bash
# 在浏览器中打开练习页面
exercism open --exercise=hello-world --track=rust
```

### 7. 请求导师指导

```bash
# 请求导师指导（即使解决方案不完整）
exercism mentor --exercise=hello-world --track=rust
```

**Windows Git Bash 示例**：
```bash
C:/tools/exercism/exercism.exe open --exercise=hello-world --track=rust
```

### 8. 获取帮助

```bash
# 查看所有命令
exercism help

# 查看特定命令帮助
exercism help download
exercism help submit
exercism help configure

# 故障排除
exercism troubleshoot

# 升级 CLI
exercism upgrade
```

**Windows Git Bash 示例**：
```bash
C:/tools/exercism/exercism.exe help
C:/tools/exercism/exercism.exe troubleshoot
```

## 常用命令参考

| 命令 | 描述 | 示例 |
|------|------|------|
| `exercism configure` | 配置 CLI | `exercism configure --token=YOUR_TOKEN` |
| `exercism download` | 下载练习 | `exercism download --exercise=hello-world --track=rust` |
| `exercism submit` | 提交解决方案 | `exercism submit src/lib.rs` |
| `exercism test` | 运行测试 | `exercism test` |
| `exercism status` | 查看练习状态 | `exercism status` |
| `exercism open` | 在网站打开练习 | `exercism open --exercise=hello-world --track=rust` |
| `exercism mentor` | 请求导师指导 | `exercism mentor --exercise=hello-world` |
| `exercism workspace` | 查看工作空间路径 | `exercism workspace` |
| `exercism help` | 获取帮助 | `exercism help` |
| `exercism version` | 查看版本 | `exercism version` |
| `exercism troubleshoot` | 故障排除 | `exercism troubleshoot` |
| `exercism upgrade` | 升级 CLI | `exercism upgrade` |

## 完整工作流程示例

### 1. 首次设置

```bash
# 1. 配置 API Token
exercism configure --token=your-api-token-here

# 2. 查看配置
exercism configure --show

# 3. 设置工作目录（重要：影响下载路径）
exercism configure --workspace=/path/to/exercism

# 4. 查看工作空间路径
exercism workspace
```

**Windows Git Bash 示例**：

打开 https://exercism.org/settings/api_cli 拷贝token

```bash
# 配置 API Token
C:/tools/exercism/exercism.exe configure --token=your-api-token-here

# 配置工作空间（重要：影响下载路径）
C:/tools/exercism/exercism.exe configure --workspace=E:/Code/my_code/Rust

# 查看配置
C:/tools/exercism/exercism.exe configure --show

# 查看工作空间路径
C:/tools/exercism/exercism.exe workspace
```

### 2. 下载并解决练习

```bash
# 1. 下载练习（可以在任何目录运行）
exercism download --exercise=hello-world --track=rust

# 2. 进入练习目录（重要：必须进入具体练习目录）
cd hello-world

# 3. 验证目录正确性（检查是否有 .exercism 文件夹）
ls -la .exercism

# 4. 查看练习说明
cat README.md

# 5. 运行测试（查看失败情况）
exercism test
# 或者
cargo test

# 6. 编辑 src/lib.rs 实现解决方案
# ... 编写代码 ...

# 7. 再次运行测试
exercism test
# 或者
cargo test

# 8. 提交解决方案（必须在练习目录中运行）
exercism submit src/lib.rs
# 或者提交所有文件
exercism submit .
```

**Windows Git Bash 示例**：
```bash
# 1. 下载练习
C:/tools/exercism/exercism.exe download --exercise=hello-world --track=rust

# 2. 进入练习目录
cd hello-world

# 3. 验证目录正确性
ls -la .exercism

# 4. 运行测试
C:/tools/exercism/exercism.exe test
# 或者
cargo test

# 5. 提交解决方案
C:/tools/exercism/exercism.exe submit src/lib.rs
# 或者
C:/tools/exercism/exercism.exe submit .
```

### 3. 获取帮助和指导

```bash
# 查看练习帮助
cat HELP.md

# 查看提示（如果有）
cat HINTS.md

# 在网站上打开练习
exercism open --exercise=hello-world --track=rust

# 请求导师指导
exercism mentor --exercise=hello-world --track=rust

# 故障排除
exercism troubleshoot
```

**Windows Git Bash 示例**：
```bash
C:/tools/exercism/exercism.exe open --exercise=hello-world --track=rust
C:/tools/exercism/exercism.exe troubleshoot
```

## 重要：命令运行目录说明

### 需要在具体题目子目录下运行的命令

以下 exercism 命令**必须在单个练习目录中运行**（包含 `.exercism` 文件夹的目录）：

| 命令 | 说明 | 示例 |
|------|------|------|
| `exercism test` | 运行练习测试 | 必须在练习目录中 |
| `exercism submit` | 提交解决方案 | 必须在练习目录中 |
| `exercism status` | 查看练习状态 | 必须在练习目录中 |

### 可以在任何目录运行的命令

以下命令可以在任何目录运行：

| 命令 | 说明 | 示例 |
|------|------|------|
| `exercism download` | 下载练习 | 可以在任何目录 |
| `exercism configure` | 配置 CLI | 可以在任何目录 |
| `exercism help` | 获取帮助 | 可以在任何目录 |
| `exercism version` | 查看版本 | 可以在任何目录 |
| `exercism troubleshoot` | 故障排除 | 可以在任何目录 |
| `exercism upgrade` | 升级 CLI | 可以在任何目录 |
| `exercism workspace` | 查看工作空间 | 可以在任何目录 |

### 常见错误和解决方案

#### 错误示例
```bash
# ❌ 错误：在项目根目录运行
cd /path/to/exercism/rust
exercism test
# Error: open .exercism\metadata.json: The system cannot find the path specified.
```

#### 正确做法
```bash
# ✅ 正确：进入具体练习目录
cd /path/to/exercism/rust/hello-world
exercism test
# 或者
cargo test
```

### 目录结构说明

```
E:/Code/my_code/Rust/             # 工作空间（workspace）
└── rust/                         # 自动创建的轨道目录
    ├── hello-world/              # 单个练习目录
    │   ├── .exercism/           # exercism 元数据（重要！）
    │   ├── src/lib.rs          # 源代码
    │   ├── tests/              # 测试文件
    │   └── README.md           # 练习说明
    ├── anagram/                 # 另一个练习目录
    │   ├── .exercism/
    │   └── ...
    ├── flower-field/            # 新下载的练习
    │   ├── .exercism/
    │   └── ...
    └── ...
```

**关键点**：
- exercism CLI 会自动在工作空间下创建以轨道名命名的子目录
- 只有包含 `.exercism` 文件夹的目录才是有效的练习目录
- 才能运行 `test`、`submit`、`status` 等命令

## Windows Git Bash 特殊说明

### PATH 环境变量问题

在 Windows Git Bash 中，如果 `exercism` 命令无法识别，可以使用以下解决方案：

#### 方法 1：使用完整路径（推荐）
```bash
# 所有命令都使用完整路径
C:/tools/exercism/exercism.exe version
C:/tools/exercism/exercism.exe configure --token=YOUR_TOKEN
C:/tools/exercism/exercism.exe download --exercise=hello-world --track=rust
```

#### 方法 2：创建别名
```bash
# 在 ~/.bashrc 中添加别名
echo 'alias exercism="C:/tools/exercism/exercism.exe"' >> ~/.bashrc
source ~/.bashrc

# 之后可以直接使用
exercism version
```

#### 方法 3：添加到 PATH
```bash
# 临时添加到 PATH
export PATH="$PATH:/c/tools/exercism"

# 永久添加到 PATH（在 ~/.bashrc 中添加）
echo 'export PATH="$PATH:/c/tools/exercism"' >> ~/.bashrc
source ~/.bashrc
```

### 路径格式说明

在 Git Bash 中，Windows 路径需要转换：
- Windows: `C:\tools\exercism\exercism.exe`
- Git Bash: `C:/tools/exercism/exercism.exe` 或 `/c/tools/exercism/exercism.exe`

## 故障排除

### 常见问题

1. **命令未找到**
   - 确保已正确添加到 PATH 环境变量
   - 重启终端或重新登录

2. **权限问题**
   - 使用 `sudo` 运行命令（Linux/macOS）
   - 检查文件权限

3. **网络连接问题**
   - 检查防火墙设置
   - 确保可以访问 exercism.org

4. **API Token 问题**
   - 重新生成 API Token
   - 检查 Token 是否正确配置

5. **测试失败但本地通过**
   - 确保使用正确的文件路径
   - 检查是否有未提交的更改
   - 验证测试环境一致性

6. **"The system cannot find the path specified" 错误**
   - 确保在正确的练习目录中运行命令
   - 检查目录中是否有 `.exercism` 文件夹
   - 不要在项目根目录运行 `exercism test` 或 `exercism submit`

7. **exercism submit 无法找到文件**
   - 确保在练习目录中运行 `exercism submit`
   - 检查文件路径是否正确
   - 可以使用 `exercism submit .` 提交所有相关文件

8. **下载路径问题（多了一层目录）**
   - 检查工作空间配置：`exercism configure --show`
   - 确保工作空间设置为父目录，而不是 `rust` 目录本身
   - 正确配置：`exercism configure --workspace=E:/Code/my_code/Rust`
   - 错误配置：`exercism configure --workspace=E:/Code/my_code/Rust/rust`

9. **练习下载到错误位置**
   - 使用 `exercism workspace` 查看当前工作空间
   - 重新配置工作空间：`exercism configure --workspace=正确路径`
   - 使用 `--force` 参数重新下载：`exercism download --exercise=练习名 --track=rust --force`

### 诊断命令

```bash
# 检查版本
exercism version

# 检查配置
exercism configure --show

# 查看工作空间
exercism workspace

# 运行故障排除
exercism troubleshoot

# 检查命令位置
which exercism  # Linux/macOS
where exercism  # Windows
```

**Windows Git Bash 示例**：
```bash
C:/tools/exercism/exercism.exe version
C:/tools/exercism/exercism.exe configure --show
C:/tools/exercism/exercism.exe troubleshoot
```

### 获取帮助

- 查看 [Exercism 官方文档](https://exercism.org/docs/using/solving-exercises/working-locally)
- 访问 [GitHub 仓库](https://github.com/exercism/cli)
- 在 [Exercism 论坛](https://forum.exercism.org) 寻求帮助
- 使用 `exercism help` 查看所有可用命令

## 更新 CLI

### 自动更新

```bash
# 检查更新
exercism upgrade

# 或者重新安装最新版本
```

### 手动更新

1. 访问 [GitHub 发布页面](https://github.com/exercism/cli/releases)
2. 下载最新版本
3. 替换现有安装

## 卸载

### Windows
```powershell
# 使用 Chocolatey
choco uninstall exercism

# 使用 Scoop
scoop uninstall exercism
```

### macOS
```bash
# 使用 Homebrew
brew uninstall exercism

# 手动删除
sudo rm /usr/local/bin/exercism
```

### Linux
```bash
# 使用包管理器
sudo apt remove exercism  # Ubuntu/Debian
sudo dnf remove exercism  # CentOS/RHEL/Fedora

# 手动删除
sudo rm /usr/local/bin/exercism
```

## 相关资源

- [Exercism 官方网站](https://exercism.org)
- [CLI 文档](https://exercism.org/docs/using/solving-exercises/working-locally)
- [GitHub 仓库](https://github.com/exercism/cli)
- [社区论坛](https://forum.exercism.org)

---

*最后更新：2025年1月*
