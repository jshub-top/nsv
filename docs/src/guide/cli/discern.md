# 自动识别
识别配置文件 自动切换 `node` 版本

## 命令介绍
```sh
$ nsv discern --help
Usage: index discern [options]

discern your project node config version.

Options:
  -e --enable   enable auto discern node version
  -d --disable  disable auto discern node version
  -s --status   discern status
  -h, --help    display help for command
```



### nsv discern -e
```sh
$ nsv discern -e
```
`-e` 单词 `enable` 的缩写 开启功能



### nsv discern -d
```sh
$ nsv discern -d
```
`-d` 单词 `disable` 的缩写 关闭功能



### nsv discern -s
```sh
$ nsv discern -s

discern enable:  true
// --or
discern enable:  false
```
`-s` 单词 `status` 的缩写 功能开启状态

以布尔值的形式展示状态
























## 如何使用

### 1. 添加配置

有两种配制方法 任选其一即可

#### 1. 添加 `.nsvrc` 文件的方式

```config
.
├─ node_modules
|  ...
├─ .nsvrc // [!code ++]
|  ...
└─ package.json
```
```config
# .nsvrc

18 // [!code ++]
```
#### 2. 添加 `nsv` 配置的方式
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

### 2. 通过命令行打开识别功能
```sh
# 打开 自动识别
$ nsv discern -e
```
```sh
$ nsv discern -s
discern enable:  true
```

### 3. 在项目根目录打开终端

以 `fish` 终端为例
```sh
v18.16.1
Welcome to fish, the friendly interactive shell
Type help for instructions on how to use fish
$ node -v
v18.16.1
```
当前node版本在打开终端时已修改完成 行为和 `nsv use` 一致
