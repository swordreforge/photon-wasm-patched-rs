# Photon 取色器 API 文档

## 概述

Photon WASM 库现在提供了完整的取色器 API，允许你从图像中提取颜色信息，包括单点颜色、区域平均颜色、主色调和调色板生成。

## 可用的 API 方法

### 1. 获取单点颜色

```javascript
const color = processor.get_pixel_color(x, y);
// 返回: Uint8Array [r, g, b, a] 或 null
```

**参数:**
- `x` (number): X 坐标 (0 到 width-1)
- `y` (number): Y 坐标 (0 到 height-1)

**返回值:**
- 如果坐标有效，返回包含 RGBA 值的 Uint8Array
- 如果坐标超出范围，返回 null

**示例:**
```javascript
const color = processor.get_pixel_color(100, 100);
if (color) {
  const [r, g, b, a] = color;
  console.log(`RGB: ${r}, ${g}, ${b}, ${a}`);
}
```

### 2. 获取单点颜色的十六进制表示

```javascript
const hex = processor.get_pixel_color_hex(x, y, includeAlpha);
// 返回: string 或 null
```

**参数:**
- `x` (number): X 坐标
- `y` (number): Y 坐标
- `includeAlpha` (boolean): 是否包含 alpha 通道

**返回值:**
- 如果坐标有效，返回十六进制颜色字符串 (如 "#ff0000" 或 "#ff0000ff")
- 如果坐标超出范围，返回 null

**示例:**
```javascript
const hex = processor.get_pixel_color_hex(100, 100, false);
console.log(`Hex: ${hex}`); // 输出: #ff0000
```

### 3. 获取单点亮度

```javascript
const brightness = processor.get_pixel_brightness(x, y);
// 返回: number (0-255) 或 null
```

**参数:**
- `x` (number): X 坐标
- `y` (number): Y 坐标

**返回值:**
- 如果坐标有效，返回亮度值 (0-255)
- 如果坐标超出范围，返回 null

**示例:**
```javascript
const brightness = processor.get_pixel_brightness(100, 100);
console.log(`Brightness: ${brightness}`);
```

### 4. 获取区域平均颜色

```javascript
const avgColor = processor.get_region_average_color(x, y, width, height);
// 返回: Uint8Array [r, g, b, a] 或 null
```

**参数:**
- `x` (number): 区域左上角 X 坐标
- `y` (number): 区域左上角 Y 坐标
- `width` (number): 区域宽度
- `height` (number): 区域高度

**返回值:**
- 如果区域有效，返回包含 RGBA 平均值的 Uint8Array
- 如果区域超出范围，返回 null

**示例:**
```javascript
const avgColor = processor.get_region_average_color(0, 0, 100, 100);
if (avgColor) {
  const [r, g, b, a] = avgColor;
  console.log(`Average RGB: ${r}, ${g}, ${b}, ${a}`);
}
```

### 5. 获取区域平均亮度

```javascript
const avgBrightness = processor.get_region_average_brightness(x, y, width, height);
// 返回: number (0-255) 或 null
```

**参数:**
- `x` (number): 区域左上角 X 坐标
- `y` (number): 区域左上角 Y 坐标
- `width` (number): 区域宽度
- `height` (number): 区域高度

**返回值:**
- 如果区域有效，返回平均亮度值 (0-255)
- 如果区域超出范围，返回 null

### 6. 获取整个图像的主色调

```javascript
const dominantColor = processor.get_dominant_color();
// 返回: Uint8Array [r, g, b, a]
```

**返回值:**
- 包含整个图像主色调 RGBA 值的 Uint8Array

**示例:**
```javascript
const dominantColor = processor.get_dominant_color();
const [r, g, b, a] = dominantColor;
console.log(`Dominant Color: RGB(${r}, ${g}, ${b}, ${a})`);
```

### 7. 获取区域主色调

```javascript
const regionDominant = processor.get_region_dominant_color(x, y, width, height);
// 返回: Uint8Array [r, g, b, a] 或 null
```

**参数:**
- `x` (number): 区域左上角 X 坐标
- `y` (number): 区域左上角 Y 坐标
- `width` (number): 区域宽度
- `height` (number): 区域高度

**返回值:**
- 如果区域有效，返回包含主色调 RGBA 值的 Uint8Array
- 如果区域超出范围，返回 null

**示例:**
```javascript
const regionDominant = processor.get_region_dominant_color(0, 0, 200, 200);
if (regionDominant) {
  const [r, g, b, a] = regionDominant;
  console.log(`Region Dominant: RGB(${r}, ${g}, ${b}, ${a})`);
}
```

### 8. 获取调色板

```javascript
const palette = processor.get_color_palette(numColors);
// 返回: Array<Uint8Array>，每个元素是 [r, g, b, a]
```

**参数:**
- `numColors` (number): 要提取的颜色数量 (建议 1-20)

**返回值:**
- 包含多个颜色数组的数组，每个颜色是 [r, g, b, a] 格式

**示例:**
```javascript
const palette = processor.get_color_palette(5);
palette.forEach((color, index) => {
  const [r, g, b, a] = color;
  console.log(`Color ${index + 1}: RGB(${r}, ${g}, ${b}, ${a})`);
});
```

## 使用示例

### 完整示例：点击取色

```javascript
import init, { ImageProcessor } from './photon_wasm.js';

async function initColorPicker() {
  await init();

  // 加载图像
  const canvas = document.getElementById('canvas');
  const ctx = canvas.getContext('2d');
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  
  const processor = new ImageProcessor(canvas.width, canvas.height, imageData.data);

  // 添加点击事件
  canvas.addEventListener('click', (e) => {
    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left) * (canvas.width / rect.width));
    const y = Math.floor((e.clientY - rect.top) * (canvas.height / rect.height));

    // 获取颜色
    const color = processor.get_pixel_color(x, y);
    if (color) {
      const [r, g, b, a] = color;
      
      // 更新颜色预览
      const preview = document.getElementById('colorPreview');
      preview.style.background = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
      
      // 显示颜色信息
      console.log(`Color: RGB(${r}, ${g}, ${b}, ${a})`);
      console.log(`Hex: #${[r, g, b].map(x => x.toString(16).padStart(2, '0')).join('')}`);
    }
  });
}

initColorPicker();
```

### 生成调色板

```javascript
async function generatePalette() {
  await init();

  const processor = new ImageProcessor(width, height, pixelData);
  const palette = processor.get_color_palette(10);

  // 显示调色板
  const paletteContainer = document.getElementById('palette');
  palette.forEach((color) => {
    const [r, g, b, a] = color;
    const colorDiv = document.createElement('div');
    colorDiv.style.background = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
    colorDiv.className = 'palette-color';
    paletteContainer.appendChild(colorDiv);
  });
}
```

## 演示页面

项目包含一个完整的取色器演示页面：

**文件位置:** `photon-master/webpack_demo/color-picker.html`

**功能包括:**
- 单点取色（点击图像任意位置）
- 区域平均颜色提取
- 整个图像主色调获取
- 区域主色调获取
- 调色板生成（可自定义颜色数量）
- 实时颜色预览
- RGB 和 Hex 格式显示

**如何运行:**

1. 确保已编译 WASM 库
2. 启动 HTTP 服务器:
```bash
python3 -m http.server 8080
```
3. 在浏览器中打开:
```
http://localhost:8080/photon-master/webpack_demo/color-picker.html
```

## 性能说明

- 所有取色器 API 都经过优化，适合大图像处理
- 调色板生成使用采样策略，在保持准确性的同时提高性能
- 主色调算法使用颜色量化，快速找出最频繁的颜色

## 注意事项

1. **坐标系统**: 坐标从 (0, 0) 开始，位于图像左上角
2. **边界检查**: 所有方法都会检查坐标和区域是否在图像范围内
3. **返回值类型**: 颜色值以 Uint8Array 形式返回，每个值范围是 0-255
4. **Alpha 通道**: 所有颜色都包含 alpha 通道，范围 0-255（255 表示完全不透明）

## 故障排除

### 导入错误

如果遇到模块导入错误，确保:
1. WASM 库已正确编译
2. 使用 HTTP 服务器（不是 file:// 协议）
3. 文件路径正确

### 坐标超出范围

如果方法返回 null，检查:
1. 坐标值是否在有效范围内
2. 图像宽度和高度是否正确
3. 是否有图像缩放导致坐标转换错误

### 性能问题

如果处理大图像时性能不佳:
1. 减少调色板颜色数量
2. 使用区域采样而不是全图像分析
3. 考虑缩小图像后再处理

## 技术支持

如有问题或建议，请：
- 查看 GitHub Issues
- 参考 photon-master/crate/examples/test_color_picker.rs 了解更多示例
- 阅读源代码注释获取详细信息