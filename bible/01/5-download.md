# 下载依赖

- 官方网站的库的下载，国内你懂得，需要翻墙，很麻烦。
- 通常推荐使用的是用国内的镜像来替换国外的。
- 具体的配置有两种：
  - 1.新增镜像地址，然后项目中需要制定对应的库，从指定的源下载。
  - 2.替换镜像地址，顾名思义就是所有的库都从替换的镜像地址下载。

### 新增镜像地址
- 新增需要在项目中指定源，那么这样就对项目存在侵入性。
- 同时，不同的项目这样管理会非常复杂。
- 不推荐这种方式。

### 替换镜像地址
- 替换就是将官方的镜像地址，全部替换为新的镜像地址。
- 操作方法：
  - 配置 ```$HOME/.cargo/config.toml```
    ```txt
    [source.crates-io]
    replace-with = 'ustc'

    [source.ustc]
    registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    ```
    - 如上配置为中科大的源。
    - 解释：
      - replace-with: 替换的源名称
      - source.<源名称>：源定义
      - registry: 源配置地址。

### 下载卡住问题
- 当我们在vscode操作配置```cargo.toml```，这时会触发下载依赖，然后下载未完成，我们在命令行执行了```cargo run/build```则会出现卡住，如下提示
    ```
    $ cargo build
      Blocking waiting for file lock on package cache
      Blocking waiting for file lock on package cache
    ```
  - 其实这个报错就是因为 VSCODE 的下载太慢了，而且该下载构建还锁住了当前的项目，导致你无法在另一个地方再次进行构建。
  - 解决办法也很简单：
    - 增加下载速度，见前面内容
    - 耐心等待持有锁的用户构建完成
    - 强行停止正在构建的进程，例如杀掉 IDE 使用的 rust-analyzer 插件进程，然后删除 $HOME/.cargo/.package_cache 目录
