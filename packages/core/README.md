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