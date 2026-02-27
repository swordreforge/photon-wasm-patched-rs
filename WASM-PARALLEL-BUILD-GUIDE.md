# WASM 并行化版本构建指南

本文档说明如何构建使用 Rayon 并行处理功能的 photon-wasm 库。

## 概述

本项目的 WASM 版本已经启用了 `wasm-bindgen-rayon` 支持，允许在 Web 环境中使用多线程并行处理图像。这可以显著提高图像处理性能，特别是在处理大图像或应用复杂滤镜时。

## 前置要求

1. **Rust 工具链**
   - Nightly Rust 编译器（已配置为 `nightly-2025-11-15`）
   - wasm32-unknown-unknown 目标
   - wasm-pack

2. **WebAssembly 工具**
   - wasm-opt（Binaryen 工具包的一部分）

3. **浏览器要求**
   - 支持 SharedArrayBuffer 的现代浏览器
   - 需要设置正确的跨域隔离策略（COOP/COEP）

## 配置说明

### 1. rust-toolchain.toml

```toml
[toolchain]
channel = "nightly-2025-11-15"
components = ["rust-src"]
targets = ["wasm32-unknown-unknown"]
```

### 2. .cargo/config.toml

```toml
[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
rustflags = [
  "-C", "target-feature=+atomics,+bulk-memory",
  "-C", "link-arg=--shared-memory",
  "-C", "link-arg=--max-memory=1073741824",
  "-C", "link-arg=--import-memory",
  "-C", "link-arg=--export=__wasm_init_tls",
  "-C", "link-arg=--export=__tls_size",
  "-C", "link-arg=--export=__tls_align",
  "-C", "link-arg=--export=__tls_base"
]

[unstable]
build-std = ["panic_abort", "std"]
```

### 3. Cargo.toml

```toml
[dependencies]
photon-rs = { path = "photon-master/crate", features = ["wasm-parallel"] }

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz", "--enable-bulk-memory", "--enable-nontrapping-float-to-int", "--enable-simd", "--enable-threads"]
```

## 构建命令

```bash
# 构建并行化版本
wasm-pack build --target web
```

构建产物将生成在 `pkg/` 目录中。

## JavaScript 使用方法

```javascript
import init, { initThreadPool, ImageProcessor } from './pkg/photon_wasm.js';

// 1. 初始化 WASM 模块
await init();

// 2. 初始化线程池
// 使用 navigator.hardwareConcurrency 获取可用线程数
const numThreads = navigator.hardwareConcurrency || 4;
await initThreadPool(numThreads);

// 3. 使用 ImageProcessor 进行图像处理
const processor = new ImageProcessor(width, height, imageData);
processor.apply_grayscale();
```

## COOP/COEP 配置

要使用 SharedArrayBuffer（多线程所需），必须设置以下 HTTP 响应头：

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

### 开发环境配置

**使用 Vite：**

```javascript
// vite.config.js
export default {
  server: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
};
```

**使用 Webpack：**

```javascript
// webpack.config.js
module.exports = {
  devServer: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
};
```

**使用 Parcel：**

```javascript
// .parcelrc
{
  "headers": {
    "*": {
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp"
    }
  }
}
```

**使用简单的 HTTP 服务器：**

```python
# http_server.py
from http.server import HTTPServer, SimpleHTTPRequestHandler

class CORSHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        super().end_headers()

if __name__ == '__main__':
    HTTPServer(('', 8000), CORSHandler).serve_forever()
```

## 特性检测

可以使用 `wasm-feature-detect` 库检测浏览器是否支持 WebAssembly 线程：

```javascript
import { threads } from 'wasm-feature-detect';

if (await threads()) {
  // 加载支持线程的版本
  const wasmPkg = await import('./pkg-with-threads/photon_wasm.js');
  await wasmPkg.default();
  await wasmPkg.initThreadPool(navigator.hardwareConcurrency);
} else {
  // 加载不支持线程的版本
  const wasmPkg = await import('./pkg-without-threads/photon_wasm.js');
  await wasmPkg.default();
}
```

## 性能对比

根据 `wasm-bindgen-rayon` 官方演示，使用多线程处理可以带来显著的性能提升：

- **单线程处理**：273ms
- **多线程处理（4核）**：87ms
- **加速比**：约 3.14x

实际性能提升取决于：
- 图像大小
- 处理操作的复杂度
- 设备的 CPU 核心数
- 浏览器的实现

## 构建产物

构建成功后，`pkg/` 目录将包含：

- `photon_wasm_bg.wasm` - WASM 二进制文件（~1.5MB）
- `photon_wasm.js` - JavaScript 胶水代码
- `photon_wasm.d.ts` - TypeScript 类型定义
- `photon_wasm_bg.wasm.d.ts` - WASM 模块的 TypeScript 类型定义
- `package.json` - NPM 包配置

## 导出的函数

并行化版本导出了以下关键函数：

- `initThreadPool(num_threads: number): Promise<any>` - 初始化线程池
- `ImageProcessor` - 图像处理器类
- 所有其他的图像处理函数（滤镜、变换等）

## 注意事项

1. **内存限制**
   - 最大内存设置为 1GB（`--max-memory=1073741824`）
   - 可以根据需要调整此值

2. **浏览器兼容性**
   - 需要 Chrome 74+、Firefox 79+、Safari 15.2+ 或 Edge 79+
   - 必须设置 COOP/COEP 响应头

3. **开发环境**
   - 如果使用本地开发服务器，确保配置了正确的响应头
   - 某些浏览器可能需要特定的安全上下文（HTTPS 或 localhost）

4. **调试**
   - 可以使用浏览器开发者工具查看 Web Worker 活动
   - 检查控制台是否有线程相关的错误信息

## 故障排除

### 问题：SharedArrayBuffer 未定义

**解决方案**：确保设置了 COOP/COEP 响应头

### 问题：wasm-opt 验证失败

**解决方案**：确保 `wasm-opt` 支持 `--enable-threads` 选项，版本 >= 2.100

### 问题：线程池初始化失败

**解决方案**：
- 检查浏览器是否支持 SharedArrayBuffer
- 确保 COOP/COEP 响应头正确设置
- 检查是否在正确的安全上下文中运行（HTTPS 或 localhost）

## 参考

- [wasm-bindgen-rayon 官方文档](https://github.com/RReverser/wasm-bindgen-rayon)
- [WebAssembly 线程支持](https://webassembly.org/roadmap/)
- [COOP 和 COEP](https://web.dev/coop-coep/)
- [photon-rs 项目](https://github.com/silvia-odwyer/photon)