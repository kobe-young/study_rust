# 认识cargo

- 包管理工具，对比go，go语言在很久之后才有了官方的包管理工具。而rust开始就有，这样为使用者统一了规范和标准。
- 总而言之，cargo 提供了一系列的工具，从项目的建立、构建到测试、运行直至部署，为 Rust 项目的管理提供尽可能完整的手段。

### 使用

#### 创建项目
- 创建项目命令如下：
  ```shell
  $ cargo new <项目名>
  ```
  - 目录结构为 
  ```shell
    .
    ├── .git
    ├── .gitignore
    ├── Cargo.toml
    └── src
        └── main.rs
  ```
  - 项目名为文件夹名的项目，里面有如下文件
    - src: 源代码
      - main.rs: 入口函数
    - Cargo.toml: 项目管理，包依赖等等。
    - .git: git管理目录
    - .gitignore: git忽略文件
  - 注意：
    - 早期的 cargo 在创建项目时，必须添加 --bin 的参数
    - 现在的版本，已经无需此参数，cargo 默认就创建 bin 类型的项目，顺便说一句，Rust 项目主要分为两个类型：bin 和 lib，前者是一个可运行的项目，后者是一个依赖库项目
  
#### 编译运行
- 有两种方式运行项目
  - 直接运行: ```cargo run```
  - 手动编译和运行：```cargo build && <二进制输出目录执行>```

##### 直接运行
- 切换到项目根目录，然后执行```cargo run```
- 输出结构如下：
  ```shell
  $ cargo run
    Compiling hello v0.1.0 (/Users/<user>/study/rust/study_rust/bible/01/hello)
      Finished dev [unoptimized + debuginfo] target(s) in 2.28s
        Running `target/debug/hello`
    Hello, world!
  ```
  - 这里分析一下输出的内容
    - v0.1.0: 编译输出，以及当前版本
    - dev [unoptimized + debuginfo] target(s) in 2.28s: 环境，分为开发、发布，即测试和线上
    - target/debug/hello: 输出目录。
      - 默认是debug模式，即dev环境，
      - 如果需要线上发布，则需要添加参数：```cargo run --release```
        - 这时编译输出则会改为：```target/release/hello```
- 注意：这里其实是先执行了编译，然后运行
  - 我们也可以直接运行编译好的二进制：```./target/debug/hello```
  
##### 手动编译并运行
- 将直接运行分拆为两步:
  - 编译: ```cargo build```
    - 默认编译为debug模式
    - release模式命令: ```cargo build --release```
  - 运行: 
    - 默认debug: ```./target/debug/hello```
    - release: ```./target/release/hello```

##### debug/release区别
- 区别:
  - debug编译速度快，不做任何优化，可以从输出看到，但是运行会很慢。
  - release编译速度慢，做优化，但是运行速度快。
- 问题:
  - 当项目大了后，cargo run 和 cargo build 不可避免的会变慢，那么有没有更快的方式来验证代码的正确性呢？大杀器来了，接着！
  
##### 检验代码
- 当项目大了之后，编译和运行会变得慢，那么我们可以用```check```命令来检测代码是否编译通过，从而来减少编译时间。
  - 作用：
    - 快速的检查一下代码能否编译通过。因此该命令速度会非常快，能节省大量的编译时间。

##### 其他项目文件
- 项目目录中还有其他项目文件:
  - Cargo.toml: 项目数据描述文件，项目的所有元配置信息，rust如果按照期望的编译、运行项目，必须按照规范构建该文件。
  - Cargo.lock: 根据```Cargo.toml```文件生成的项目依赖详细清单。
    - 注意：git文件管理
      - 什么情况下该把 Cargo.lock 上传到 git 仓库里？很简单，当你的项目是一个可运行的程序时，就上传 Cargo.lock，如果是一个依赖库项目，那么请把它添加到 .gitignore 中。
- Cargo.toml文件内容：
  - 文件内容如下：
    ```txt
    [package]
    name = "hello"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    ```
  - 内容详解
    - package 配置段落
      - ```[package]```: 记录了项目描述信息
      - ```name```: 项目名称
      - ```version```: 当前版本
      - ```edition```: 当前rust的大版本。注意，rust的发布机制，6周一个小版本，2-3年一个大版本，即一个稳定版本。
    - dependencies 配置段落
      - ```[dependencies]```: 定义了项目依赖。
      - 示例
        ```txt
        [dependencies]
        rand = "0.3"
        hammer = { version = "0.5.0"}
        color = { git = "https://github.com/bjz/color-rs" }
        geometry = { path = "crates/geometry" }
        ```
      - 在 Cargo.toml 中，主要通过各种依赖段落来描述该项目的各种依赖项：
        - 基于 Rust 官方仓库 crates.io，通过版本说明来描述
        - 基于项目源代码的 git 仓库地址，通过 URL 来描述
        - 基于本地项目的绝对路径或者相对路径，通过类 Unix 模式的路径来描述
      - 基于上面的示例，分析如下：
        - crates.io: 即官方仓库，但是国内好像不能访问? 这个需要确认一下
        - git: git仓库，即一个git的url
        - path: 本地路径
  
  