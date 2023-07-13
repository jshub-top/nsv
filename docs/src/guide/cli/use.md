
# 临时切换 `node` 版本
临时切换 `node` 版本 只对当前终端生效

## 命令介绍
```sh
$ nsv use --help
Usage: index use [options] [version]

use node version

Arguments:
  version      use node version. (v14, 14, v14.xx.xx, 14.xx.xx) (default: "")

Options:
  -r --remove  remove local node version
  -h, --help   display help for command
```

:::tip
如果设置的版本在本地不存在 会自动下载与解压
:::

### nsv use xx
::: details 版本号该如何填写
:tada: :tada: :tada:
```sh
# 当本地有 v18.16.1 版本时
$ nsv use 18 // 选中了本地的 18.16.1 版本 不会进行下载

$ nsv use 18.15 // 本地没有 `18.15.xx` 的版本 会下载 `18.15.xx` 对应的最新版本

$ nsv use 17 // 本地没有 `17.xx.xx` 的版本 会下载 `17.xx.xx` 对应的最新版本
```
:::
```sh
$ nsv use 18
v18.16.1
```
### nsv use -r
```sh
$ nsv use -r
```
删除当前临时设置的node版本


## 如何使用

### 1. 临时修改
```sh
$ nsv use 18
Downloading  [■■■■■■■■■■■■■■■■■■■■■■] 100% | ETA: 0s | 22646968/22646968 6s
Extracting   [■■■■■■■■■■■■■■■■■■■■■■] 100% | ETA: 0s | 50143072/22646968 3s
v18.16.1
$ node -v
v18.16.1
```
