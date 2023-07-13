---
title: 开始
---

## 安装 {#install}

### 1. 请选择您的平台

#### windows

```PowerShell [windows]
$ irm https://raw.githubusercontent.com/1739616529/nsv/main/shell/install.ps1 | iex
```

::: tip
可选安装路径(默认 -- C:\\Users\\{username}\\.nsv)
:::

#### macos

```sh [macos]
$ curl -fsSL https://raw.githubusercontent.com/1739616529/nsv/main/shell/install.sh | sh
```

#### linux

```sh [linux]
$ curl -fsSL https://raw.githubusercontent.com/1739616529/nsv/main/shell/install.sh | sh
```

### 2. 验证安装

重启终端
并输入 `nsv -v`
显示当前版本号

```sh
$ nsv -v
0.0.1
```

::: warning
安装过程遇到问题? [在Github上提交issuse](https://github.com/1739616529/nsv/issues/new)
:::