# 构建

## 1. 安装 llvm & opencv

首先安装好 chocolatey，安装完毕后执行如下指令：

```bash
choco install llvm opencv
```

待上述依赖安装完毕后，需要手动配置环境变量。首先找到 opencv 的安装路径（默认 `C:\tools\opencv`），这里我们需要添加三个环境变量：

- **OPENCV_INCLUDE_PATHS**: C:\tools\opencv
- **OPENCV_LINK_LIBS**: opencv_world460
- **OPENCV_LINK_PATHS**: C:\tools\opencv\build\x64\vc15\lib

> 这里的 `OPENCV_LINK_LIBS` 实际上就是 `OPENCV_LINK_PATHS` 下的 `.lib` 文件。

## 2. 通过 cargo 编译源代码

```bash
cargo build
# 或
cargo build -r # release
```

**此处有一步非常重要的步骤不可以省略，否则会导致应用无法运行**，将该仓库下的 `opencv_world460.dll` 拷贝至待执行的 exe 文件旁。

执行 run 指令运行程序

```bash
cargo run -r -- "C:/Users/10563/Desktop/dzc.jpg" # 参数为需要读取的文件
```

> 因为用了 `rust-analyze` 这个插件，导致 debug 期间经常乱编译，每次 `cargo build` 都需要重新链接编译 `opencv`。所以建议调试时也通过 **release** 运行。

> 如果 `cargo run` 的过程中出现 `(exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)` 的错误，请确认环境变量是否按要求配置并生效，或者将该仓库下的 `opencv_world460.dll` 拷贝至待执行的 exe 文件旁。
