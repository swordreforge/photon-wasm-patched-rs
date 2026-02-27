use wasm_bindgen::prelude::*;

/// 颜色空间枚举，用于明度调整
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSpace {
    /// HSL 颜色空间
    Hsl,
    /// LCH 颜色空间
    Lch,
    /// HSV 颜色空间
    Hsv,
    /// HSLuv 颜色空间
    Hsluv,
}

/// 笔刷类型
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BrushType {
    /// 基础画笔
    Basic,
    /// 铅笔风格
    Pencil,
    /// 马克笔风格
    Marker,
    /// 水彩笔风格
    Watercolor,
    /// 橡皮擦
    Eraser,
}

/// 混合模式
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
    Exclusion,
    Lighten,
    Darken,
}

/// 笔刷配置
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct BrushConfig {
    /// 笔刷类型
    pub brush_type: BrushType,
    /// 基础宽度（像素）
    pub base_width: f32,
    /// 颜色（RGBA）
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub color_a: u8,
    /// 混合模式
    pub blend_mode: BlendMode,
    /// 平滑度（0.0 - 1.0）
    pub smoothness: f32,
    /// 压感强度（0.0 - 1.0）
    pub pressure_sensitivity: f32,
}

#[wasm_bindgen]
impl BrushConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(
        brush_type: BrushType,
        base_width: f32,
        color_r: u8,
        color_g: u8,
        color_b: u8,
        color_a: u8,
        blend_mode: BlendMode,
        smoothness: f32,
        pressure_sensitivity: f32,
    ) -> BrushConfig {
        BrushConfig {
            brush_type,
            base_width,
            color_r,
            color_g,
            color_b,
            color_a,
            blend_mode,
            smoothness,
            pressure_sensitivity,
        }
    }

    /// 创建默认基础画笔配置
    pub fn default_basic() -> BrushConfig {
        BrushConfig {
            brush_type: BrushType::Basic,
            base_width: 5.0,
            color_r: 0,
            color_g: 0,
            color_b: 0,
            color_a: 255,
            blend_mode: BlendMode::Normal,
            smoothness: 0.5,
            pressure_sensitivity: 0.5,
        }
    }
}

/// 笔划点
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct StrokePoint {
    /// X 坐标
    pub x: f32,
    /// Y 坐标
    pub y: f32,
    /// 压感（0.0 - 1.0）
    pub pressure: f32,
    /// 时间戳（毫秒）
    pub timestamp: u64,
}

#[wasm_bindgen]
impl StrokePoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, pressure: f32, timestamp: u64) -> StrokePoint {
        StrokePoint {
            x,
            y,
            pressure,
            timestamp,
        }
    }
}