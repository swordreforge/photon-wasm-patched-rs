/**
 * 图层高级功能测试脚本
 * 测试智能对象、噪点调整、明度调整、色彩空间转换和透视变换
 */

const { LayerStack, Layer, ColorSpace } = require('./pkg/photon_wasm.js');

function log(message) {
    console.log(`[测试] ${message}`);
}

function assert(condition, message) {
    if (!condition) {
        throw new Error(`断言失败: ${message}`);
    }
}

// 测试智能对象功能
function testSmartObject() {
    log('测试智能对象功能...');
    
    const stack = new LayerStack(400, 300);
    const layerId = stack.add_layer("Test Layer");
    const layer = stack.get_layer(layerId);
    
    // 设置一些像素数据
    const pixels = new Uint8Array(400 * 300 * 4);
    for (let i = 0; i < pixels.length; i += 4) {
        pixels[i] = 100;     // R
        pixels[i + 1] = 150; // G
        pixels[i + 2] = 200; // B
        pixels[i + 3] = 255; // A
    }
    layer.set_pixels(pixels);
    
    // 启用智能对象
    assert(layer.smartObject === false, '初始状态不应为智能对象');
    layer.set_smartObject(true);
    assert(layer.smartObject === true, '应该启用智能对象');
    
    // 应用效果
    const success = layer.apply_noise(0.3);
    assert(success === true, '应该成功应用噪点');
    
    // 重置到原始状态
    const resetSuccess = layer.reset_smart_object();
    assert(resetSuccess === true, '应该成功重置');
    
    log('✓ 智能对象功能测试通过');
}

// 测试噪点调整
function testNoiseAdjustment() {
    log('测试噪点调整...');
    
    const stack = new LayerStack(400, 300);
    const layerId = stack.add_layer("Noise Test");
    const layer = stack.get_layer(layerId);
    
    // 设置像素数据
    const pixels = new Uint8Array(400 * 300 * 4);
    for (let i = 0; i < pixels.length; i += 4) {
        pixels[i] = 128;
        pixels[i + 1] = 128;
        pixels[i + 2] = 128;
        pixels[i + 3] = 255;
    }
    layer.set_pixels(pixels);
    
    // 测试随机噪点
    let success = layer.apply_noise(0.5);
    assert(success === true, '应该成功应用随机噪点');
    
    // 测试彩色噪点
    success = layer.apply_color_noise(1.0, 0.5, 0.3, 0.4);
    assert(success === true, '应该成功应用彩色噪点');
    
    // 测试粉色噪点
    success = layer.apply_pink_noise();
    assert(success === true, '应该成功应用粉色噪点');
    
    log('✓ 噪点调整测试通过');
}

// 测试明度调整
function testLightnessAdjustment() {
    log('测试明度调整...');
    
    const stack = new LayerStack(400, 300);
    const layerId = stack.add_layer("Lightness Test");
    const layer = stack.get_layer(layerId);
    
    // 设置像素数据（中等亮度）
    const pixels = new Uint8Array(400 * 300 * 4);
    for (let i = 0; i < pixels.length; i += 4) {
        pixels[i] = 128;
        pixels[i + 1] = 128;
        pixels[i + 2] = 128;
        pixels[i + 3] = 255;
    }
    layer.set_pixels(pixels);
    
    // 测试不同色彩空间的明度调整
    const colorSpaces = [ColorSpace.Hsl, ColorSpace.Lch, ColorSpace.Hsv, ColorSpace.Hsluv];
    
    for (const colorSpace of colorSpaces) {
        const success = layer.adjust_lightness(0.2, colorSpace);
        assert(success === true, `应该成功调整明度 (${colorSpace})`);
        
        const success2 = layer.adjust_lightness(-0.2, colorSpace);
        assert(success2 === true, `应该成功降低明度 (${colorSpace})`);
    }
    
    log('✓ 明度调整测试通过');
}

// 测试色彩空间转换
function testColorSpaceConversion() {
    log('测试色彩空间转换...');
    
    const stack = new LayerStack(400, 300);
    const layerId = stack.add_layer("Color Space Test");
    const layer = stack.get_layer(layerId);
    
    // 设置像素数据
    const pixels = new Uint8Array(400 * 300 * 4);
    for (let i = 0; i < pixels.length; i += 4) {
        pixels[i] = 200;
        pixels[i + 1] = 100;
        pixels[i + 2] = 50;
        pixels[i + 3] = 255;
    }
    layer.set_pixels(pixels);
    
    // 测试色彩空间转换
    const conversions = [
        [ColorSpace.Hsl, ColorSpace.Lch],
        [ColorSpace.Lch, ColorSpace.Hsv],
        [ColorSpace.Hsv, ColorSpace.Hsluv],
        [ColorSpace.Hsluv, ColorSpace.Hsl]
    ];
    
    for (const [from, to] of conversions) {
        const success = layer.convert_color_space(from, to);
        assert(success === true, `应该成功转换 ${from} -> ${to}`);
    }
    
    log('✓ 色彩空间转换测试通过');
}

// 测试透视变换
function testPerspectiveTransform() {
    log('测试透视变换...');
    
    const stack = new LayerStack(400, 300);
    const layerId = stack.add_layer("Perspective Test");
    const layer = stack.get_layer(layerId);
    
    // 设置像素数据
    const pixels = new Uint8Array(400 * 300 * 4);
    for (let i = 0; i < pixels.length; i += 4) {
        pixels[i] = 255;
        pixels[i + 1] = 0;
        pixels[i + 2] = 0;
        pixels[i + 3] = 255;
    }
    layer.set_pixels(pixels);
    
    // 测试设置透视点
    layer.set_perspective_points(
        0.1, 0.1,   // 左上
        0.9, 0.1,   // 右上
        0.0, 0.9,   // 左下
        1.0, 0.9    // 右下
    );
    
    // 启用透视
    layer.set_perspective_enabled(true);
    assert(layer.perspective_enabled === true, '应该启用透视变换');
    
    // 获取透视点
    const points = layer.get_perspective_points();
    assert(points.length === 8, '应该返回8个值');
    
    // 重置透视
    layer.reset_perspective();
    assert(layer.perspective_enabled === false, '应该禁用透视变换');
    
    log('✓ 透视变换测试通过');
}

// 测试智能对象的效果清除功能
function testSmartObjectEffectClearing() {
    log('测试智能对象效果清除...');
    
    const stack = new LayerStack(400, 300);
    const layerId = stack.add_layer("Smart Object Clear Test");
    const layer = stack.get_layer(layerId);
    
    // 设置像素数据
    const pixels = new Uint8Array(400 * 300 * 4);
    for (let i = 0; i < pixels.length; i += 4) {
        pixels[i] = 128;
        pixels[i + 1] = 128;
        pixels[i + 2] = 128;
        pixels[i + 3] = 255;
    }
    layer.set_pixels(pixels);
    
    // 启用智能对象
    layer.set_smartObject(true);
    
    // 应用多种效果
    layer.apply_noise(0.3);
    layer.adjust_lightness(0.2, ColorSpace.Hsl);
    layer.convert_color_space(ColorSpace.Hsl, ColorSpace.Lch);
    
    // 清除单个效果
    let success = layer.clear_noise();
    assert(success === true, '应该成功清除噪点');
    
    success = layer.clear_lightness();
    assert(success === true, '应该成功清除明度调整');
    
    success = layer.clear_color_space_conversion();
    assert(success === true, '应该成功清除色彩空间转换');
    
    // 重置所有效果
    success = layer.reset_smart_object();
    assert(success === true, '应该成功重置所有效果');
    
    log('✓ 智能对象效果清除测试通过');
}

// 测试组合效果
function testCombinedEffects() {
    log('测试组合效果...');
    
    const stack = new LayerStack(400, 300);
    const layerId = stack.add_layer("Combined Effects Test");
    const layer = stack.get_layer(layerId);
    
    // 设置像素数据（渐变）
    const pixels = new Uint8Array(400 * 300 * 4);
    for (let y = 0; y < 300; y++) {
        for (let x = 0; x < 400; x++) {
            const idx = (y * 400 + x) * 4;
            pixels[idx] = Math.floor((x / 400) * 255);
            pixels[idx + 1] = Math.floor((y / 300) * 255);
            pixels[idx + 2] = 128;
            pixels[idx + 3] = 255;
        }
    }
    layer.set_pixels(pixels);
    
    // 启用智能对象
    layer.set_smartObject(true);
    
    // 组合多种效果
    layer.apply_noise(0.15);
    layer.adjust_lightness(0.1, ColorSpace.Lch);
    layer.convert_color_space(ColorSpace.Hsl, ColorSpace.Lch);
    
    // 设置位置和缩放
    layer.set_position(20, 10);
    layer.set_scale(0.9);
    
    // 应用透视
    layer.set_perspective_points(0.15, 0.15, 0.85, 0.15, 0.05, 0.9, 0.95, 0.9);
    layer.set_perspective_enabled(true);
    
    // 渲染
    const composite = stack.render_composite();
    assert(composite !== null, '应该成功渲染组合效果');
    
    log('✓ 组合效果测试通过');
}

// 运行所有测试
async function runAllTests() {
    log('开始运行所有测试...\n');
    
    try {
        testSmartObject();
        testNoiseAdjustment();
        testLightnessAdjustment();
        testColorSpaceConversion();
        testPerspectiveTransform();
        testSmartObjectEffectClearing();
        testCombinedEffects();
        
        log('\n✅ 所有测试通过！');
    } catch (error) {
        log(`\n❌ 测试失败: ${error.message}`);
        console.error(error);
        process.exit(1);
    }
}

// 运行测试
runAllTests();
