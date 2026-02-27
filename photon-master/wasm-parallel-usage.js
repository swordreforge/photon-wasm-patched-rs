/**
 * Photon WASM 并行处理使用示例
 * 
 * 本示例演示如何使用 wasm-bindgen-rayon 在 WebAssembly 中启用并行处理
 * 以提高图像处理性能。
 */

// 导入 WASM 模块
import * as photon from './pkg/photon_wasm.js';

/**
 * 示例 1: 基本用法 - 初始化线程池
 */
async function basicExample() {
    // 初始化线程池，使用 4 个线程
    // 必须在任何并行操作之前调用
    await photon.PhotonImage.init_thread_pool(4);
    
    console.log('线程池已初始化，现在可以使用并行处理功能');
}

/**
 * 示例 2: 自动检测硬件并发
 */
async function autoDetectExample() {
    // 传入 0 表示自动检测硬件并发数
    await photon.PhotonImage.init_thread_pool(0);
    
    console.log('线程池已自动初始化，使用硬件默认线程数');
}

/**
 * 示例 3: 完整的图像处理流程
 */
async function fullProcessingExample() {
    // 1. 初始化线程池
    await photon.PhotonImage.init_thread_pool(4);
    
    // 2. 加载图像
    const imageFile = document.getElementById('imageInput').files[0];
    const arrayBuffer = await imageFile.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);
    
    // 3. 创建 PhotonImage
    const photonImage = new photon.PhotonImage(
        Array.from(uint8Array),
        800, // 宽度
        600  // 高度
    );
    
    // 4. 应用各种效果（这些操作可能会使用并行处理）
    
    // 转换为灰度图（使用 SIMD 优化）
    photon.grayscale(photonImage);
    
    // 调整对比度（使用 SIMD 优化）
    photon.adjust_contrast(photonImage, 30.0);
    
    // 应用滤镜（使用 SIMD 优化）
    photon.neue(photonImage);
    
    // 5. 获取处理后的像素数据
    const pixels = photonImage.get_raw_pixels();
    
    // 6. 将结果显示在 Canvas 上
    const canvas = document.getElementById('outputCanvas');
    const ctx = canvas.getContext('2d');
    const imageData = new ImageData(
        new Uint8ClampedArray(pixels),
        photonImage.get_width(),
        photonImage.get_height()
    );
    ctx.putImageData(imageData, 0, 0);
}

/**
 * 示例 4: 性能对比测试
 */
async function performanceBenchmark() {
    const imageFile = document.getElementById('imageInput').files[0];
    const arrayBuffer = await imageFile.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);
    
    // 测试不同的线程数
    const threadCounts = [1, 2, 4, 8];
    const results = [];
    
    for (const threads of threadCounts) {
        // 初始化线程池
        await photon.PhotonImage.init_thread_pool(threads);
        
        // 创建图像副本
        const photonImage = new photon.PhotonImage(
            Array.from(uint8Array),
            800,
            600
        );
        
        // 测量性能
        const startTime = performance.now();
        
        // 应用多个效果
        for (let i = 0; i < 10; i++) {
            photon.grayscale(photonImage);
            photon.adjust_contrast(photonImage, 30.0);
        }
        
        const endTime = performance.now();
        const duration = endTime - startTime;
        
        results.push({ threads, duration });
        console.log(`${threads} 线程: ${duration.toFixed(2)}ms`);
    }
    
    // 显示结果
    console.table(results);
}

/**
 * 示例 5: 使用 Web Worker 进行后台处理
 */
function workerExample() {
    // 创建 Web Worker
    const worker = new Worker('image-worker.js');
    
    // 发送图像数据进行处理
    worker.postMessage({
        type: 'process-image',
        imageData: imageArrayBuffer,
        width: 800,
        height: 600
    });
    
    // 接收处理结果
    worker.onmessage = function(event) {
        const { processedImageData } = event.data;
        
        // 显示处理后的图像
        displayImage(processedImageData);
    };
}

/**
 * 示例 6: 实时图像处理（视频流）
 */
async function realTimeVideoProcessing() {
    // 初始化线程池
    await photon.PhotonImage.init_thread_pool(4);
    
    // 获取视频流
    const video = document.getElementById('video');
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    
    // 处理视频帧
    function processFrame() {
        // 绘制当前帧到 canvas
        ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
        
        // 获取像素数据
        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        
        // 创建 PhotonImage
        const photonImage = new photon.PhotonImage(
            Array.from(imageData.data),
            canvas.width,
            canvas.height
        );
        
        // 应用效果（使用 SIMD 和并行处理）
        photon.grayscale(photonImage);
        photon.threshold(photonImage, 128);
        
        // 更新 canvas
        const processedPixels = photonImage.get_raw_pixels();
        const processedImageData = new ImageData(
            new Uint8ClampedArray(processedPixels),
            canvas.width,
            canvas.height
        );
        ctx.putImageData(processedImageData, 0, 0);
        
        // 请求下一帧
        requestAnimationFrame(processFrame);
    }
    
    // 启动处理
    video.addEventListener('play', () => {
        processFrame();
    });
}

/**
 * 最佳实践
 */

// 1. 在应用启动时初始化线程池
window.addEventListener('load', async () => {
    await photon.PhotonImage.init_thread_pool(4);
    console.log('Photon WASM 线程池已准备就绪');
});

// 2. 根据设备性能调整线程数
function getOptimalThreadCount() {
    // 检测硬件并发数
    const concurrency = navigator.hardwareConcurrency || 4;
    
    // 对于低端设备，使用较少的线程
    if (concurrency <= 2) {
        return 1;
    }
    
    // 对于高端设备，可以使用更多线程
    if (concurrency >= 8) {
        return 4;
    }
    
    // 默认使用 2 个线程
    return 2;
}

// 3. 错误处理
async function safeInitThreadPool() {
    try {
        const threadCount = getOptimalThreadCount();
        await photon.PhotonImage.init_thread_pool(threadCount);
        console.log(`线程池初始化成功，使用 ${threadCount} 个线程`);
    } catch (error) {
        console.error('线程池初始化失败:', error);
        // 降级到单线程模式
        console.log('使用单线程模式');
    }
}

// 导出函数供外部使用
export {
    basicExample,
    autoDetectExample,
    fullProcessingExample,
    performanceBenchmark,
    realTimeVideoProcessing,
    getOptimalThreadCount,
    safeInitThreadPool
};

// HTML 示例
/*
<!DOCTYPE html>
<html>
<head>
    <title>Photon WASM 并行处理示例</title>
</head>
<body>
    <input type="file" id="imageInput" accept="image/*">
    <canvas id="outputCanvas"></canvas>
    <video id="video" autoplay></video>
    <canvas id="canvas"></canvas>
    
    <script type="module">
        import * as photon from './pkg/photon_wasm.js';
        import {
            safeInitThreadPool,
            fullProcessingExample
        } from './wasm-parallel-usage.js';
        
        // 初始化
        safeInitThreadPool().then(() => {
            // 设置图像处理
            document.getElementById('imageInput').addEventListener('change', fullProcessingExample);
        });
    </script>
</body>
</html>
*/