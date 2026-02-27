use wasm_bindgen::prelude::*;
use photon_rs::{PhotonImage, multiple};
use crate::types::{BlendMode, ColorSpace};

/// 图层效果参数（用于非破坏性编辑）
#[derive(Clone, Debug)]
pub struct LayerEffectParams {
    // 噪点参数
    noise_strength: Option<f32>,
    noise_r_factor: Option<f32>,
    noise_g_factor: Option<f32>,
    noise_b_factor: Option<f32>,
    noise_type: Option<String>, // "random", "color", "pink"
    
    // 明度参数
    lightness_level: Option<f32>,
    lightness_color_space: Option<ColorSpace>,
    
    // 色彩空间参数
    color_space_from: Option<ColorSpace>,
    color_space_to: Option<ColorSpace>,
}

impl Default for LayerEffectParams {
    fn default() -> Self {
        LayerEffectParams {
            noise_strength: None,
            noise_r_factor: None,
            noise_g_factor: None,
            noise_b_factor: None,
            noise_type: None,
            lightness_level: None,
            lightness_color_space: None,
            color_space_from: None,
            color_space_to: None,
        }
    }
}

/// 图层结构体
/// 包含图像数据、变换、混合模式等属性
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Layer {
    id: usize,
    name: String,
    image: PhotonImage,
    visible: bool,
    locked: bool,
    opacity: u8,
    blend_mode: BlendMode,
    // 变换属性
    position_x: f32,  // X 位置（像素）
    position_y: f32,  // Y 位置（像素）
    scale_x: f32,     // X 缩放比例
    scale_y: f32,     // Y 缩放比例
    rotation_degrees: f32,  // 旋转角度（度）
    origin_x: f32,    // 变换原点 X（相对于图层中心，0-1）
    origin_y: f32,    // 变换原点 Y（相对于图层中心，0-1）
    // 外部变换中心（当使用外部变换中心时生效，None表示使用origin）
    transform_center_x: Option<f32>,  // 外部变换中心 X（画布坐标系）
    transform_center_y: Option<f32>,  // 外部变换中心 Y（画布坐标系）
    flip_horizontal: bool,  // 水平翻转
    flip_vertical: bool,    // 垂直翻转
    // 智能对象相关
    smart_object: bool,     // 是否为智能对象
    original_image: Option<PhotonImage>,  // 原始图像数据（智能对象用）
    effect_params: LayerEffectParams,     // 效果参数（非破坏性编辑）
    // 透视变换参数
    perspective_enabled: bool,  // 是否启用透视变换
    perspective_top_left: (f32, f32),     // 左上角点 (相对坐标 0-1)
    perspective_top_right: (f32, f32),    // 右上角点 (相对坐标 0-1)
    perspective_bottom_left: (f32, f32),  // 左下角点 (相对坐标 0-1)
    perspective_bottom_right: (f32, f32), // 右下角点 (相对坐标 0-1)
}

impl Layer {
    /// 创建新图层
    pub fn new(id: usize, name: String, width: u32, height: u32) -> Layer {
        // 创建透明背景图层
        let pixel_count = (width * height) as usize;
        let mut pixels = vec![0u8; pixel_count * 4];
        // 设置完全透明
        for i in (0..pixels.len()).step_by(4) {
            pixels[i + 3] = 0; // Alpha 通道设为 0
        }

        Layer {
            id,
            name,
            image: PhotonImage::new(pixels, width, height),
            visible: true,
            locked: false,
            opacity: 255,
            blend_mode: BlendMode::Normal,
            // 变换属性默认值
            position_x: 0.0,
            position_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation_degrees: 0.0,
            origin_x: 0.5,  // 默认中心点
            origin_y: 0.5,
            transform_center_x: None,
            transform_center_y: None,
            flip_horizontal: false,
            flip_vertical: false,
            // 智能对象相关
            smart_object: false,
            original_image: None,
            effect_params: LayerEffectParams::default(),
            // 透视变换相关
            perspective_enabled: false,
            perspective_top_left: (0.0, 0.0),
            perspective_top_right: (1.0, 0.0),
            perspective_bottom_left: (0.0, 1.0),
            perspective_bottom_right: (1.0, 1.0),
        }
    }

    /// 从现有图像创建图层
    pub fn from_image(id: usize, name: String, image: PhotonImage) -> Layer {
        Layer {
            id,
            name,
            image: image.clone(),
            visible: true,
            locked: false,
            opacity: 255,
            blend_mode: BlendMode::Normal,
            // 变换属性默认值
            position_x: 0.0,
            position_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation_degrees: 0.0,
            origin_x: 0.5,
            origin_y: 0.5,
            transform_center_x: None,
            transform_center_y: None,
            flip_horizontal: false,
            flip_vertical: false,
            // 智能对象相关
            smart_object: false,
            original_image: None,
            effect_params: LayerEffectParams::default(),
            // 透视变换相关
            perspective_enabled: false,
            perspective_top_left: (0.0, 0.0),
            perspective_top_right: (1.0, 0.0),
            perspective_bottom_left: (0.0, 1.0),
            perspective_bottom_right: (1.0, 1.0),
        }
    }
}

#[wasm_bindgen]
impl Layer {
    /// 获取图层 ID
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> usize {
        self.id
    }

    /// 获取图层名称
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// 设置图层名称
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// 获取可见性
    #[wasm_bindgen(getter)]
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// 设置可见性
    #[wasm_bindgen(setter)]
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// 获取锁定状态
    #[wasm_bindgen(getter)]
    pub fn locked(&self) -> bool {
        self.locked
    }

    /// 设置锁定状态
    #[wasm_bindgen(setter)]
    pub fn set_locked(&mut self, locked: bool) {
        self.locked = locked;
    }

    /// 获取不透明度 (0-255)
    #[wasm_bindgen(getter)]
    pub fn opacity(&self) -> u8 {
        self.opacity
    }

    /// 设置不透明度 (0-255)
    #[wasm_bindgen(setter)]
    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }

    /// 获取混合模式
    #[wasm_bindgen(getter)]
    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }

    /// 设置混合模式
    #[wasm_bindgen(setter)]
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.blend_mode = blend_mode;
    }

    /// 获取图层宽度
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.image.get_width()
    }

    /// 获取图层高度
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.image.get_height()
    }

    /// 获取图层的像素数据
    #[wasm_bindgen]
    pub fn get_pixels(&self) -> Vec<u8> {
        self.image.get_raw_pixels()
    }

    /// 设置图层的像素数据
    #[wasm_bindgen]
    pub fn set_pixels(&mut self, pixels: Vec<u8>) {
        self.image = PhotonImage::new(pixels, self.image.get_width(), self.image.get_height());
    }

    // ==================== 变换属性 ====================

    /// 获取位置 X
    #[wasm_bindgen(getter = positionX)]
    pub fn position_x(&self) -> f32 {
        self.position_x
    }

    /// 设置位置 X
    #[wasm_bindgen(setter = positionX)]
    pub fn set_position_x(&mut self, x: f32) {
        self.position_x = x;
    }

    /// 获取位置 Y
    #[wasm_bindgen(getter = positionY)]
    pub fn position_y(&self) -> f32 {
        self.position_y
    }

    /// 设置位置 Y
    #[wasm_bindgen(setter = positionY)]
    pub fn set_position_y(&mut self, y: f32) {
        self.position_y = y;
    }

    /// 设置位置
    #[wasm_bindgen]
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position_x = x;
        self.position_y = y;
    }

    /// 获取缩放 X
    #[wasm_bindgen(getter = scaleX)]
    pub fn scale_x(&self) -> f32 {
        self.scale_x
    }

    /// 设置缩放 X
    #[wasm_bindgen(setter = scaleX)]
    pub fn set_scale_x(&mut self, scale: f32) {
        self.scale_x = scale.max(0.01); // 防止除零
    }

    /// 获取缩放 Y
    #[wasm_bindgen(getter = scaleY)]
    pub fn scale_y(&self) -> f32 {
        self.scale_y
    }

    /// 设置缩放 Y
    #[wasm_bindgen(setter = scaleY)]
    pub fn set_scale_y(&mut self, scale: f32) {
        self.scale_y = scale.max(0.01);
    }

    /// 设置缩放（均匀缩放）
    #[wasm_bindgen]
    pub fn set_scale(&mut self, scale: f32) {
        let safe_scale = scale.max(0.01);
        self.scale_x = safe_scale;
        self.scale_y = safe_scale;
    }

    /// 获取旋转角度（度）
    #[wasm_bindgen(getter = rotation)]
    pub fn rotation_degrees(&self) -> f32 {
        self.rotation_degrees
    }

    /// 设置旋转角度（度）
    #[wasm_bindgen(setter = rotation)]
    pub fn set_rotation_degrees(&mut self, degrees: f32) {
        self.rotation_degrees = degrees % 360.0; // 归一化到 0-360
    }

    /// 获取变换原点 X（0-1，相对于图层中心）
    #[wasm_bindgen(getter = originX)]
    pub fn origin_x(&self) -> f32 {
        self.origin_x
    }

    /// 设置变换原点 X
    #[wasm_bindgen(setter = originX)]
    pub fn set_origin_x(&mut self, origin: f32) {
        self.origin_x = origin.clamp(0.0, 1.0);
    }

    /// 获取变换原点 Y（0-1，相对于图层中心）
    #[wasm_bindgen(getter = originY)]
    pub fn origin_y(&self) -> f32 {
        self.origin_y
    }

    /// 设置变换原点 Y
    #[wasm_bindgen(setter = originY)]
    pub fn set_origin_y(&mut self, origin: f32) {
        self.origin_y = origin.clamp(0.0, 1.0);
    }

    /// 设置变换原点
    #[wasm_bindgen]
    pub fn set_origin(&mut self, origin_x: f32, origin_y: f32) {
        self.origin_x = origin_x.clamp(0.0, 1.0);
        self.origin_y = origin_y.clamp(0.0, 1.0);
    }

    /// 获取水平翻转状态
    #[wasm_bindgen(getter = flipHorizontal)]
    pub fn flip_horizontal(&self) -> bool {
        self.flip_horizontal
    }

    /// 设置水平翻转
    #[wasm_bindgen(setter = flipHorizontal)]
    pub fn set_flip_horizontal(&mut self, flip: bool) {
        self.flip_horizontal = flip;
    }

    /// 获取垂直翻转状态
    #[wasm_bindgen(getter = flipVertical)]
    pub fn flip_vertical(&self) -> bool {
        self.flip_vertical
    }

    /// 设置垂直翻转
    #[wasm_bindgen(setter = flipVertical)]
    pub fn set_flip_vertical(&mut self, flip: bool) {
        self.flip_vertical = flip;
    }

    /// 重置所有变换为默认值
    #[wasm_bindgen]
    pub fn reset_transform(&mut self) {
        self.position_x = 0.0;
        self.position_y = 0.0;
        self.scale_x = 1.0;
        self.scale_y = 1.0;
        self.rotation_degrees = 0.0;
        self.origin_x = 0.5;
        self.origin_y = 0.5;
        self.transform_center_x = None;
        self.transform_center_y = None;
        self.flip_horizontal = false;
        self.flip_vertical = false;
    }

    /// 获取外部变换中心 X（画布坐标系）
    #[wasm_bindgen(getter = transformCenterX)]
    pub fn transform_center_x(&self) -> Option<f32> {
        self.transform_center_x
    }

    /// 设置外部变换中心 X
    #[wasm_bindgen(setter = transformCenterX)]
    pub fn set_transform_center_x(&mut self, center: Option<f32>) {
        self.transform_center_x = center;
    }

    /// 获取外部变换中心 Y（画布坐标系）
    #[wasm_bindgen(getter = transformCenterY)]
    pub fn transform_center_y(&self) -> Option<f32> {
        self.transform_center_y
    }

    /// 设置外部变换中心 Y
    #[wasm_bindgen(setter = transformCenterY)]
    pub fn set_transform_center_y(&mut self, center: Option<f32>) {
        self.transform_center_y = center;
    }

    /// 设置外部变换中心（画布坐标系）
    #[wasm_bindgen]
    pub fn set_transform_center(&mut self, center_x: Option<f32>, center_y: Option<f32>) {
        self.transform_center_x = center_x;
        self.transform_center_y = center_y;
    }

    /// 清除外部变换中心，恢复使用origin
    #[wasm_bindgen]
    pub fn clear_transform_center(&mut self) {
        self.transform_center_x = None;
        self.transform_center_y = None;
    }

    // ==================== 智能对象相关 ====================

    /// 获取是否为智能对象
    #[wasm_bindgen(getter = smartObject)]
    pub fn smart_object(&self) -> bool {
        self.smart_object
    }

    /// 设置是否为智能对象
    #[wasm_bindgen(setter = smartObject)]
    pub fn set_smart_object(&mut self, smart_object: bool) {
        self.smart_object = smart_object;
        if smart_object && self.original_image.is_none() {
            // 保存当前图像作为原始图像
            self.original_image = Some(self.image.clone());
        }
    }

    /// 重置智能对象到原始状态
    #[wasm_bindgen]
    pub fn reset_smart_object(&mut self) -> bool {
        if self.smart_object {
            if let Some(ref original) = self.original_image {
                self.image = original.clone();
                // 清除所有效果参数
                self.effect_params = LayerEffectParams::default();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// 更新智能对象的原始图像（在编辑内容后调用）
    #[wasm_bindgen]
    pub fn update_smart_object_original(&mut self) -> bool {
        if self.smart_object {
            self.original_image = Some(self.image.clone());
            true
        } else {
            false
        }
    }

    // ==================== 噪点调整 ====================

    /// 添加噪点到图层
    #[wasm_bindgen]
    pub fn apply_noise(&mut self, strength: f32) -> bool {
        if self.smart_object {
            // 保存参数
            self.effect_params.noise_strength = Some(strength);
            self.effect_params.noise_type = Some("random".to_string());
            // 从原始图像重新应用所有效果
            self.reapply_effects();
            true
        } else {
            // 直接应用
            apply_noise_to_image(&mut self.image, strength);
            true
        }
    }

    /// 添加彩色噪点到图层
    #[wasm_bindgen]
    pub fn apply_color_noise(&mut self, r_factor: f32, g_factor: f32, b_factor: f32, strength: f32) -> bool {
        if self.smart_object {
            // 保存参数
            self.effect_params.noise_r_factor = Some(r_factor);
            self.effect_params.noise_g_factor = Some(g_factor);
            self.effect_params.noise_b_factor = Some(b_factor);
            self.effect_params.noise_strength = Some(strength);
            self.effect_params.noise_type = Some("color".to_string());
            // 从原始图像重新应用所有效果
            self.reapply_effects();
            true
        } else {
            // 直接应用
            apply_color_noise_to_image(&mut self.image, r_factor, g_factor, b_factor, strength);
            true
        }
    }

    /// 添加粉色噪点到图层
    #[wasm_bindgen]
    pub fn apply_pink_noise(&mut self) -> bool {
        if self.smart_object {
            // 保存参数
            self.effect_params.noise_type = Some("pink".to_string());
            // 从原始图像重新应用所有效果
            self.reapply_effects();
            true
        } else {
            // 直接应用
            apply_pink_noise_to_image(&mut self.image);
            true
        }
    }

    /// 清除噪点效果
    #[wasm_bindgen]
    pub fn clear_noise(&mut self) -> bool {
        if self.smart_object {
            self.effect_params.noise_strength = None;
            self.effect_params.noise_r_factor = None;
            self.effect_params.noise_g_factor = None;
            self.effect_params.noise_b_factor = None;
            self.effect_params.noise_type = None;
            self.reapply_effects();
            true
        } else {
            false // 非智能对象无法撤销
        }
    }

    // ==================== 明度调整 ====================

    /// 调整图层明度
    #[wasm_bindgen]
    pub fn adjust_lightness(&mut self, level: f32, color_space: ColorSpace) -> bool {
        if self.smart_object {
            // 保存参数
            self.effect_params.lightness_level = Some(level);
            self.effect_params.lightness_color_space = Some(color_space);
            // 从原始图像重新应用所有效果
            self.reapply_effects();
            true
        } else {
            // 直接应用
            adjust_lightness_of_image(&mut self.image, level, color_space);
            true
        }
    }

    /// 清除明度调整
    #[wasm_bindgen]
    pub fn clear_lightness(&mut self) -> bool {
        if self.smart_object {
            self.effect_params.lightness_level = None;
            self.effect_params.lightness_color_space = None;
            self.reapply_effects();
            true
        } else {
            false
        }
    }

    // ==================== 色彩空间转换 ====================

    /// 转换图层色彩空间
    #[wasm_bindgen]
    pub fn convert_color_space(&mut self, from: ColorSpace, to: ColorSpace) -> bool {
        if self.smart_object {
            // 保存参数
            self.effect_params.color_space_from = Some(from);
            self.effect_params.color_space_to = Some(to);
            // 从原始图像重新应用所有效果
            self.reapply_effects();
            true
        } else {
            // 直接应用
            convert_image_color_space(&mut self.image, from, to);
            true
        }
    }

    /// 清除色彩空间转换
    #[wasm_bindgen]
    pub fn clear_color_space_conversion(&mut self) -> bool {
        if self.smart_object {
            self.effect_params.color_space_from = None;
            self.effect_params.color_space_to = None;
            self.reapply_effects();
            true
        } else {
            false
        }
    }

    // ==================== 透视变换 ====================

    /// 获取是否启用透视变换
    #[wasm_bindgen(getter = perspectiveEnabled)]
    pub fn perspective_enabled(&self) -> bool {
        self.perspective_enabled
    }

    /// 设置是否启用透视变换
    #[wasm_bindgen(setter = perspectiveEnabled)]
    pub fn set_perspective_enabled(&mut self, enabled: bool) {
        self.perspective_enabled = enabled;
    }

    /// 设置透视变换的四个角点（相对坐标 0-1）
    #[wasm_bindgen]
    pub fn set_perspective_points(
        &mut self,
        top_left_x: f32, top_left_y: f32,
        top_right_x: f32, top_right_y: f32,
        bottom_left_x: f32, bottom_left_y: f32,
        bottom_right_x: f32, bottom_right_y: f32,
    ) {
        self.perspective_top_left = (top_left_x, top_left_y);
        self.perspective_top_right = (top_right_x, top_right_y);
        self.perspective_bottom_left = (bottom_left_x, bottom_left_y);
        self.perspective_bottom_right = (bottom_right_x, bottom_right_y);
    }

    /// 获取透视变换的四个角点
    #[wasm_bindgen]
    pub fn get_perspective_points(&self) -> Vec<f32> {
        vec![
            self.perspective_top_left.0, self.perspective_top_left.1,
            self.perspective_top_right.0, self.perspective_top_right.1,
            self.perspective_bottom_left.0, self.perspective_bottom_left.1,
            self.perspective_bottom_right.0, self.perspective_bottom_right.1,
        ]
    }

    /// 重置透视变换为默认状态
    #[wasm_bindgen]
    pub fn reset_perspective(&mut self) {
        self.perspective_enabled = false;
        self.perspective_top_left = (0.0, 0.0);
        self.perspective_top_right = (1.0, 0.0);
        self.perspective_bottom_left = (0.0, 1.0);
        self.perspective_bottom_right = (1.0, 1.0);
    }

    // ==================== 内部方法 ====================

    /// 重新应用所有效果（智能对象内部使用）
    fn reapply_effects(&mut self) {
        if let Some(ref original) = self.original_image {
            self.image = original.clone();
            
            // 应用噪点
            if let Some(noise_type) = &self.effect_params.noise_type {
                match noise_type.as_str() {
                    "random" => {
                        if let Some(strength) = self.effect_params.noise_strength {
                            apply_noise_to_image(&mut self.image, strength);
                        }
                    }
                    "color" => {
                        if let (Some(r), Some(g), Some(b), Some(strength)) = (
                            self.effect_params.noise_r_factor,
                            self.effect_params.noise_g_factor,
                            self.effect_params.noise_b_factor,
                            self.effect_params.noise_strength,
                        ) {
                            apply_color_noise_to_image(&mut self.image, r, g, b, strength);
                        }
                    }
                    "pink" => {
                        apply_pink_noise_to_image(&mut self.image);
                    }
                    _ => {}
                }
            }
            
            // 应用明度调整
            if let (Some(level), Some(color_space)) = (
                self.effect_params.lightness_level,
                self.effect_params.lightness_color_space,
            ) {
                adjust_lightness_of_image(&mut self.image, level, color_space);
            }
            
            // 应用色彩空间转换
            if let (Some(from), Some(to)) = (
                self.effect_params.color_space_from,
                self.effect_params.color_space_to,
            ) {
                convert_image_color_space(&mut self.image, from, to);
            }
        }
    }
}

// ==================== 图像处理辅助函数 ====================

/// 给图像添加噪点
fn apply_noise_to_image(image: &mut PhotonImage, strength: f32) {
    let pixels = image.get_raw_pixels();
    let mut result = pixels.clone();
    
    for i in (0..result.len()).step_by(4) {
        if result[i + 3] > 0 { // 只处理非透明像素
            let noise = ((js_sys::Math::random() as f32 - 0.5) * 2.0 * strength) as i32;
            for c in 0..3 {
                result[i + c] = (result[i + c] as i32 + noise).clamp(0, 255) as u8;
            }
        }
    }
    
    *image = PhotonImage::new(result, image.get_width(), image.get_height());
}

/// 给图像添加彩色噪点
fn apply_color_noise_to_image(image: &mut PhotonImage, r_factor: f32, g_factor: f32, b_factor: f32, strength: f32) {
    let pixels = image.get_raw_pixels();
    let mut result = pixels.clone();
    
    for i in (0..result.len()).step_by(4) {
        if result[i + 3] > 0 {
            let r_noise = ((js_sys::Math::random() as f32 - 0.5) * 2.0 * strength * r_factor) as i32;
            let g_noise = ((js_sys::Math::random() as f32 - 0.5) * 2.0 * strength * g_factor) as i32;
            let b_noise = ((js_sys::Math::random() as f32 - 0.5) * 2.0 * strength * b_factor) as i32;
            
            result[i] = (result[i] as i32 + r_noise).clamp(0, 255) as u8;
            result[i + 1] = (result[i + 1] as i32 + g_noise).clamp(0, 255) as u8;
            result[i + 2] = (result[i + 2] as i32 + b_noise).clamp(0, 255) as u8;
        }
    }
    
    *image = PhotonImage::new(result, image.get_width(), image.get_height());
}

/// 给图像添加粉色噪点
fn apply_pink_noise_to_image(image: &mut PhotonImage) {
    let pixels = image.get_raw_pixels();
    let mut result = pixels.clone();
    let width = image.get_width() as usize;
    
    for y in 0..image.get_height() as usize {
        for x in 0..width {
            let i = (y * width + x) * 4;
            if result[i + 3] > 0 {
                // 1/f 噪声近似
                let freq = (x + y) as f32;
                let strength = 50.0 / (freq.sqrt() + 1.0);
                let noise = ((js_sys::Math::random() as f32 - 0.5) * 2.0 * strength) as i32;
                
                for c in 0..3 {
                    result[i + c] = (result[i + c] as i32 + noise).clamp(0, 255) as u8;
                }
            }
        }
    }
    
    *image = PhotonImage::new(result, image.get_width(), image.get_height());
}

/// 调整图像明度
fn adjust_lightness_of_image(image: &mut PhotonImage, level: f32, color_space: ColorSpace) {
    let pixels = image.get_raw_pixels();
    let mut result = pixels.clone();
    
    for i in (0..result.len()).step_by(4) {
        if result[i + 3] > 0 {
            let r = result[i] as f32 / 255.0;
            let g = result[i + 1] as f32 / 255.0;
            let b = result[i + 2] as f32 / 255.0;
            
            let (h, s, l, c) = match color_space {
                ColorSpace::Hsl => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    let c = 2.0 * s * l.min(1.0 - l);
                    (h, s, l, c)
                }
                ColorSpace::Lch => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    let c = 2.0 * s * l.min(1.0 - l);
                    (h, s, l, c)
                }
                ColorSpace::Hsv => {
                    let (h, s, v) = rgb_to_hsv(r, g, b);
                    let c = 2.0 * s * v.min(1.0 - v);
                    (h, s, v, c)
                }
                ColorSpace::Hsluv => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    let c = 2.0 * s * l.min(1.0 - l);
                    (h, s, l, c)
                }
            };
            
            let new_l = (l + level).clamp(0.0, 1.0);
            
            let (new_r, new_g, new_b) = match color_space {
                ColorSpace::Hsl => hsl_to_rgb_f32(h, s, new_l),
                ColorSpace::Lch => {
                    let new_s = if new_l > 0.0 { c / (2.0 * new_l.min(1.0 - new_l)) } else { 0.0 };
                    hsl_to_rgb_f32(h, new_s, new_l)
                }
                ColorSpace::Hsv => hsv_to_rgb(h, s, new_l),
                ColorSpace::Hsluv => hsl_to_rgb_f32(h, s, new_l),
            };
            
            result[i] = (new_r * 255.0).round().clamp(0.0, 255.0) as u8;
            result[i + 1] = (new_g * 255.0).round().clamp(0.0, 255.0) as u8;
            result[i + 2] = (new_b * 255.0).round().clamp(0.0, 255.0) as u8;
        }
    }
    
    *image = PhotonImage::new(result, image.get_width(), image.get_height());
}

/// 转换图像色彩空间
fn convert_image_color_space(image: &mut PhotonImage, from: ColorSpace, to: ColorSpace) {
    if from == to {
        return;
    }
    
    let pixels = image.get_raw_pixels();
    let mut result = pixels.clone();
    
    for i in (0..result.len()).step_by(4) {
        if result[i + 3] > 0 {
            let r = result[i] as f32 / 255.0;
            let g = result[i + 1] as f32 / 255.0;
            let b = result[i + 2] as f32 / 255.0;
            
            // 转换到中间空间（RGB）
            let (r, g, b) = match from {
                ColorSpace::Hsl => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    hsl_to_rgb_f32(h, s, l)
                }
                ColorSpace::Lch => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    let c = 2.0 * s * l.min(1.0 - l);
                    // LCH to RGB
                    let s = if l > 0.0 { c / (2.0 * l.min(1.0 - l)) } else { 0.0 };
                    hsl_to_rgb_f32(h, s, l)
                }
                ColorSpace::Hsv => {
                    let (h, s, v) = rgb_to_hsv(r, g, b);
                    hsv_to_rgb(h, s, v)
                }
                ColorSpace::Hsluv => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    hsl_to_rgb_f32(h, s, l)
                }
            };
            
            // 从 RGB 转换到目标空间
            let (new_r, new_g, new_b) = match to {
                ColorSpace::Hsl => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    hsl_to_rgb_f32(h, s, l)
                }
                ColorSpace::Lch => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    let c = 2.0 * s * l.min(1.0 - l);
                    let s = if l > 0.0 { c / (2.0 * l.min(1.0 - l)) } else { 0.0 };
                    hsl_to_rgb_f32(h, s, l)
                }
                ColorSpace::Hsv => {
                    let (h, s, v) = rgb_to_hsv(r, g, b);
                    hsv_to_rgb(h, s, v)
                }
                ColorSpace::Hsluv => {
                    let (h, s, l) = rgb_to_hsl_f32(r, g, b);
                    hsl_to_rgb_f32(h, s, l)
                }
            };
            
            result[i] = (new_r * 255.0).round().clamp(0.0, 255.0) as u8;
            result[i + 1] = (new_g * 255.0).round().clamp(0.0, 255.0) as u8;
            result[i + 2] = (new_b * 255.0).round().clamp(0.0, 255.0) as u8;
        }
    }
    
    *image = PhotonImage::new(result, image.get_width(), image.get_height());
}

/// RGB 转 HSV
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    
    let v = max;
    let s = if max == 0.0 { 0.0 } else { delta / max };
    
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        (((g - b) / delta) + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
        (((b - r) / delta) + 2.0) / 6.0
    } else {
        (((r - g) / delta) + 4.0) / 6.0
    };
    
    (h, s, v)
}

/// HSV 转 RGB
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (v, v, v);
    }
    
    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    
    match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

/// 脏区域（需要重新渲染的区域）
#[derive(Clone, Debug)]
struct DirtyRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl DirtyRect {
    /// 创建新的脏区域
    fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        DirtyRect {
            x: x.max(0),
            y: y.max(0),
            width,
            height,
        }
    }

    /// 创建包含整个画布的脏区域
    fn full(canvas_width: u32, canvas_height: u32) -> Self {
        DirtyRect {
            x: 0,
            y: 0,
            width: canvas_width,
            height: canvas_height,
        }
    }

    /// 合并两个脏区域，返回包含两者的最小矩形
    fn merge(&self, other: &DirtyRect) -> DirtyRect {
        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);

        DirtyRect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    /// 检查是否与另一个脏区域重叠
    fn intersects(&self, other: &DirtyRect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

/// 图层堆栈
/// 管理多个图层及其合成顺序
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct LayerStack {
    layers: Vec<Layer>,
    next_id: usize,
    canvas_width: u32,
    canvas_height: u32,
    background_color: [u8; 4], // RGBA
    // 增量渲染相关字段
    dirty_rects: Vec<DirtyRect>,      // 脏区域列表
    cached_composite: Option<PhotonImage>,  // 缓存的合成图像
    incremental_enabled: bool,        // 是否启用增量渲染
}

impl LayerStack {
    /// 创建新的图层堆栈
    pub fn new(width: u32, height: u32) -> LayerStack {
        LayerStack {
            layers: Vec::new(),
            next_id: 1,
            canvas_width: width,
            canvas_height: height,
            background_color: [255, 255, 255, 255], // 默认白色背景
            dirty_rects: Vec::new(),
            cached_composite: None,
            incremental_enabled: true, // 默认启用增量渲染
        }
    }

    /// 创建带有背景图层的堆栈
    pub fn with_background(width: u32, height: u32, background_image: PhotonImage) -> LayerStack {
        let mut stack = LayerStack::new(width, height);
        let bg_layer = Layer::from_image(0, "Background".to_string(), background_image);
        stack.layers.push(bg_layer);
        stack.next_id = 1;
        // 标记整个画布为脏区域
        stack.mark_dirty_full();
        stack
    }
}

#[wasm_bindgen]
impl LayerStack {
    /// 创建新的图层堆栈（JavaScript 调用）
    #[wasm_bindgen(constructor)]
    pub fn new_js(width: u32, height: u32) -> LayerStack {
        LayerStack::new(width, height)
    }

    /// 获取画布宽度
    #[wasm_bindgen(getter)]
    pub fn canvas_width(&self) -> u32 {
        self.canvas_width
    }

    /// 获取画布高度
    #[wasm_bindgen(getter)]
    pub fn canvas_height(&self) -> u32 {
        self.canvas_height
    }

    /// 获取图层数量
    #[wasm_bindgen(getter)]
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// 获取背景颜色
    #[wasm_bindgen(getter)]
    pub fn background_color(&self) -> Vec<u8> {
        self.background_color.to_vec()
    }

    /// 设置背景颜色
    #[wasm_bindgen]
    pub fn set_background_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.background_color = [r, g, b, a];
    }

    /// 添加新图层
    #[wasm_bindgen]
    pub fn add_layer(&mut self, name: Option<String>) -> usize {
        let layer_name = name.unwrap_or_else(|| format!("Layer {}", self.next_id));
        let layer = Layer::new(self.next_id, layer_name, self.canvas_width, self.canvas_height);
        self.layers.push(layer);
        let id = self.next_id;
        self.next_id += 1;

        // 标记新图层区域为脏区域
        self.mark_layer_dirty(self.layers.len() - 1);

        id
    }

    /// 从图像数据添加图层
    #[wasm_bindgen]
    pub fn add_layer_from_pixels(&mut self, pixels: Vec<u8>, name: Option<String>) -> Result<usize, JsValue> {
        if pixels.len() != (self.canvas_width * self.canvas_height * 4) as usize {
            return Err(JsValue::from_str("Pixel data size does not match canvas dimensions"));
        }

        let layer_name = name.unwrap_or_else(|| format!("Layer {}", self.next_id));
        let image = PhotonImage::new(pixels, self.canvas_width, self.canvas_height);
        let layer = Layer::from_image(self.next_id, layer_name, image);
        self.layers.push(layer);
        let id = self.next_id;
        self.next_id += 1;

        // 标记新图层区域为脏区域
        self.mark_layer_dirty(self.layers.len() - 1);

        Ok(id)
    }

    /// 通过 ID 获取图层
    #[wasm_bindgen]
    pub fn get_layer(&self, id: usize) -> Option<Layer> {
        self.layers.iter().find(|l| l.id == id).cloned()
    }

    /// 通过索引获取图层（索引 0 为最底层）
    #[wasm_bindgen]
    pub fn get_layer_by_index(&self, index: usize) -> Option<Layer> {
        if index < self.layers.len() {
            Some(self.layers[index].clone())
        } else {
            None
        }
    }

    /// 通过索引获取可变图层的引用（内部使用）
    fn get_layer_mut_by_index(&mut self, index: usize) -> Option<&mut Layer> {
        self.layers.get_mut(index)
    }

    /// 通过 ID 获取可变图层的引用（内部使用）
    fn get_layer_mut(&mut self, id: usize) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }

    /// 通过 ID 获取图层索引（内部使用）
    fn get_layer_index(&self, id: usize) -> Option<usize> {
        self.layers.iter().position(|l| l.id == id)
    }

    // ==================== 增量渲染辅助方法 ====================

    /// 标记整个画布为脏区域
    fn mark_dirty_full(&mut self) {
        self.dirty_rects = vec![DirtyRect::full(self.canvas_width, self.canvas_height)];
        self.cached_composite = None;
    }

    /// 标记指定区域为脏区域
    fn mark_dirty(&mut self, x: u32, y: u32, width: u32, height: u32) {
        if !self.incremental_enabled {
            self.mark_dirty_full();
            return;
        }

        let new_dirty = DirtyRect::new(x, y, width, height);

        // 尝试与现有的脏区域合并
        let mut merged = false;
        for dirty in &mut self.dirty_rects {
            if dirty.intersects(&new_dirty) {
                *dirty = dirty.merge(&new_dirty);
                merged = true;
                break;
            }
        }

        if !merged {
            self.dirty_rects.push(new_dirty);
        }

        // 清除缓存，因为需要重新渲染
        self.cached_composite = None;
    }

    /// 标记图层影响的所有区域为脏区域
    fn mark_layer_dirty(&mut self, layer_index: usize) {
        if let Some(layer) = self.layers.get(layer_index) {
            // 计算图层的边界框（考虑变换）
            if let Some(bounds) = self.get_layer_bounds(layer.id) {
                let x = bounds[0].max(0.0) as u32;
                let y = bounds[1].max(0.0) as u32;
                let width = bounds[2].ceil() as u32;
                let height = bounds[3].ceil() as u32;

                // 扩展边界框以考虑混合模式的影响（例如模糊效果）
                let padding = 2u32;
                self.mark_dirty(
                    x.saturating_sub(padding),
                    y.saturating_sub(padding),
                    width + padding * 2,
                    height + padding * 2,
                );
            }
        }
    }

    /// 标记从指定索引开始的所有图层为脏区域
    fn mark_layers_above_dirty(&mut self, start_index: usize) {
        for i in start_index..self.layers.len() {
            self.mark_layer_dirty(i);
        }
    }

    /// 清除所有脏区域标记
    fn clear_dirty(&mut self) {
        self.dirty_rects.clear();
    }

    /// 检查是否有脏区域
    fn has_dirty(&self) -> bool {
        !self.dirty_rects.is_empty() || self.cached_composite.is_none()
    }

    /// 设置是否启用增量渲染
    #[wasm_bindgen]
    pub fn set_incremental_rendering(&mut self, enabled: bool) {
        if !enabled {
            // 如果禁用增量渲染，清除缓存和脏区域
            self.cached_composite = None;
            self.dirty_rects.clear();
        }
        self.incremental_enabled = enabled;
    }

    /// 获取是否启用增量渲染
    #[wasm_bindgen(getter = incrementalEnabled)]
    pub fn incremental_enabled(&self) -> bool {
        self.incremental_enabled
    }

    /// 设置图层位置
    #[wasm_bindgen]
    pub fn set_layer_position(&mut self, id: usize, x: f32, y: f32) -> bool {
        // 先获取索引
        let index = self.get_layer_index(id);

        if let Some(index) = index {
            // 标记旧位置的区域为脏区域
            self.mark_layer_dirty(index);

            // 修改图层
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_position(x, y);
            }

            // 标记新位置的区域为脏区域
            self.mark_layer_dirty(index);
            true
        } else {
            false
        }
    }

    /// 设置图层缩放
    #[wasm_bindgen]
    pub fn set_layer_scale(&mut self, id: usize, scale_x: f32, scale_y: f32) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_scale_x(scale_x);
                layer.set_scale_y(scale_y);
            }
            self.mark_layer_dirty(index);
            true
        } else {
            false
        }
    }

    /// 设置图层旋转
    #[wasm_bindgen]
    pub fn set_layer_rotation(&mut self, id: usize, degrees: f32) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_rotation_degrees(degrees);
            }
            self.mark_layer_dirty(index);
            true
        } else {
            false
        }
    }

    /// 设置图层翻转
    #[wasm_bindgen]
    pub fn set_layer_flip(&mut self, id: usize, horizontal: bool, vertical: bool) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_flip_horizontal(horizontal);
                layer.set_flip_vertical(vertical);
            }
            self.mark_layer_dirty(index);
            true
        } else {
            false
        }
    }

    /// 设置图层混合模式
    #[wasm_bindgen]
    pub fn set_layer_blend_mode(&mut self, id: usize, blend_mode: BlendMode) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.blend_mode = blend_mode;
            }
            true
        } else {
            false
        }
    }

    /// 设置图层不透明度
    #[wasm_bindgen]
    pub fn set_layer_opacity(&mut self, id: usize, opacity: u8) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.opacity = opacity;
            }
            true
        } else {
            false
        }
    }

    /// 设置图层可见性
    #[wasm_bindgen]
    pub fn set_layer_visible(&mut self, id: usize, visible: bool) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.visible = visible;
            }
            true
        } else {
            false
        }
    }

    /// 设置图层是否为智能对象
    #[wasm_bindgen]
    pub fn set_layer_smart_object(&mut self, id: usize, smart_object: bool) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.set_smart_object(smart_object);
            true
        } else {
            false
        }
    }

    /// 设置图层的像素数据
    #[wasm_bindgen]
    pub fn set_layer_pixels(&mut self, id: usize, pixels: Vec<u8>) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_pixels(pixels);
            }
            true
        } else {
            false
        }
    }

    /// 设置图层透视变换是否启用
    #[wasm_bindgen]
    pub fn set_layer_perspective_enabled(&mut self, id: usize, enabled: bool) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_perspective_enabled(enabled);
            }
            true
        } else {
            false
        }
    }

    /// 设置图层透视变换的四个角点
    #[wasm_bindgen]
    pub fn set_layer_perspective_points(
        &mut self,
        id: usize,
        top_left_x: f32,
        top_left_y: f32,
        top_right_x: f32,
        top_right_y: f32,
        bottom_left_x: f32,
        bottom_left_y: f32,
        bottom_right_x: f32,
        bottom_right_y: f32,
    ) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_perspective_points(
                    top_left_x, top_left_y,
                    top_right_x, top_right_y,
                    bottom_left_x, bottom_left_y,
                    bottom_right_x, bottom_right_y,
                );
            }
            true
        } else {
            false
        }
    }

    /// 重置图层透视变换
    #[wasm_bindgen]
    pub fn reset_layer_perspective(&mut self, id: usize) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layer_dirty(index);
            if let Some(layer) = self.get_layer_mut(id) {
                layer.reset_perspective();
            }
            true
        } else {
            false
        }
    }

    /// 以指定图层中心为参照点进行变换
    /// target_id: 要变换的图层ID
    /// reference_id: 参照图层ID（None表示使用画布中心）
    #[wasm_bindgen]
    pub fn set_transform_reference_layer(&mut self, target_id: usize, reference_id: Option<usize>) -> bool {
        // 先获取画布中心和参照图层信息（避免借用冲突）
        let canvas_center_x = self.canvas_width as f32 / 2.0;
        let canvas_center_y = self.canvas_height as f32 / 2.0;

        let transform_center = if let Some(ref_id) = reference_id {
            self.layers.iter().find(|l| l.id == ref_id).map(|ref_layer| {
                // 参照图层的中心位置（考虑位置偏移）
                let ref_center_x = canvas_center_x + ref_layer.position_x;
                let ref_center_y = canvas_center_y + ref_layer.position_y;
                (ref_center_x, ref_center_y)
            })
        } else {
            None
        };

        // 现在可以安全地修改目标图层
        if let Some(layer) = self.get_layer_mut(target_id) {
            if let Some((center_x, center_y)) = transform_center {
                layer.set_transform_center(Some(center_x), Some(center_y));
            } else {
                layer.clear_transform_center();
            }
            true
        } else {
            false
        }
    }

    /// 以画布指定点为变换中心
    /// target_id: 要变换的图层ID
    /// center_x, center_y: 画布坐标系中的变换中心点
    #[wasm_bindgen]
    pub fn set_transform_center_on_canvas(&mut self, target_id: usize, center_x: f32, center_y: f32) -> bool {
        if let Some(layer) = self.get_layer_mut(target_id) {
            layer.set_transform_center(Some(center_x), Some(center_y));
            true
        } else {
            false
        }
    }

    /// 删除图层
    #[wasm_bindgen]
    pub fn remove_layer(&mut self, id: usize) -> bool {
        if let Some(index) = self.get_layer_index(id) {
            self.mark_layers_above_dirty(index);
            self.layers.remove(index);
            self.mark_dirty_full(); // 删除图层会影响整个画布
            true
        } else {
            false
        }
    }

    /// 通过索引删除图层
    #[wasm_bindgen]
    pub fn remove_layer_by_index(&mut self, index: usize) -> bool {
        if index < self.layers.len() {
            self.mark_layers_above_dirty(index);
            self.layers.remove(index);
            self.mark_dirty_full(); // 删除图层会影响整个画布
            true
        } else {
            false
        }
    }

    /// 移动图层
    /// from_index: 原始索引
    /// to_index: 目标索引
    #[wasm_bindgen]
    pub fn move_layer(&mut self, from_index: usize, to_index: usize) -> bool {
        if from_index >= self.layers.len() || to_index > self.layers.len() {
            return false;
        }

        if from_index == to_index {
            return true;
        }

        // 标记受影响的图层区域为脏区域
        let min_index = from_index.min(to_index);
        self.mark_layers_above_dirty(min_index);

        let layer = self.layers.remove(from_index);
        self.layers.insert(to_index, layer);
        true
    }

    /// 复制图层
    #[wasm_bindgen]
    pub fn duplicate_layer(&mut self, id: usize) -> Option<usize> {
        let layer = self.layers.iter().find(|l| l.id == id)?;
        let mut new_layer = layer.clone();
        new_layer.id = self.next_id;
        new_layer.name = format!("{} Copy", layer.name);
        self.layers.push(new_layer);
        let new_id = self.next_id;
        self.next_id += 1;

        // 标记新图层区域为脏区域
        self.mark_layer_dirty(self.layers.len() - 1);

        Some(new_id)
    }

    /// 向下合并图层
    /// 将指定图层与其下方的图层合并
    #[wasm_bindgen]
    pub fn merge_down(&mut self, index: usize) -> bool {
        if index == 0 || index >= self.layers.len() {
            return false;
        }

        // 标记受影响的区域为脏区域
        self.mark_layer_dirty(index - 1);

        // 先克隆上层的信息
        let upper_blend_mode = self.layers[index].blend_mode;
        let upper_opacity = self.layers[index].opacity;
        let upper_image = self.layers[index].image.clone();

        let lower = self.layers.get_mut(index - 1).unwrap();

        // 合并图像：将 upper 混合到 lower 上
        blend_layers(&mut lower.image, &upper_image, upper_blend_mode, upper_opacity);

        // 删除上层
        self.layers.remove(index);
        true
    }

    /// 合并所有可见图层
    #[wasm_bindgen]
    pub fn flatten(&mut self) -> bool {
        if self.layers.len() <= 1 {
            return false;
        }

        // 标记整个画布为脏区域
        self.mark_dirty_full();

        let bottom_index = 0;

        // 从上到下依次合并到底层
        while self.layers.len() > 1 {
            let top_index = self.layers.len() - 1;

            // 先克隆顶层的信息
            let top_visible = self.layers[top_index].visible;
            let top_blend_mode = self.layers[top_index].blend_mode;
            let top_opacity = self.layers[top_index].opacity;
            let top_image = self.layers[top_index].image.clone();

            let bottom = self.layers.get_mut(bottom_index).unwrap();

            if top_visible {
                blend_layers(&mut bottom.image, &top_image, top_blend_mode, top_opacity);
            }

            self.layers.remove(top_index);
        }

        true
    }

    /// 获取所有图层 ID 列表
    #[wasm_bindgen]
    pub fn get_layer_ids(&self) -> Vec<usize> {
        self.layers.iter().map(|l| l.id).collect()
    }

    /// 获取所有图层名称列表
    #[wasm_bindgen]
    pub fn get_layer_names(&self) -> Vec<String> {
        self.layers.iter().map(|l| l.name.clone()).collect()
    }

    /// 获取所有图层可见性列表 (0 = 不可见, 1 = 可见)
    #[wasm_bindgen]
    pub fn get_layer_visibility(&self) -> Vec<u8> {
        self.layers.iter().map(|l| if l.visible { 1 } else { 0 }).collect()
    }

    /// 获取所有图层不透明度列表
    #[wasm_bindgen]
    pub fn get_layer_opacities(&self) -> Vec<u8> {
        self.layers.iter().map(|l| l.opacity).collect()
    }

    /// 合成所有图层，生成最终图像
    #[wasm_bindgen]
    pub fn render_composite(&self) -> PhotonImage {
        // 创建基础画布（背景色）
        let pixel_count = (self.canvas_width * self.canvas_height) as usize;
        let mut composited_pixels = vec![0u8; pixel_count * 4];

        // 填充背景色
        for i in (0..composited_pixels.len()).step_by(4) {
            composited_pixels[i] = self.background_color[0];
            composited_pixels[i + 1] = self.background_color[1];
            composited_pixels[i + 2] = self.background_color[2];
            composited_pixels[i + 3] = self.background_color[3];
        }

        let mut composited = PhotonImage::new(composited_pixels, self.canvas_width, self.canvas_height);

        // 从下到上合成所有可见图层
        for layer in &self.layers {
            if !layer.visible {
                continue;
            }

            // 检查图层是否有变换
            let has_transform = has_transform(layer);

            let layer_image = if has_transform {
                // 应用变换
                apply_layer_transform(layer, self.canvas_width, self.canvas_height)
            } else {
                // 无变换，直接使用原图
                layer.image.clone()
            };

            blend_layers(&mut composited, &layer_image, layer.blend_mode, layer.opacity);
        }

        composited
    }

    /// 增量渲染：只重新渲染脏区域
    /// 如果没有脏区域，返回缓存的图像
    #[wasm_bindgen]
    pub fn incremental_render(&mut self) -> PhotonImage {
        // 如果没有启用增量渲染或者没有缓存，使用完整渲染
        if !self.incremental_enabled || self.cached_composite.is_none() {
            let result = self.render_composite();
            self.cached_composite = Some(result.clone());
            self.clear_dirty();
            return result;
        }

        // 如果没有脏区域，直接返回缓存
        if !self.has_dirty() {
            return self.cached_composite.as_ref().unwrap().clone();
        }

        // 获取缓存的图像
        let mut composited = self.cached_composite.as_ref().unwrap().clone();
        let composited_pixels = composited.get_raw_pixels();
        let mut result_pixels = composited_pixels.clone();

        // 对每个脏区域进行重新渲染
        for dirty_rect in &self.dirty_rects {
            // 确保脏区域在画布范围内
            let x = dirty_rect.x.min(self.canvas_width);
            let y = dirty_rect.y.min(self.canvas_height);
            let width = (dirty_rect.width).min(self.canvas_width - x);
            let height = (dirty_rect.height).min(self.canvas_height - y);

            if width == 0 || height == 0 {
                continue;
            }

            // 创建脏区域的临时画布
            let dirty_pixel_count = (width * height) as usize;
            let mut dirty_pixels = vec![0u8; dirty_pixel_count * 4];

            // 填充背景色
            for i in (0..dirty_pixels.len()).step_by(4) {
                dirty_pixels[i] = self.background_color[0];
                dirty_pixels[i + 1] = self.background_color[1];
                dirty_pixels[i + 2] = self.background_color[2];
                dirty_pixels[i + 3] = self.background_color[3];
            }

            let mut dirty_canvas = PhotonImage::new(dirty_pixels, width, height);

            // 从下到上合成所有可见图层到脏区域
            for layer in &self.layers {
                if !layer.visible {
                    continue;
                }

                // 检查图层是否与脏区域相交
                if let Some(layer_bounds) = self.get_layer_bounds(layer.id) {
                    let layer_x = layer_bounds[0] as i32;
                    let layer_y = layer_bounds[1] as i32;
                    let layer_w = layer_bounds[2] as i32;
                    let layer_h = layer_bounds[3] as i32;
                    let dirty_x_i = x as i32;
                    let dirty_y_i = y as i32;
                    let dirty_w_i = width as i32;
                    let dirty_h_i = height as i32;

                    // 简单的矩形相交测试
                    if layer_x + layer_w <= dirty_x_i
                        || layer_x >= dirty_x_i + dirty_w_i
                        || layer_y + layer_h <= dirty_y_i
                        || layer_y >= dirty_y_i + dirty_h_i
                    {
                        continue; // 不相交，跳过
                    }
                }

                // 应用变换
                let layer_image = if has_transform(layer) {
                    apply_layer_transform(layer, self.canvas_width, self.canvas_height)
                } else {
                    layer.image.clone()
                };

                // 将图层合成到脏区域
                blend_layers(&mut dirty_canvas, &layer_image, layer.blend_mode, layer.opacity);
            }

            // 将渲染好的脏区域复制回主画布
            let dirty_rendered_pixels = dirty_canvas.get_raw_pixels();
            for dy in 0..height {
                for dx in 0..width {
                    let src_idx = (dy * width + dx) as usize * 4;
                    let dst_idx = ((y + dy) * self.canvas_width + (x + dx)) as usize * 4;

                    result_pixels[dst_idx] = dirty_rendered_pixels[src_idx];
                    result_pixels[dst_idx + 1] = dirty_rendered_pixels[src_idx + 1];
                    result_pixels[dst_idx + 2] = dirty_rendered_pixels[src_idx + 2];
                    result_pixels[dst_idx + 3] = dirty_rendered_pixels[src_idx + 3];
                }
            }
        }

        // 更新缓存
        composited = PhotonImage::new(result_pixels, self.canvas_width, self.canvas_height);
        self.cached_composite = Some(composited.clone());
        self.clear_dirty();

        composited
    }

    /// 清空所有图层
    #[wasm_bindgen]
    pub fn clear_all(&mut self) {
        self.layers.clear();
        self.next_id = 1;
        self.mark_dirty_full();
        self.cached_composite = None;
    }

    /// 获取图层的变换信息（用于调试）
    #[wasm_bindgen]
    pub fn get_layer_transform_info(&self, id: usize) -> Option<String> {
        self.layers.iter().find(|l| l.id == id).map(|layer| {
            let transform_center_str = match (layer.transform_center_x, layer.transform_center_y) {
                (Some(x), Some(y)) => format!("transform_center=({},{})", x, y),
                _ => "transform_center=None".to_string(),
            };
            format!(
                "Layer {}: pos=({},{}) scale=({},{}) rot={}° origin=({},{}) {} flip=({},{})",
                layer.id,
                layer.position_x,
                layer.position_y,
                layer.scale_x,
                layer.scale_y,
                layer.rotation_degrees,
                layer.origin_x,
                layer.origin_y,
                transform_center_str,
                layer.flip_horizontal,
                layer.flip_vertical
            )
        })
    }

    /// 批量设置多个图层的缩放
    #[wasm_bindgen]
    pub fn batch_set_scale(&mut self, layer_ids: Vec<usize>, scale_x: f32, scale_y: f32) -> usize {
        let mut count = 0;
        for id in layer_ids {
            if self.set_layer_scale(id, scale_x, scale_y) {
                count += 1;
            }
        }
        count
    }

    /// 批量设置多个图层的旋转
    #[wasm_bindgen]
    pub fn batch_set_rotation(&mut self, layer_ids: Vec<usize>, degrees: f32) -> usize {
        let mut count = 0;
        for id in layer_ids {
            if self.set_layer_rotation(id, degrees) {
                count += 1;
            }
        }
        count
    }

    /// 批量设置多个图层的缩放（均匀缩放）
    #[wasm_bindgen]
    pub fn batch_set_scale_uniform(&mut self, layer_ids: Vec<usize>, scale: f32) -> usize {
        let mut count = 0;
        for id in layer_ids {
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_scale(scale);
                count += 1;
            }
        }
        count
    }

    /// 批量设置多个图层的位置（相对偏移）
    #[wasm_bindgen]
    pub fn batch_move_layers(&mut self, layer_ids: Vec<usize>, delta_x: f32, delta_y: f32) -> usize {
        let mut count = 0;
        for id in layer_ids {
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_position(layer.position_x + delta_x, layer.position_y + delta_y);
                count += 1;
            }
        }
        count
    }

    /// 将图层移动到画布中心
    #[wasm_bindgen]
    pub fn center_layer(&mut self, id: usize) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.set_position(0.0, 0.0);
            true
        } else {
            false
        }
    }

    /// 批量将图层移动到画布中心
    #[wasm_bindgen]
    pub fn batch_center_layers(&mut self, layer_ids: Vec<usize>) -> usize {
        let mut count = 0;
        for id in layer_ids {
            if self.center_layer(id) {
                count += 1;
            }
        }
        count
    }

    /// 对齐多个图层到指定图层
    #[wasm_bindgen]
    pub fn align_layers_to_layer(&mut self, target_ids: Vec<usize>, reference_id: usize) -> bool {
        let reference_layer = match self.get_layer(reference_id) {
            Some(l) => l,
            None => return false,
        };

        let ref_pos_x = reference_layer.position_x;
        let ref_pos_y = reference_layer.position_y;

        for id in target_ids {
            if let Some(layer) = self.get_layer_mut(id) {
                layer.set_position(ref_pos_x, ref_pos_y);
            }
        }
        true
    }

    /// 获取图层的实际边界框（考虑变换后的位置）
    #[wasm_bindgen]
    pub fn get_layer_bounds(&self, id: usize) -> Option<Vec<f32>> {
        self.layers.iter().find(|l| l.id == id).map(|layer| {
            let layer_w = layer.image.get_width() as f32;
            let layer_h = layer.image.get_height() as f32;
            let canvas_center_x = self.canvas_width as f32 / 2.0;
            let canvas_center_y = self.canvas_height as f32 / 2.0;

            // 计算图层的实际中心位置
            let actual_center_x = canvas_center_x + layer.position_x;
            let actual_center_y = canvas_center_y + layer.position_y;

            // 考虑缩放后的尺寸
            let scaled_w = layer_w * layer.scale_x.abs();
            let scaled_h = layer_h * layer.scale_y.abs();

            // 返回边界框 [x, y, width, height]
            vec![
                actual_center_x - scaled_w / 2.0,
                actual_center_y - scaled_h / 2.0,
                scaled_w,
                scaled_h
            ]
        })
    }
}

/// 混合两个图层
fn blend_layers(bottom: &mut PhotonImage, top: &PhotonImage, blend_mode: BlendMode, opacity: u8) {
    // 转换 blend_mode 到 photon_rs 的混合模式
    let photon_blend_mode = match blend_mode {
        BlendMode::Normal => "over",
        BlendMode::Multiply => "multiply",
        BlendMode::Screen => "screen",
        BlendMode::Overlay => "overlay",
        BlendMode::SoftLight => "soft_light",
        BlendMode::HardLight => "hard_light",
        BlendMode::Difference => "difference",
        BlendMode::Exclusion => "exclusion",
        BlendMode::Lighten => "lighten",
        BlendMode::Darken => "darken",
        // 对于 photon_rs 不支持的混合模式，使用自定义实现
        BlendMode::ColorDodge => {
            blend_custom(bottom, top, |b, t| color_dodge(b, t));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::ColorBurn => {
            blend_custom(bottom, top, |b, t| color_burn(b, t));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::LinearDodge => {
            blend_custom(bottom, top, |b, t| linear_dodge(b, t));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::LinearBurn => {
            blend_custom(bottom, top, |b, t| linear_burn(b, t));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::VividLight => {
            blend_custom(bottom, top, |b, t| vivid_light(b, t));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::LinearLight => {
            blend_custom(bottom, top, |b, t| linear_light(b, t));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::PinLight => {
            blend_custom(bottom, top, |b, t| pin_light(b, t));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::HardMix => {
            blend_custom(bottom, top, |b, t| hard_mix(b, t));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::Hue => {
            blend_hsl(bottom, top, |_bh, bs, bl, th, _ts, _tl| (th, bs, bl));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::Saturation => {
            blend_hsl(bottom, top, |bh, _bs, bl, _th, ts, _tl| (bh, ts, bl));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::Color => {
            blend_hsl(bottom, top, |_bh, _bs, bl, th, ts, _tl| (th, ts, bl));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
        BlendMode::Luminosity => {
            blend_hsl(bottom, top, |bh, bs, _bl, _th, _ts, tl| (bh, bs, tl));
            if opacity < 255 {
                apply_opacity(bottom, opacity);
            }
            return;
        }
    };

    // 应用混合
    multiple::blend(bottom, top, photon_blend_mode);

    // 应用不透明度
    if opacity < 255 {
        apply_opacity(bottom, opacity);
    }
}

// ==================== 自定义混合模式实现 ====================

/// 自定义像素混合函数
fn blend_custom<F>(bottom: &mut PhotonImage, top: &PhotonImage, blend_fn: F)
where
    F: Fn(u8, u8) -> u8,
{
    let bottom_pixels = bottom.get_raw_pixels();
    let top_pixels = top.get_raw_pixels();
    let mut result = bottom_pixels.clone();

    for i in (0..result.len()).step_by(4) {
        let alpha = top_pixels[i + 3] as f32 / 255.0;

        for c in 0..3 {
            let b = bottom_pixels[i + c] as f32;
            let t = top_pixels[i + c] as f32;
            let blended = blend_fn(b as u8, t as u8) as f32;
            result[i + c] = (b * (1.0 - alpha) + blended * alpha) as u8;
        }

        // Alpha 混合
        let b_alpha = bottom_pixels[i + 3] as f32;
        let t_alpha = top_pixels[i + 3] as f32;
        result[i + 3] = (b_alpha * (1.0 - alpha) + t_alpha * alpha) as u8;
    }

    *bottom = PhotonImage::new(result, bottom.get_width(), bottom.get_height());
}

/// Color Dodge 混合模式
fn color_dodge(bottom: u8, top: u8) -> u8 {
    let b = bottom as f32;
    let t = top as f32;
    if t == 255.0 {
        255
    } else {
        let result = (b * 256.0) / (256.0 - t);
        result.min(255.0) as u8
    }
}

/// Color Burn 混合模式
fn color_burn(bottom: u8, top: u8) -> u8 {
    let b = bottom as f32;
    let t = top as f32;
    if t == 0.0 {
        0
    } else {
        let result = 255.0 - ((255.0 - b) * 256.0) / t;
        result.max(0.0) as u8
    }
}

/// Linear Dodge (Add) 混合模式
fn linear_dodge(bottom: u8, top: u8) -> u8 {
    (bottom as f32 + top as f32).min(255.0) as u8
}

/// Linear Burn 混合模式
fn linear_burn(bottom: u8, top: u8) -> u8 {
    (bottom as f32 + top as f32 - 255.0).max(0.0) as u8
}

/// Vivid Light 混合模式
fn vivid_light(bottom: u8, top: u8) -> u8 {
    let t = top as f32;
    if t < 128.0 {
        color_burn(bottom, (2.0 * t) as u8)
    } else {
        color_dodge(bottom, (2.0 * (t - 128.0)) as u8)
    }
}

/// Linear Light 混合模式
fn linear_light(bottom: u8, top: u8) -> u8 {
    let t = top as f32;
    if t < 128.0 {
        linear_burn(bottom, (2.0 * t) as u8)
    } else {
        linear_dodge(bottom, (2.0 * (t - 128.0)) as u8)
    }
}

/// Pin Light 混合模式
fn pin_light(bottom: u8, top: u8) -> u8 {
    let t = top as f32;
    if t < 128.0 {
        bottom.min(top)
    } else {
        bottom.max(top)
    }
}

/// Hard Mix 混合模式
fn hard_mix(bottom: u8, top: u8) -> u8 {
    if (bottom as f32 + top as f32) >= 255.0 {
        255
    } else {
        0
    }
}

/// HSL 混合模式基础函数
fn blend_hsl<F>(bottom: &mut PhotonImage, top: &PhotonImage, blend_fn: F)
where
    F: Fn(f32, f32, f32, f32, f32, f32) -> (f32, f32, f32),
{
    let bottom_pixels = bottom.get_raw_pixels();
    let top_pixels = top.get_raw_pixels();
    let mut result = bottom_pixels.clone();

    for i in (0..result.len()).step_by(4) {
        let alpha = top_pixels[i + 3] as f32 / 255.0;

        // 转换到 HSL
        let (bh, bs, bl) = rgb_to_hsl(
            bottom_pixels[i],
            bottom_pixels[i + 1],
            bottom_pixels[i + 2],
        );
        let (th, ts, tl) = rgb_to_hsl(
            top_pixels[i],
            top_pixels[i + 1],
            top_pixels[i + 2],
        );

        // 应用混合函数
        let (h, s, l) = blend_fn(bh, bs, bl, th, ts, tl);

        // 转换回 RGB
        let (r, g, b) = hsl_to_rgb(h, s, l);

        for (c, val) in [r, g, b].iter().enumerate() {
            let b_val = bottom_pixels[i + c] as f32;
            result[i + c] = (b_val * (1.0 - alpha) + *val * alpha) as u8;
        }

        // Alpha 混合
        let b_alpha = bottom_pixels[i + 3] as f32;
        let t_alpha = top_pixels[i + 3] as f32;
        result[i + 3] = (b_alpha * (1.0 - alpha) + t_alpha * alpha) as u8;
    }

    *bottom = PhotonImage::new(result, bottom.get_width(), bottom.get_height());
}

/// RGB 转 HSL (f32 版本，输入输出范围 0-1)
fn rgb_to_hsl_f32(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let (h, s) = if delta == 0.0 {
        (0.0, 0.0)
    } else {
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        let h = if max == r {
            (g - b) / delta + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };

        (h / 6.0, s)
    };

    (h, s, l)
}

/// HSL 转 RGB (f32 版本，输入输出范围 0-1)
fn hsl_to_rgb_f32(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let hue2rgb = |p: f32, q: f32, t: f32| -> f32 {
            let mut t = t;
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 1.0 / 2.0 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        };

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        (
            hue2rgb(p, q, h + 1.0 / 3.0),
            hue2rgb(p, q, h),
            hue2rgb(p, q, h - 1.0 / 3.0),
        )
    };

    (r, g, b)
}

/// RGB 转 HSL
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let (h, s) = if delta == 0.0 {
        (0.0, 0.0)
    } else {
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        let h = if max == rf {
            (gf - bf) / delta + if gf < bf { 6.0 } else { 0.0 }
        } else if max == gf {
            (bf - rf) / delta + 2.0
        } else {
            (rf - gf) / delta + 4.0
        };

        (h / 6.0, s)
    };

    (h, s, l)
}

/// HSL 转 RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let hue2rgb = |p: f32, q: f32, t: f32| -> f32 {
            let mut t = t;
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 1.0 / 2.0 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        };

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        (
            hue2rgb(p, q, h + 1.0 / 3.0),
            hue2rgb(p, q, h),
            hue2rgb(p, q, h - 1.0 / 3.0),
        )
    };

    (r, g, b)
}

/// 应用不透明度到图层
fn apply_opacity(image: &mut PhotonImage, opacity: u8) {
    let opacity_f = opacity as f32 / 255.0;
    let mut pixels = image.get_raw_pixels();

    for i in (0..pixels.len()).step_by(4) {
        pixels[i + 3] = (pixels[i + 3] as f32 * opacity_f) as u8;
    }

    *image = PhotonImage::new(pixels, image.get_width(), image.get_height());
}

// ==================== 变换矩阵和应用逻辑 ====================

/// 2D 变换矩阵
#[derive(Clone, Copy, Debug)]
struct TransformMatrix {
    a: f32,  // 缩放 X
    b: f32,  // 倾斜 Y
    c: f32,  // 倾斜 X
    d: f32,  // 缩放 Y
    tx: f32, // 平移 X
    ty: f32, // 平移 Y
}

impl TransformMatrix {
    /// 创建单位矩阵
    fn identity() -> Self {
        TransformMatrix {
            a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0,
        }
    }

    /// 创建平移矩阵
    fn translate(x: f32, y: f32) -> Self {
        TransformMatrix {
            a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: x, ty: y,
        }
    }

    /// 创建缩放矩阵
    fn scale(sx: f32, sy: f32) -> Self {
        TransformMatrix {
            a: sx, b: 0.0, c: 0.0, d: sy, tx: 0.0, ty: 0.0,
        }
    }

    /// 创建旋转矩阵（弧度）
    fn rotate(angle_rad: f32) -> Self {
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();
        TransformMatrix {
            a: cos, b: sin, c: -sin, d: cos, tx: 0.0, ty: 0.0,
        }
    }

    /// 矩阵乘法
    fn multiply(&self, other: &TransformMatrix) -> Self {
        TransformMatrix {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            tx: self.a * other.tx + self.c * other.ty + self.tx,
            ty: self.b * other.tx + self.d * other.ty + self.ty,
        }
    }

    /// 应用变换到点
    fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.a * x + self.c * y + self.tx,
            self.b * x + self.d * y + self.ty,
        )
    }

    /// 计算逆矩阵
    fn invert(&self) -> Option<TransformMatrix> {
        let det = self.a * self.d - self.b * self.c;
        if det.abs() < 1e-6 {
            return None; // 奇异矩阵，无法求逆
        }

        let inv_det = 1.0 / det;
        Some(TransformMatrix {
            a: self.d * inv_det,
            b: -self.b * inv_det,
            c: -self.c * inv_det,
            d: self.a * inv_det,
            tx: (self.c * self.ty - self.d * self.tx) * inv_det,
            ty: (self.b * self.tx - self.a * self.ty) * inv_det,
        })
    }
}

/// 构建图层的完整变换矩阵
fn build_layer_transform(layer: &Layer, canvas_width: u32, canvas_height: u32) -> TransformMatrix {
    let layer_width = layer.image.get_width() as f32;
    let layer_height = layer.image.get_height() as f32;
    let canvas_w = canvas_width as f32;
    let canvas_h = canvas_height as f32;

    // 计算图层中心（相对于图层坐标系）
    let layer_center_x = layer_width / 2.0;
    let layer_center_y = layer_height / 2.0;

    // 计算画布中心
    let canvas_center_x = canvas_w / 2.0;
    let canvas_center_y = canvas_h / 2.0;

    // 检查是否使用外部变换中心
    let use_external_center = layer.transform_center_x.is_some() && layer.transform_center_y.is_some();

    if use_external_center {
        // 使用外部变换中心的逻辑
        let ext_x = layer.transform_center_x.unwrap();
        let ext_y = layer.transform_center_y.unwrap();

        // 1. 将图层从其中心移到原点
        let mut matrix = TransformMatrix::translate(-layer_center_x, -layer_center_y);

        // 2. 应用变换原点偏移
        let origin_offset_x = (layer.origin_x - 0.5) * layer_width;
        let origin_offset_y = (layer.origin_y - 0.5) * layer_height;
        matrix = matrix.multiply(&TransformMatrix::translate(origin_offset_x, origin_offset_y));

        // 3. 应用缩放
        if layer.scale_x != 1.0 || layer.scale_y != 1.0 {
            matrix = matrix.multiply(&TransformMatrix::scale(layer.scale_x, layer.scale_y));
        }

        // 4. 应用旋转
        if layer.rotation_degrees != 0.0 {
            let angle_rad = layer.rotation_degrees * std::f32::consts::PI / 180.0;
            matrix = matrix.multiply(&TransformMatrix::rotate(angle_rad));
        }

        // 5. 应用翻转
        if layer.flip_horizontal {
            matrix = matrix.multiply(&TransformMatrix::scale(-1.0, 1.0));
        }
        if layer.flip_vertical {
            matrix = matrix.multiply(&TransformMatrix::scale(1.0, -1.0));
        }

        // 6. 移动到画布中心
        matrix = matrix.multiply(&TransformMatrix::translate(canvas_center_x, canvas_center_y));

        // 7. 应用位置偏移
        matrix = matrix.multiply(&TransformMatrix::translate(layer.position_x, layer.position_y));

        // 8. 移动到外部变换中心
        matrix = matrix.multiply(&TransformMatrix::translate(ext_x - canvas_center_x, ext_y - canvas_center_y));

        matrix
    } else {
        // 使用图层自身中心的逻辑（默认）
        // 正确的逻辑：先移到目标位置，然后在目标位置处应用变换
        
        // 1. 先移到目标位置（画布中心 + 位置偏移）
        let mut matrix = TransformMatrix::translate(
            canvas_center_x + layer.position_x,
            canvas_center_y + layer.position_y
        );

        // 2. 应用变换原点偏移（相对于图层中心）
        let origin_offset_x = (layer.origin_x - 0.5) * layer_width;
        let origin_offset_y = (layer.origin_y - 0.5) * layer_height;
        matrix = matrix.multiply(&TransformMatrix::translate(origin_offset_x, origin_offset_y));

        // 3. 应用缩放
        if layer.scale_x != 1.0 || layer.scale_y != 1.0 {
            matrix = matrix.multiply(&TransformMatrix::scale(layer.scale_x, layer.scale_y));
        }

        // 4. 应用旋转
        if layer.rotation_degrees != 0.0 {
            let angle_rad = layer.rotation_degrees * std::f32::consts::PI / 180.0;
            matrix = matrix.multiply(&TransformMatrix::rotate(angle_rad));
        }

        // 5. 应用翻转
        if layer.flip_horizontal {
            matrix = matrix.multiply(&TransformMatrix::scale(-1.0, 1.0));
        }
        if layer.flip_vertical {
            matrix = matrix.multiply(&TransformMatrix::scale(1.0, -1.0));
        }

        // 6. 移回原点（这样变换就是相对于目标位置进行的）
        matrix = matrix.multiply(&TransformMatrix::translate(-layer_center_x, -layer_center_y));

        matrix
    }
}

/// 应用变换到图层图像，返回变换后的图像
fn apply_layer_transform(layer: &Layer, canvas_width: u32, canvas_height: u32) -> PhotonImage {
    let canvas_w = canvas_width as usize;
    let canvas_h = canvas_height as usize;
    let layer_w = layer.image.get_width() as usize;
    let layer_h = layer.image.get_height() as usize;

    let src_pixels = layer.image.get_raw_pixels();
    let mut dst_pixels = vec![0u8; canvas_w * canvas_h * 4];

    if layer.perspective_enabled {
        // 使用透视变换
        apply_perspective_transform(
            &src_pixels,
            &mut dst_pixels,
            layer,
            canvas_width,
            canvas_height,
        );
    } else {
        // 使用常规变换
        let transform = build_layer_transform(layer, canvas_width, canvas_height);
        let inv_transform = transform.invert().unwrap_or_else(|| TransformMatrix::identity());

        // 对画布的每个像素进行反向映射
        for y in 0..canvas_h {
            for x in 0..canvas_w {
                let (src_x, src_y) = inv_transform.transform_point(x as f32, y as f32);

                // 检查是否在源图像范围内
                if src_x >= 0.0 && src_x < layer_w as f32 && src_y >= 0.0 && src_y < layer_h as f32 {
                    let src_x0 = src_x.floor() as i32;
                    let src_y0 = src_y.floor() as i32;
                    let src_x1 = src_x0 + 1;
                    let src_y1 = src_y0 + 1;

                    // 双线性插值
                    let dx = src_x - src_x.floor();
                    let dy = src_y - src_y.floor();

                    let get_pixel = |px: i32, py: i32| -> [u8; 4] {
                        if px >= 0 && px < layer_w as i32 && py >= 0 && py < layer_h as i32 {
                            let idx = (py as usize * layer_w + px as usize) * 4;
                            [src_pixels[idx], src_pixels[idx + 1], src_pixels[idx + 2], src_pixels[idx + 3]]
                        } else {
                            [0, 0, 0, 0]
                        }
                    };

                    let p00 = get_pixel(src_x0, src_y0);
                    let p10 = get_pixel(src_x1, src_y0);
                    let p01 = get_pixel(src_x0, src_y1);
                    let p11 = get_pixel(src_x1, src_y1);

                    let dst_idx = (y * canvas_w + x) * 4;

                    // 对每个通道进行双线性插值
                    for c in 0..4 {
                        let c00 = p00[c] as f32;
                        let c10 = p10[c] as f32;
                        let c01 = p01[c] as f32;
                        let c11 = p11[c] as f32;

                        let c0 = c00 * (1.0 - dx) + c10 * dx;
                        let c1 = c01 * (1.0 - dx) + c11 * dx;
                        let result = c0 * (1.0 - dy) + c1 * dy;

                        dst_pixels[dst_idx + c] = result.round().clamp(0.0, 255.0) as u8;
                    }
                }
            }
        }
    }

    PhotonImage::new(dst_pixels, canvas_width, canvas_height)
}

/// 应用透视变换
fn apply_perspective_transform(
    src_pixels: &[u8],
    dst_pixels: &mut [u8],
    layer: &Layer,
    canvas_width: u32,
    canvas_height: u32,
) {
    let canvas_w = canvas_width as usize;
    let canvas_h = canvas_height as usize;
    let layer_w = layer.image.get_width() as usize;
    let layer_h = layer.image.get_height() as usize;
    let canvas_center_x = canvas_width as f32 / 2.0;
    let canvas_center_y = canvas_height as f32 / 2.0;
    
    // 获取图层的实际位置和尺寸
    let actual_center_x = canvas_center_x + layer.position_x;
    let actual_center_y = canvas_center_y + layer.position_y;
    let scaled_w = layer_w as f32 * layer.scale_x.abs();
    let scaled_h = layer_h as f32 * layer.scale_y.abs();
    
    // 定义源矩形的四个角点（图像空间）
    let src_corners = [
        (0.0, 0.0),                          // 左上
        (layer_w as f32, 0.0),              // 右上
        (0.0, layer_h as f32),              // 左下
        (layer_w as f32, layer_h as f32),   // 右下
    ];
    
    // 定义目标矩形的四个角点（画布空间，考虑位置和缩放）
    let layer_left = actual_center_x - scaled_w / 2.0;
    let layer_top = actual_center_y - scaled_h / 2.0;
    let dst_corners = [
        (
            layer_left + layer.perspective_top_left.0 * scaled_w,
            layer_top + layer.perspective_top_left.1 * scaled_h,
        ),
        (
            layer_left + layer.perspective_top_right.0 * scaled_w,
            layer_top + layer.perspective_top_right.1 * scaled_h,
        ),
        (
            layer_left + layer.perspective_bottom_left.0 * scaled_w,
            layer_top + layer.perspective_bottom_left.1 * scaled_h,
        ),
        (
            layer_left + layer.perspective_bottom_right.0 * scaled_w,
            layer_top + layer.perspective_bottom_right.1 * scaled_h,
        ),
    ];
    
    // 计算透视变换矩阵（从目标空间到源空间）
    if let Some(perspective_matrix) = compute_perspective_matrix(&dst_corners, &src_corners) {
        // 对画布的每个像素进行反向映射
        for y in 0..canvas_h {
            for x in 0..canvas_w {
                let dst_idx = (y * canvas_w + x) * 4;
                
                // 应用透视矩阵
                let (src_x, src_y) = perspective_matrix.transform_point(x as f32, y as f32);
                
                // 检查是否在源图像范围内
                if src_x >= 0.0 && src_x < layer_w as f32 && src_y >= 0.0 && src_y < layer_h as f32 {
                    let src_x0 = src_x.floor() as i32;
                    let src_y0 = src_y.floor() as i32;
                    let src_x1 = src_x0 + 1;
                    let src_y1 = src_y0 + 1;
                    
                    // 双线性插值
                    let dx = src_x - src_x.floor();
                    let dy = src_y - src_y.floor();
                    
                    let get_pixel = |px: i32, py: i32| -> [u8; 4] {
                        if px >= 0 && px < layer_w as i32 && py >= 0 && py < layer_h as i32 {
                            let idx = (py as usize * layer_w + px as usize) * 4;
                            [src_pixels[idx], src_pixels[idx + 1], src_pixels[idx + 2], src_pixels[idx + 3]]
                        } else {
                            [0, 0, 0, 0]
                        }
                    };
                    
                    let p00 = get_pixel(src_x0, src_y0);
                    let p10 = get_pixel(src_x1, src_y0);
                    let p01 = get_pixel(src_x0, src_y1);
                    let p11 = get_pixel(src_x1, src_y1);
                    
                    // 对每个通道进行双线性插值
                    for c in 0..4 {
                        let c00 = p00[c] as f32;
                        let c10 = p10[c] as f32;
                        let c01 = p01[c] as f32;
                        let c11 = p11[c] as f32;
                        
                        let c0 = c00 * (1.0 - dx) + c10 * dx;
                        let c1 = c01 * (1.0 - dx) + c11 * dx;
                        let result = c0 * (1.0 - dy) + c1 * dy;
                        
                        dst_pixels[dst_idx + c] = result.round().clamp(0.0, 255.0) as u8;
                    }
                }
            }
        }
    }
}

/// 计算透视变换矩阵
fn compute_perspective_matrix(
    dst_corners: &[(f32, f32); 4],
    src_corners: &[(f32, f32); 4],
) -> Option<TransformMatrix> {
    // 使用高斯消元法求解透视变换矩阵
    // 从目标坐标 (x', y') 到源坐标 (x, y)
    // x' = (a*x + b*y + c) / (g*x + h*y + 1)
    // y' = (d*x + e*y + f) / (g*x + h*y + 1)
    
    // 构建线性方程组
    let mut a = vec![0.0; 64]; // 8x8 矩阵
    let mut b = vec![0.0; 8];  // 右侧向量
    
    // 为每个角点添加两个方程
    for i in 0..4 {
        let (x, y) = dst_corners[i];
        let (x_prime, y_prime) = src_corners[i];
        
        let idx = i * 2;
        
        // 第一个方程：x' * (g*x + h*y + 1) = a*x + b*y + c
        a[idx * 8 + 0] = x;
        a[idx * 8 + 1] = y;
        a[idx * 8 + 2] = 1.0;
        a[idx * 8 + 3] = 0.0;
        a[idx * 8 + 4] = 0.0;
        a[idx * 8 + 5] = 0.0;
        a[idx * 8 + 6] = -x_prime * x;
        a[idx * 8 + 7] = -x_prime * y;
        b[idx] = x_prime;
        
        // 第二个方程：y' * (g*x + h*y + 1) = d*x + e*y + f
        let idx2 = idx + 1;
        a[idx2 * 8 + 0] = 0.0;
        a[idx2 * 8 + 1] = 0.0;
        a[idx2 * 8 + 2] = 0.0;
        a[idx2 * 8 + 3] = x;
        a[idx2 * 8 + 4] = y;
        a[idx2 * 8 + 5] = 1.0;
        a[idx2 * 8 + 6] = -y_prime * x;
        a[idx2 * 8 + 7] = -y_prime * y;
        b[idx2] = y_prime;
    }
    
    // 高斯消元
    if gaussian_elimination(&mut a, &mut b) {
        // 提取变换参数
        let params = b;
        
        Some(TransformMatrix {
            a: params[0],
            b: params[3],
            c: params[1],
            d: params[4],
            tx: params[2],
            ty: params[5],
        })
    } else {
        None
    }
}

/// 高斯消元法求解线性方程组
fn gaussian_elimination(a: &mut [f32], b: &mut [f32]) -> bool {
    let n = 8;
    
    for i in 0..n {
        // 寻找主元
        let mut max_row = i;
        for k in (i + 1)..n {
            if a[k * n + i].abs() > a[max_row * n + i].abs() {
                max_row = k;
            }
        }
        
        // 交换行
        for k in i..n {
            a.swap(max_row * n + k, i * n + k);
        }
        b.swap(max_row, i);
        
        // 检查是否奇异
        if a[i * n + i].abs() < 1e-10 {
            return false;
        }
        
        // 消元
        for k in (i + 1)..n {
            let factor = a[k * n + i] / a[i * n + i];
            for j in i..n {
                a[k * n + j] -= factor * a[i * n + j];
            }
            b[k] -= factor * b[i];
        }
    }
    
    // 回代
    for i in (0..n).rev() {
        for j in (i + 1)..n {
            b[i] -= a[i * n + j] * b[j];
        }
        b[i] /= a[i * n + i];
    }
    
    true
}

/// 检查图层是否有变换
fn has_transform(layer: &Layer) -> bool {
    layer.position_x != 0.0
        || layer.position_y != 0.0
        || layer.scale_x != 1.0
        || layer.scale_y != 1.0
        || layer.rotation_degrees != 0.0
        || layer.flip_horizontal
        || layer.flip_vertical
        || layer.perspective_enabled
}