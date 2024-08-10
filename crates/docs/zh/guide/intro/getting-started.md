# 快速开始 {#getting-started}





## windows安装 {#windows}


1. 输入并运行远程命令.

2. 运行完毕请重启电脑或者注销用户重新登录, 以便正确加载环境变量.

3. 在终端输入 `nsv -V` 查阅版本是否为最新版

- [Windows](https://support.microsoft.com/zh-cn/welcometowindows) 10版本以上.
- 通过[powershell](https://learn.microsoft.com/zh-cn/powershell/scripting/learn/ps101/01-getting-started?view=powershell-5.1)运行远程脚本.

::: code-group

```powershell [powershell]
$ powershell -c "irm https://raw.githubusercontent.com/1739616529/nsv/main/install/install.ps1 | iex"
```
:::

::: details 安装过程遇到了问题?
请查阅解决方案
:::






## Unix/MacOS/Linux安装 {#Unix}


1. 输入并运行远程命令.

2. 运行完毕请`source $PROFILE`或新建终端, 以便正确加载环境变量.

3. 在终端输入 `nsv -V` 查阅版本是否为最新版

- 通过 [bash](https://www.gnu.org/software/bash/) / [zsh](https://www.zsh.org/) / [fish](https://fishshell.com/) 运行远程脚本.

::: code-group

```sh [bash]
$ curl -fsSL https://raw.githubusercontent.com/1739616529/nsv/main/install/install.sh | bash
```

```sh [zsh]
$ curl -fsSL https://raw.githubusercontent.com/1739616529/nsv/main/install/install.sh | zsh
```

```sh [fish]
$ curl -fsSL https://raw.githubusercontent.com/1739616529/nsv/main/install/install.fish | fish
```

:::

::: details 安装过程遇到了问题?
请查阅解决方案
:::


## 配置文件 {#the-config-file}

全局配资文件储存位置在 `$HOME/.nsvrc`. 可能不存在


## 安装位置 {#source-files}

#### windows
当环境变量 `NSV_HOME` 存在时, 使用此路径, 不存在时使用当前 `pwd` 目录.

#### Unix
在用户目录 `~/.nsv`

## 配置默认版本 {#set-default-versions}
```sh
$ nsv add 20
20.10.12
```
