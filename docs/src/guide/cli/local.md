
# 持久切换 `node` 版本
持久切换 `node` 版本

## 命令介绍
```sh
$ nsv local --help
Usage: index local [options] [version]

lasting you select node version

Arguments:
  version     use node version. (v14, 14, v14.xx.xx, 14.xx.xx)

Options:
  -h, --help  display help for command
```

:::tip
如果设置的版本在本地不存在 会自动下载与解压
:::

### nsv local xx
::: details 版本号该如何填写
:tada: :tada: :tada:
```sh
# 当本地有 v18.16.1 版本时
$ nsv local 18 // 选中了本地的 18.16.1 版本 不会进行下载

$ nsv local 18.15 // 本地没有 `18.15.xx` 的版本 会下载 `18.15.xx` 对应的最新版本

$ nsv local 17 // 本地没有 `17.xx.xx` 的版本 会下载 `17.xx.xx` 对应的最新版本
```
:::

```sh
$ nsv local 18
v18.16.1
```



## 如何使用

### 1. 持久修改
```sh
$ nsv local 18
Downloading  [■■■■■■■■■■■■■■■■■■■■■■] 100% | ETA: 0s | 22646968/22646968 6s
Extracting   [■■■■■■■■■■■■■■■■■■■■■■] 100% | ETA: 0s | 50143072/22646968 3s
v18.16.1
$ node -v
v18.16.1
```
