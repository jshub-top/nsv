# nsv 是什么？ {#what-is-nsv}


`nsv` 是一个命令行`node`版本管理工具, 可以方便快捷的切换node版本. 使用rust构建, 适配主流平台.


<div class="tip custom-block" style="padding-top: 8px">

只是想尝试一下？跳到[快速开始](./getting-started)。

</div>

## 使用场景 {#use-cases}


- **前端**

  当公司项目存在历史包袱, 如三年前的前端项目使用的依赖生态或许并不适配高版本node, 在这种场景下使用 `nsv` 来管理node版本是一个非常明智的选择

- **nodejs项目**

  和前端项目遇到了同样的场景, 由于历史包袱的原因, 需要更低的node版本来运行项目

- **ci/cd集成**

  `nsv`允许使用项目配置的node版本进行切换, 在项目根目录打开终端, nsv会自动识别项目所配置的node版本, 减轻开发者维护负担.


## 使用体验 {#developer-experience}

nsv 旨在使用减少用户手动操作命令行的次数, 提高开发效率


- **[Rust 驱动 🚀](https://www.rust-lang.org/zh-CN/)**：高性能, 无需手动管理GC, 0成本抽象, 使得rust同时拥有开发效率和运行性能.

- **[兼容其他工具](./markdown)**：nsv 自定义读取行为, 可以配置为读取类似工具的配置文件如 `.nvmrc`


## 性能 {#performance}

与其他工具对比性能更强. nvm使用shell脚本语言构建, 运行时速度 nsv更强

