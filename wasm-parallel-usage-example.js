/**
 * WASM 并行化版本使用示例
 * 
 * 本示例展示如何使用启用 Rayon 并行处理功能的 photon-wasm 库
 * 
 * 注意：使用 SharedArrayBuffer 需要设置正确的跨域隔离策略（COOP/COEP）
 */

import init, { initThreadPool, ImageProcessor } from './pkg/photon_wasm.js';

// 配置跨域隔离策略
// 在 HTML 中添加以下响应头：
// Cross-Origin-Opener-Policy: same-origin
// Cross-Origin-Embedder-Policy: require-corp

async function main() {
    try {
        // 1. 初始化 WASM 模块
        console.log('正在初始化 WASM 模块...');
        await init();
        console.log('WASM 模块初始化完成');

        // 2. 初始化线程池
        // 使用 navigator.hardwareConcurrency 获取可用线程数
        const numThreads = navigator.hardwareConcurrency || 4;
        console.log(`正在初始化线程池，线程数: ${numThreads}...`);
        await initThreadPool(numThreads);
        console.log('线程池初始化完成');

        // 3. 加载图像
        const imageResponse = await fetch('path/to/your/image.jpg');
        const imageBytes = new Uint8Array(await imageResponse.arrayBuffer());
        
        // 4. 创建 ImageProcessor 实例
        const processor = new ImageProcessor(
            imageBytes.width,
            imageBytes.height,
            imageBytes
        );

        // 5. 应用图像处理操作（这些操作现在将使用多线程并行处理）
        console.log('正在应用滤镜...');
        processor.apply_grayscale();
        processor.apply_brightess(50);
        processor.apply_contrast(20);
        
        // 6. 获取处理后的图像
        const resultBase64 = processor.to_base64();
        console.log('图像处理完成');

        // 7. 显示结果
        const img = document.createElement('img');
        img.src = `data:image/jpeg;base64,${resultBase64}`;
        document.body.appendChild(img);

    } catch (error) {
        console.error('错误:', error);
    }
}

// 检测浏览器是否支持 WebAssembly 线程
async function checkThreadSupport() {
    try {
        // 检查 SharedArrayBuffer 支持
        if (typeof SharedArrayBuffer === 'undefined') {
            console.warn('SharedArrayBuffer 不支持，可能需要设置 COOP/COEP 响应头');
            return false;
        }

        // 可以使用 wasm-feature-detect 库进行更详细的检测
        // import { threads } from 'wasm-feature-detect';
        // const supported = await threads();
        // return supported;

        return true;
    } catch (error) {
        console.error('线程支持检测失败:', error);
        return false;
    }
}

// 启动应用
(async () => {
    const hasThreadSupport = await checkThreadSupport();
    
    if (hasThreadSupport) {
        console.log('✓ WebAssembly 线程支持已启用');
        await main();
    } else {
        console.warn('⚠ WebAssembly 线程不支持，将使用单线程模式');
        // 可以在这里加载不支持线程的版本
        // import init from './pkg-without-threads/photon_wasm.js';
        // await init();
        // ...
    }
})();

/**
 * 性能对比示例
 */
async function performanceComparison() {
    const imageBytes = /* 加载图像数据 */;
    const iterations = 10;

    // 不使用线程（使用不支持线程的构建）
    // const wasmPkgNoThreads = await import('./pkg-without-threads/photon_wasm.js');
    // await wasmPkgNoThreads.default();
    
    // const startNoThreads = performance.now();
    // for (let i = 0; i < iterations; i++) {
    //     const processor = new wasmPkgNoThreads.ImageProcessor(/* ... */);
    //     processor.apply_grayscale();
    // }
    // const timeNoThreads = performance.now() - startNoThreads;

    // 使用线程
    await init();
    await initThreadPool(navigator.hardwareConcurrency);
    
    const startWithThreads = performance.now();
    for (let i = 0; i < iterations; i++) {
        const processor = new ImageProcessor(/* ... */);
        processor.apply_grayscale();
    }
    const timeWithThreads = performance.now() - startWithThreads;

    console.log(`不使用线程: ${timeNoThreads.toFixed(2)}ms`);
    console.log(`使用线程: ${timeWithThreads.toFixed(2)}ms`);
    console.log(`加速比: ${(timeNoThreads / timeWithThreads).toFixed(2)}x`);
}