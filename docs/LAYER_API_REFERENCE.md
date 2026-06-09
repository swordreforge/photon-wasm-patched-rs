# Layer API 使用说明

## 重要提示

WASM 绑定将某些属性导出为 **JavaScript 属性（properties）** 而不是方法。请注意以下区别：

## 正确的 API 使用方式

### 1. 位置变换

**错误：**
```javascript
layer.set_position_x(100);  // ❌ 这个方法不存在
layer.position_x();         // ❌ 这不是 getter
```

**正确：**
```javascript
// 使用 set_position 方法设置位置（同时设置 X 和 Y）
layer.set_position(100, 50);

// 使用属性直接访问和修改
layer.positionX = 100;  // ✅ 设置 X 位置
const x = layer.positionX;  // ✅ 获取 X 位置

layer.positionY = 50;   // ✅ 设置 Y 位置
const y = layer.positionY;  // ✅ 获取 Y 位置
```

### 2. 缩放变换

**错误：**
```javascript
layer.set_scale_x(1.5);   // ❌ 这个方法不存在
layer.set_scale_y(1.5);   // ❌ 这个方法不存在
```

**正确：**
```javascript
// 使用 set_scale 方法进行均匀缩放
layer.set_scale(1.5);

// 使用属性直接访问和修改
layer.scaleX = 1.5;  // ✅ 设置 X 缩放
const scaleX = layer.scaleX;  // ✅ 获取 X 缩放

layer.scaleY = 1.5;  // ✅ 设置 Y 缩放
const scaleY = layer.scaleY;  // ✅ 获取 Y 缩放
```

### 3. 旋转变换

**错误：**
```javascript
layer.set_rotation_degrees(30);  // ❌ 这个方法不存在
layer.rotation_degrees();        // ❌ 这不是 getter
```

**正确：**
```javascript
// 使用属性直接访问和修改
layer.rotation = 30;  // ✅ 设置旋转角度（度）
const angle = layer.rotation;  // ✅ 获取旋转角度（度）
```

### 4. 翻转变换

**错误：**
```javascript
layer.set_flip_horizontal(true);  // ❌ 这个方法不存在
layer.set_flip_vertical(true);    // ❌ 这个方法不存在
```

**正确：**
```javascript
// 使用属性直接访问和修改
layer.flipHorizontal = true;  // ✅ 设置水平翻转
const flipH = layer.flipHorizontal;  // ✅ 获取水平翻转状态

layer.flipVertical = true;    // ✅ 设置垂直翻转
const flipV = layer.flipVertical;    // ✅ 获取垂直翻转状态
```

### 5. 变换原点

**错误：**
```javascript
layer.set_origin_x(0.5);  // ❌ 这个方法不存在
layer.set_origin_y(0.5);  // ❌ 这个方法不存在
```

**正确：**
```javascript
// 使用 set_origin 方法设置原点（同时设置 X 和 Y）
layer.set_origin(0.5, 0.5);

// 使用属性直接访问和修改
layer.originX = 0.5;  // ✅ 设置 X 原点
const originX = layer.originX;  // ✅ 获取 X 原点

layer.originY = 0.5;  // ✅ 设置 Y 原点
const originY = layer.originY;  // ✅ 获取 Y 原点
```

### 6. 其他属性

```javascript
// 混合模式
layer.blend_mode = BlendMode.Screen;  // ✅ 使用属性赋值
const mode = layer.blend_mode;        // ✅ 使用属性读取

// 不透明度
layer.opacity = 128;  // ✅ 使用属性赋值 (0-255)
const opacity = layer.opacity;  // ✅ 使用属性读取

// 可见性
layer.visible = true;  // ✅ 使用属性赋值
const visible = layer.visible;  // ✅ 使用属性读取

// 锁定状态
layer.locked = true;  // ✅ 使用属性赋值
const locked = layer.locked;  // ✅ 使用属性读取

// 图层名称
layer.name = "New Name";  // ✅ 使用属性赋值
const name = layer.name;  // ✅ 使用属性读取
```

## 完整示例

```javascript
import { LayerStack, BlendMode } from './pkg/photon_wasm.js';

// 创建图层堆栈
const stack = new LayerStack(800, 600);

// 添加图层
const layerId = stack.add_layer_from_pixels(pixels, 'My Layer');
const layer = stack.get_layer(layerId);

// 设置变换
layer.positionX = 100;      // X 位置
layer.positionY = 50;       // Y 位置
layer.scaleX = 1.5;         // X 缩放
layer.scaleY = 1.5;         // Y 缩放
layer.rotation = 30;        // 旋转角度（度）
layer.flipHorizontal = true; // 水平翻转
layer.flipVertical = false;  // 垂直翻转

// 设置属性
layer.blend_mode = BlendMode.Screen;  // 混合模式
layer.opacity = 200;                   // 不透明度 (0-255)
layer.visible = true;                  // 可见性

// 获取属性
const x = layer.positionX;
const y = layer.positionY;
const scaleX = layer.scaleX;
const scaleY = layer.scaleY;
const rotation = layer.rotation;
const flipH = layer.flipHorizontal;
const flipV = layer.flipVertical;
const opacity = layer.opacity;
const visible = layer.visible;

// 渲染
const result = stack.render_composite();
```

## 方法 vs 属性总结

| 功能 | 方法 | 属性 |
|------|------|------|
| 设置位置 (X+Y) | `set_position(x, y)` | - |
| 获取位置 X | - | `positionX` |
| 设置位置 X | - | `positionX = value` |
| 获取位置 Y | - | `positionY` |
| 设置位置 Y | - | `positionY = value` |
| 设置缩放 (均匀) | `set_scale(scale)` | - |
| 获取缩放 X | - | `scaleX` |
| 设置缩放 X | - | `scaleX = value` |
| 获取缩放 Y | - | `scaleY` |
| 设置缩放 Y | - | `scaleY = value` |
| 获取旋转角度 | - | `rotation` |
| 设置旋转角度 | - | `rotation = value` |
| 获取水平翻转 | - | `flipHorizontal` |
| 设置水平翻转 | - | `flipHorizontal = value` |
| 获取垂直翻转 | - | `flipVertical` |
| 设置垂直翻转 | - | `flipVertical = value` |
| 设置原点 (X+Y) | `set_origin(x, y)` | - |
| 获取原点 X | - | `originX` |
| 设置原点 X | - | `originX = value` |
| 获取原点 Y | - | `originY` |
| 设置原点 Y | - | `originY = value` |
| 设置混合模式 | - | `blend_mode = value` |
| 获取混合模式 | - | `blend_mode` |
| 设置不透明度 | - | `opacity = value` |
| 获取不透明度 | - | `opacity` |
| 设置可见性 | - | `visible = value` |
| 获取可见性 | - | `visible` |
| 设置锁定状态 | - | `locked = value` |
| 获取锁定状态 | - | `locked` |
| 设置名称 | - | `name = value` |
| 获取名称 | - | `name` |

## 常见错误

### 错误 1: 使用不存在的方法

```javascript
layer.set_rotation_degrees(30);  // ❌ 错误
```

**修复：**
```javascript
layer.rotation = 30;  // ✅ 正确
```

### 错误 2: 使用方法调用属性

```javascript
layer.positionX();  // ❌ 错误
```

**修复：**
```javascript
const x = layer.positionX;  // ✅ 正确
```

### 错误 3: 使用下划线命名

```javascript
layer.position_x = 100;  // ❌ 错误
layer.rotation_degrees = 30;  // ❌ 错误
```

**修复：**
```javascript
layer.positionX = 100;  // ✅ 正确
layer.rotation = 30;  // ✅ 正确
```

## TypeScript 类型定义参考

```typescript
export class Layer {
    // 方法
    set_position(x: number, y: number): void;
    set_scale(scale: number): void;
    set_origin(origin_x: number, origin_y: number): void;
    reset_transform(): void;

    // 属性（getter/setter）
    positionX: number;
    positionY: number;
    scaleX: number;
    scaleY: number;
    rotation: number;
    flipHorizontal: boolean;
    flipVertical: boolean;
    originX: number;
    originY: number;
    blend_mode: BlendMode;
    opacity: number;
    visible: boolean;
    locked: boolean;
    name: string;

    // 只读属性
    readonly id: number;
    readonly width: number;
    readonly height: number;
}
```