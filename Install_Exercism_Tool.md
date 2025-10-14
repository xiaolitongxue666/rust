# Exercism CLI 安装指南 

https://exercism.org/cli-walkthrough

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

# 配置工作目录（可选）
exercism configure --workspace=/path/to/your/exercism/workspace
```

### 3. 验证配置

```bash
# 检查配置
exercism whoami
```

## 基本使用

### 下载练习

```bash
# 下载特定练习
exercism download --exercise=hello-world --track=rust

# 下载所有练习
exercism download --track=rust
```

### 提交解决方案

```bash
# 提交解决方案
exercism submit path/to/solution.rs

# 提交多个文件
exercism submit src/lib.rs tests/lib.rs
```

### 获取帮助

```bash
# 查看所有命令
exercism help

# 查看特定命令帮助
exercism help download
exercism help submit
```

## 常用命令

| 命令 | 描述 |
|------|------|
| `exercism download` | 下载练习 |
| `exercism submit` | 提交解决方案 |
| `exercism status` | 查看练习状态 |
| `exercism test` | 运行测试 |
| `exercism mentor` | 请求导师指导 |
| `exercism whoami` | 查看当前用户信息 |

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

### 获取帮助

- 查看 [Exercism 文档](https://exercism.org/docs)
- 访问 [GitHub 仓库](https://github.com/exercism/cli)
- 在 [Exercism 论坛](https://forum.exercism.org) 寻求帮助

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
