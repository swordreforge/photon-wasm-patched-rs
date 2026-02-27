use wasm_bindgen::prelude::*;
use photon_rs::{PhotonImage, multiple};
use crate::types::BlendMode;

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
        }
    }

    /// 从现有图像创建图层
    pub fn from_image(id: usize, name: String, image: PhotonImage) -> Layer {
        Layer {
            id,
            name,
            image,
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
        }
    }

    /// 创建带有背景图层的堆栈
    pub fn with_background(width: u32, height: u32, background_image: PhotonImage) -> LayerStack {
        let mut stack = LayerStack::new(width, height);
        let bg_layer = Layer::from_image(0, "Background".to_string(), background_image);
        stack.layers.push(bg_layer);
        stack.next_id = 1;
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

    /// 设置图层位置
    #[wasm_bindgen]
    pub fn set_layer_position(&mut self, id: usize, x: f32, y: f32) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.set_position(x, y);
            true
        } else {
            false
        }
    }

    /// 设置图层缩放
    #[wasm_bindgen]
    pub fn set_layer_scale(&mut self, id: usize, scale_x: f32, scale_y: f32) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.set_scale_x(scale_x);
            layer.set_scale_y(scale_y);
            true
        } else {
            false
        }
    }

    /// 设置图层旋转
    #[wasm_bindgen]
    pub fn set_layer_rotation(&mut self, id: usize, degrees: f32) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.set_rotation_degrees(degrees);
            true
        } else {
            false
        }
    }

    /// 设置图层翻转
    #[wasm_bindgen]
    pub fn set_layer_flip(&mut self, id: usize, horizontal: bool, vertical: bool) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.set_flip_horizontal(horizontal);
            layer.set_flip_vertical(vertical);
            true
        } else {
            false
        }
    }

    /// 设置图层混合模式
    #[wasm_bindgen]
    pub fn set_layer_blend_mode(&mut self, id: usize, blend_mode: BlendMode) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.blend_mode = blend_mode;
            true
        } else {
            false
        }
    }

    /// 设置图层不透明度
    #[wasm_bindgen]
    pub fn set_layer_opacity(&mut self, id: usize, opacity: u8) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.opacity = opacity;
            true
        } else {
            false
        }
    }

    /// 设置图层可见性
    #[wasm_bindgen]
    pub fn set_layer_visible(&mut self, id: usize, visible: bool) -> bool {
        if let Some(layer) = self.get_layer_mut(id) {
            layer.visible = visible;
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
        let initial_len = self.layers.len();
        self.layers.retain(|l| l.id != id);
        self.layers.len() < initial_len
    }

    /// 通过索引删除图层
    #[wasm_bindgen]
    pub fn remove_layer_by_index(&mut self, index: usize) -> bool {
        if index < self.layers.len() {
            self.layers.remove(index);
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
        Some(new_id)
    }

    /// 向下合并图层
    /// 将指定图层与其下方的图层合并
    #[wasm_bindgen]
    pub fn merge_down(&mut self, index: usize) -> bool {
        if index == 0 || index >= self.layers.len() {
            return false;
        }

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
            
            // 调试信息
            if has_transform {
                web_sys::console::log_1(&format!(
                    "渲染图层 {} - 位置: ({}, {}), 缩放: ({}, {}), 旋转: {}°",
                    layer.id,
                    layer.position_x,
                    layer.position_y,
                    layer.scale_x,
                    layer.scale_y,
                    layer.rotation_degrees
                ).into());
            }

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

    /// 清空所有图层
    #[wasm_bindgen]
    pub fn clear_all(&mut self) {
        self.layers.clear();
        self.next_id = 1;
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

    web_sys::console::log_1(&format!(
        "构建变换矩阵 - 图层: {}, 位置: ({}, {}), 图层尺寸: {}x{}, 使用外部中心: {}",
        layer.id,
        layer.position_x,
        layer.position_y,
        layer_width,
        layer_height,
        use_external_center
    ).into());

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
        web_sys::console::log_1(&format!("  步骤1: 移到目标位置: a={:.2}, b={:.2}, c={:.2}, d={:.2}, tx={:.2}, ty={:.2}",
            matrix.a, matrix.b, matrix.c, matrix.d, matrix.tx, matrix.ty).into());

        // 2. 应用变换原点偏移（相对于图层中心）
        let origin_offset_x = (layer.origin_x - 0.5) * layer_width;
        let origin_offset_y = (layer.origin_y - 0.5) * layer_height;
        web_sys::console::log_1(&format!("  原点偏移: ({:.2}, {:.2})", origin_offset_x, origin_offset_y).into());
        matrix = matrix.multiply(&TransformMatrix::translate(origin_offset_x, origin_offset_y));
        web_sys::console::log_1(&format!("  步骤2: 应用原点偏移: a={:.2}, b={:.2}, c={:.2}, d={:.2}, tx={:.2}, ty={:.2}",
            matrix.a, matrix.b, matrix.c, matrix.d, matrix.tx, matrix.ty).into());

        // 3. 应用缩放
        if layer.scale_x != 1.0 || layer.scale_y != 1.0 {
            matrix = matrix.multiply(&TransformMatrix::scale(layer.scale_x, layer.scale_y));
            web_sys::console::log_1(&format!("  步骤3: 应用缩放: a={:.2}, b={:.2}, c={:.2}, d={:.2}, tx={:.2}, ty={:.2}",
                matrix.a, matrix.b, matrix.c, matrix.d, matrix.tx, matrix.ty).into());
        }

        // 4. 应用旋转
        if layer.rotation_degrees != 0.0 {
            let angle_rad = layer.rotation_degrees * std::f32::consts::PI / 180.0;
            matrix = matrix.multiply(&TransformMatrix::rotate(angle_rad));
            web_sys::console::log_1(&format!("  步骤4: 应用旋转: a={:.2}, b={:.2}, c={:.2}, d={:.2}, tx={:.2}, ty={:.2}",
                matrix.a, matrix.b, matrix.c, matrix.d, matrix.tx, matrix.ty).into());
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
        web_sys::console::log_1(&format!("  步骤6: 移回原点: a={:.2}, b={:.2}, c={:.2}, d={:.2}, tx={:.2}, ty={:.2}",
            matrix.a, matrix.b, matrix.c, matrix.d, matrix.tx, matrix.ty).into());

        matrix
    }
}

/// 打印变换矩阵的详细信息
fn log_matrix_info(matrix: &TransformMatrix, layer: &Layer) {
    web_sys::console::log_1(&format!(
        "图层 {} 变换矩阵: a={:.2}, b={:.2}, c={:.2}, d={:.2}, tx={:.2}, ty={:.2}",
        layer.id, matrix.a, matrix.b, matrix.c, matrix.d, matrix.tx, matrix.ty
    ).into());
    
    // 测试变换几个关键点
    let layer_center = (layer.image.get_width() as f32 / 2.0, layer.image.get_height() as f32 / 2.0);
    let transformed_center = matrix.transform_point(layer_center.0, layer_center.1);
    web_sys::console::log_1(&format!(
        "图层中心 ({:.0}, {:.0}) 变换后位置: ({:.2}, {:.2})",
        layer_center.0, layer_center.1, transformed_center.0, transformed_center.1
    ).into());
}

/// 应用变换到图层图像，返回变换后的图像
fn apply_layer_transform(layer: &Layer, canvas_width: u32, canvas_height: u32) -> PhotonImage {
    web_sys::console::log_1(&format!("应用变换到图层 {}", layer.id).into());
    
    let transform = build_layer_transform(layer, canvas_width, canvas_height);
    log_matrix_info(&transform, layer);
    let inv_transform = transform.invert().unwrap_or_else(|| TransformMatrix::identity());

    let canvas_w = canvas_width as usize;
    let canvas_h = canvas_height as usize;
    let layer_w = layer.image.get_width() as usize;
    let layer_h = layer.image.get_height() as usize;

    let src_pixels = layer.image.get_raw_pixels();
    let mut dst_pixels = vec![0u8; canvas_w * canvas_h * 4];

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

    PhotonImage::new(dst_pixels, canvas_width, canvas_height)
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
}