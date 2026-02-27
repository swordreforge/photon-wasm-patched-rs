# 图层高级功能 API 参考

本文档介绍了图层系统新增的高级功能，包括智能对象、噪点调整、明度调整、色彩空间转换和透视变换。

## 目录

- [智能对象 (Smart Objects)](#智能对象-smart-objects)
- [噪点调整 (Noise Adjustment)](#噪点调整-noise-adjustment)
- [明度调整 (Lightness Adjustment)](#明度调整-lightness-adjustment)
- [色彩空间转换 (Color Space Conversion)](#色彩空间转换-color-space-conversion)
- [透视变换 (Perspective Transform)](#透视变换-perspective-transform)

---

## 智能对象 (Smart Objects)

智能对象允许对图层进行非破坏性编辑。所有效果调整都会被记录，可以随时重置到原始状态。

### 属性

#### `smartObject: boolean`
获取或设置图层是否为智能对象。

```typescript
// 启用智能对象
layer.set_smartObject(true);
layer.smartObject; // true

// 禁用智能对象
layer.set_smartObject(false);
```

### 方法

#### `reset_smart_object(): boolean`
重置智能对象到原始状态，清除所有效果参数。

```typescript
layer.reset_smart_object();
```

#### `update_smart_object_original(): boolean`
更新智能对象的原始图像。在编辑内容后调用此方法保存新的原始状态。

```typescript
layer.update_smart_object_original();
```

### 工作流程

```typescript
// 1. 创建图层
const layerId = layerStack.add_layer("My Layer");

// 2. 获取图层
const layer = layerStack.get_layer(layerId);

// 3. 启用智能对象
layer.set_smartObject(true);

// 4. 应用各种效果（非破坏性）
layer.apply_noise(0.3);
layer.adjust_lightness(0.2, ColorSpace.Hsl);

// 5. 查看结果
render();

// 6. 如果不满意，可以重置
layer.reset_smart_object();

// 7. 或者修改效果参数（会重新应用所有效果）
layer.adjust_lightness(-0.1, ColorSpace.Hsl);
```

---

## 噪点调整 (Noise Adjustment)

为图层添加各种类型的噪点效果。

### 方法

#### `apply_noise(strength: number): boolean`
添加随机噪点到图层。

**参数:**
- `strength`: 噪点强度 (0.0 - 1.0)

```typescript
layer.apply_noise(0.5);
```

#### `apply_color_noise(r_factor: number, g_factor: number, b_factor: number, strength: number): boolean`
添加彩色噪点到图层，每个颜色通道可以独立控制。

**参数:**
- `r_factor`: 红色通道因子 (0.0 - 2.0)
- `g_factor`: 绿色通道因子 (0.0 - 2.0)
- `b_factor`: 蓝色通道因子 (0.0 - 2.0)
- `strength`: 噪点强度 (0.0 - 1.0)

```typescript
layer.apply_color_noise(1.0, 1.0, 0.5, 0.3);
```

#### `apply_pink_noise(): boolean`
添加粉色噪点（1/f 噪声）到图层。

```typescript
layer.apply_pink_noise();
```

#### `clear_noise(): boolean`
清除噪点效果（仅智能对象支持）。

```typescript
layer.clear_noise();
```

### 智能对象行为

对于智能对象，每次调用 `apply_noise` 系列方法时：
1. 保存噪点参数
2. 从原始图像重新应用所有效果
3. 支持使用 `clear_noise()` 清除效果

对于普通图层：
1. 直接修改图像数据
2. 无法撤销

---

## 明度调整 (Lightness Adjustment)

在不同色彩空间中调整图层的明度。

### 方法

#### `adjust_lightness(level: number, color_space: ColorSpace): boolean`
调整图层明度。

**参数:**
- `level`: 明度调整值 (-1.0 到 1.0，负值降低，正值提高)
- `color_space`: 使用的色彩空间

```typescript
// 提高 20% 明度（HSL 空间）
layer.adjust_lightness(0.2, ColorSpace.Hsl);

// 降低 30% 明度（LCH 空间）
layer.adjust_lightness(-0.3, ColorSpace.Lch);
```

#### `clear_lightness(): boolean`
清除明度调整（仅智能对象支持）。

```typescript
layer.clear_lightness();
```

### 支持的色彩空间

- `ColorSpace.Hsl`: HSL (Hue, Saturation, Lightness)
- `ColorSpace.Lch`: LCH (Lightness, Chroma, Hue)
- `ColorSpace.Hsv`: HSV (Hue, Saturation, Value)
- `ColorSpace.Hsluv`: HSLuv (更均匀的 HSL)

### 色彩空间对比

| 色彩空间 | 特点 | 适用场景 |
|---------|------|---------|
| HSL | 直观易懂 | 一般用途 |
| LCH | 感知均匀 | 颜色匹配 |
| HSV | 与 HSV 模型兼容 | 颜色选择 |
| HSLuv | 视觉感知均匀 | 设计工具 |

---

## 色彩空间转换 (Color Space Conversion)

在不同色彩空间之间转换图层。

### 方法

#### `convert_color_space(from: ColorSpace, to: ColorSpace): boolean`
转换图层色彩空间。

**参数:**
- `from`: 源色彩空间
- `to`: 目标色彩空间

```typescript
// 从 HSL 转换到 LCH
layer.convert_color_space(ColorSpace.Hsl, ColorSpace.Lch);
```

#### `clear_color_space_conversion(): boolean`
清除色彩空间转换（仅智能对象支持）。

```typescript
layer.clear_color_space_conversion();
```

### 使用场景

```typescript
// 场景 1: 在不同色彩空间间调整明度
layer.adjust_lightness(0.1, ColorSpace.Hsl);
layer.convert_color_space(ColorSpace.Hsl, ColorSpace.Lch);
layer.adjust_lightness(0.1, ColorSpace.Lch);

// 场景 2: 使用 LCH 进行精确颜色匹配
layer.convert_color_space(ColorSpace.Hsl, ColorSpace.Lch);
// 进行精确的颜色操作...
```

---

## 透视变换 (Perspective Transform)

应用透视变换到图层，实现三维透视效果。

### 属性

#### `perspectiveEnabled: boolean`
获取或设置是否启用透视变换。

```typescript
layer.set_perspective_enabled(true);
layer.perspectiveEnabled; // true
```

### 方法

#### `set_perspective_points(top_left_x: number, top_left_y: number, top_right_x: number, top_right_y: number, bottom_left_x: number, bottom_left_y: number, bottom_right_x: number, bottom_right_y: number): void`
设置透视变换的四个角点位置（相对坐标，范围 0.0 - 1.0）。

**参数:**
- `top_left_x`, `top_left_y`: 左上角点
- `top_right_x`, `top_right_y`: 右上角点
- `bottom_left_x`, `bottom_left_y`: 左下角点
- `bottom_right_x`, `bottom_right_y`: 右下角点

```typescript
// 创建透视效果
layer.set_perspective_points(
    0.1, 0.1,   // 左上角
    0.9, 0.1,   // 右上角
    0.0, 0.9,   // 左下角（向内收缩）
    1.0, 0.9    // 右下角（向外扩张）
);
layer.set_perspective_enabled(true);
```

#### `get_perspective_points(): Float32Array`
获取当前透视变换的四个角点位置。

```typescript
const points = layer.get_perspective_points();
// points: [tl_x, tl_y, tr_x, tr_y, bl_x, bl_y, br_x, br_y]
```

#### `reset_perspective(): void`
重置透视变换为默认状态（矩形）。

```typescript
layer.reset_perspective();
```

### 透视变换示例

```typescript
// 示例 1: 远景透视（模拟远处物体）
layer.set_perspective_points(
    0.3, 0.3,   // 远端较窄
    0.7, 0.3,
    0.0, 1.0,   // 近端较宽
    1.0, 1.0
);

// 示例 2: 倾斜效果
layer.set_perspective_points(
    0.0, 0.0,
    0.8, 0.2,   // 右上角下沉
    0.2, 0.8,   // 左下角上移
    1.0, 1.0
);

// 示例 3: 鱼眼效果（四角向内）
layer.set_perspective_points(
    0.1, 0.1,
    0.9, 0.1,
    0.1, 0.9,
    0.9, 0.9
);
```

### 与其他变换的组合

透视变换可以与缩放、旋转、平移等变换组合使用：

```typescript
// 先设置位置和缩放
layer.set_position(100, 50);
layer.set_scale(0.8);

// 然后应用透视
layer.set_perspective_points(0.2, 0.2, 0.8, 0.2, 0.0, 0.9, 1.0, 0.9);
layer.set_perspective_enabled(true);

// 最后渲染
render();
```

---

## 完整示例

```typescript
import { LayerStack, Layer, ColorSpace } from './pkg/photon_wasm.js';

// 创建图层堆栈
const layerStack = new LayerStack(800, 600);

// 添加图层
const layerId = layerStack.add_layer_from_pixels(imageData, "Photo");
const layer = layerStack.get_layer(layerId);

// 启用智能对象
layer.set_smartObject(true);

// 组合多种效果
layer.apply_noise(0.2);                    // 添加轻微噪点
layer.adjust_lightness(0.15, ColorSpace.Lch);  // 在 LCH 空间提高明度
layer.convert_color_space(ColorSpace.Hsl, ColorSpace.Lch);  // 转换色彩空间

// 应用透视变换
layer.set_perspective_points(0.2, 0.2, 0.8, 0.2, 0.0, 0.9, 1.0, 0.9);
layer.set_perspective_enabled(true);

// 调整位置和缩放
layer.set_position(50, 30);
layer.set_scale(0.9);

// 渲染
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const composite = layerStack.render_composite();
const imageData = ctx.createImageData(800, 600);
imageData.data.set(new Uint8Array(composite.get_bytes()));
ctx.putImageData(imageData, 0, 0);

// 如果需要重置
// layer.reset_smart_object();
// render();
```

---

## 性能建议

1. **智能对象**: 虽然智能对象提供非破坏性编辑，但在频繁调整时会有性能开销。建议在最终确定效果后禁用智能对象。

2. **透视变换**: 透视变换计算量较大，建议只在必要时启用。

3. **批量操作**: 当需要调整多个图层时，使用批量方法可以提高性能。

4. **增量渲染**: 使用 `incremental_render()` 方法只重新渲染有变化的区域。

---

## 注意事项

1. **坐标系统**: 透视变换使用相对坐标 (0.0 - 1.0)，相对于图层的边界。

2. **效果顺序**: 对于智能对象，效果的顺序很重要。调整明度后再添加噪点，效果会不同。

3. **色彩空间**: 不同色彩空间调整明度的视觉效果不同，LCH 通常提供更符合人眼感知的结果。

4. **透视限制**: 透视变换不支持完全反转或极端扭曲，可能会导致渲染错误。

---

## 相关文档

- [图层基础 API](./LAYER_API_REFERENCE.md)
- [图层系统使用指南](./图层系统快速使用指南.md)
- [图层系统第二阶段使用指南](./图层系统第二阶段使用指南.md)