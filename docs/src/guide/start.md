---
title: 开始
---

# 快速开始
本页面将为您介绍如何安装nsv 以及基本的使用例子

## 安装 {#install}

### 1. 请选择您的平台

#### windows

```PowerShell [windows]
irm https://raw.githubusercontent.com/1739616529/nsv/main/shell/install.ps1 | iex
```

::: tip
可选安装路径(默认 -- C:\\Users\\{username}\\.nsv)
:::

#### macos

```sh [macos]
sh -c "$(curl -fsSL https://raw.githubusercontent.com/1739616529/nsv/main/shell/install.sh)"
```

#### linux

```sh [linux]
sh -c "$(curl -fsSL https://raw.githubusercontent.com/1739616529/nsv/main/shell/install.sh)"
```

### 2. 验证安装

重启终端
并输入 `nsv -v`
显示当前版本号

```sh
$ nsv -v
0.0.1
```

::: warning 安装过程遇到问题?
[点击此处向Github提交issuse](https://github.com/1739616529/nsv/issues/new)
:::


## 基本使用例子

::: tip
在本地无法找到对应版本时, 会自动下载解压
:::

### 1. 临时切换node版本

在这个例子中我演示如何临时切换 `node` 版本 切换到大版本为 18 的最新版本

详细信息请前往[这个页面](/guide/cli/use)查看详情

```sh
$ nsv use 18
Downloading  [■■■■■■■■■■■■■■■■■■■■■■] 100% | ETA: 0s | 22646968/22646968 6s
Extracting   [■■■■■■■■■■■■■■■■■■■■■■] 100% | ETA: 0s | 50143072/22646968 3s
v18.16.1
$ node -v
v18.16.1
```

### 2. 持久切换node版本

在这个例子中我演示如何持久切换 `node` 版本 切换到大版本为 18 的最新版本

详细信息请前往[这个页面](/guide/cli/local)查看详情

```sh
$ nsv local 18
Downloading  [■■■■■■■■■■■■■■■■■■■■■■] 100% | ETA: 0s | 22646968/22646968 6s
Extracting   [■■■■■■■■■■■■■■■■■■■■■■] 100% | ETA: 0s | 50143072/22646968 3s
v18.16.1
$ node -v
v18.16.1
```

请重启终端验证是否生效

```sh
$ node -v
v18.16.1
```

### 3. 自动修改node版本

在这个例子中我演示如何根据配置文件自动切换切换 `node` 版本 切换到大版本为 18 的最新版本

详细信息请前往[这个页面](/guide/cli/discern)查看详情

有两种配置方式 需要预先添加配置

第一种在 package.json 同级目录下创建 .nsvrc 文件 内容为 18
```config
.
├─ node_modules
├─ .nsvrc // [!code ++]
└─ package.json
```
```config
# .nsvrc

18 // [!code ++]
```
第二种在 package.json 中 设置 nsv 配置
```json
# package.json

{
    "name": "xxx",

    ...

    "nsv": { // [!code ++]
        "version": "18" // [!code ++]
    }, // [!code ++]

    ...

}

```

确保满足以上任一条件后 再继续接下来的操作
```sh
# 打开 自动识别
$ nsv discern -e
```
```sh
$ nsv discern -s
discern enable:  true
```
状态为true时 即可在 项目根目录打开终端 自动修改到对应版本

以fish终端为例
```sh
v18.16.1
Welcome to fish, the friendly interactive shell
Type help for instructions on how to use fish
$ node -v
v18.16.1
```

::: warning 在例子中遇到问题?
[点击此处向Github提交issuse](https://github.com/1739616529/nsv/issues/new)
:::
