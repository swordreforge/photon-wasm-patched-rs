use crate::types::{BrushConfig, BrushType, BlendMode, StrokePoint};
use photon_rs::PhotonImage;
use wasm_bindgen::prelude::*;

/// 笔划数据
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BrushStroke {
    /// 笔刷配置
    pub config: BrushConfig,
    /// 输入点集（包含坐标和压感）
    #[wasm_bindgen(skip)]
    pub points: Vec<StrokePoint>,
    /// 缓存的路径（避免重复计算，不导出到 JS）
    #[wasm_bindgen(skip)]
    pub cached_path: Option<tiny_skia::Path>,
}

#[wasm_bindgen]
impl BrushStroke {
    #[wasm_bindgen(constructor)]
    pub fn new(config: BrushConfig) -> BrushStroke {
        BrushStroke {
            config,
            points: Vec::new(),
            cached_path: None,
        }
    }

    /// 获取笔刷配置
    pub fn get_config(&self) -> BrushConfig {
        self.config
    }

    /// 获取点数量
    pub fn get_points_count(&self) -> usize {
        self.points.len()
    }

    /// 内部方法：获取点的引用（仅供内部使用）
    #[allow(dead_code)]
    pub(crate) fn get_points_ref(&self) -> &[StrokePoint] {
        &self.points
    }

    /// 内部方法：获取点的可变引用（仅供内部使用）
    #[allow(dead_code)]
    pub(crate) fn get_points_mut(&mut self) -> &mut Vec<StrokePoint> {
        &mut self.points
    }

    /// 添加一个点到笔划
    pub fn add_point(&mut self, point: StrokePoint) {
        self.points.push(point);
        self.cached_path = None;  // 点集改变，清除缓存
    }

    /// 获取点数量
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// 清除所有点
    pub fn clear_points(&mut self) {
        self.points.clear();
        self.cached_path = None;
    }

    /// 获取或生成路径（带缓存）
    #[allow(dead_code)]
    fn get_or_generate_path(&mut self) -> Option<tiny_skia::Path> {
        if self.cached_path.is_none() {
            let generator = StrokeGenerator::new(self.config);
            self.cached_path = generator.generate_path(&self.points);
        }
        self.cached_path.as_ref().map(|p| p.clone())
    }

    /// 清除路径缓存（当点集改变时调用）
    pub fn invalidate_path_cache(&mut self) {
        self.cached_path = None;
    }
}

// ==================== 数据转换模块 ====================

pub struct DataConverter;

impl DataConverter {
    /// 从 PhotonImage 创建 Pixmap
    pub fn photon_to_pixmap(photon: &PhotonImage) -> Option<tiny_skia::Pixmap> {
        let width = photon.get_width();
        let height = photon.get_height();
        let pixels = photon.get_raw_pixels();

        tiny_skia::Pixmap::from_vec(
            pixels,
            tiny_skia::IntSize::from_wh(width, height)?
        )
    }

    /// 将 Pixmap 数据写回 PhotonImage（优化版本，避免重复获取尺寸）
    pub fn pixmap_to_photon(pixmap: &tiny_skia::Pixmap) -> PhotonImage {
        let width = pixmap.width() as u32;
        let height = pixmap.height() as u32;
        // 使用 clone_from_vec 如果可能，减少拷贝
        let data = pixmap.data().to_vec();
        PhotonImage::new(data, width, height)
    }

    /// 从 Pixmap 数据直接创建 PhotonImage（用于已知尺寸的情况）
    #[allow(dead_code)]
    pub fn pixmap_to_photon_with_size(
        pixmap: &tiny_skia::Pixmap,
        width: u32,
        height: u32,
    ) -> PhotonImage {
        PhotonImage::new(pixmap.data().to_vec(), width, height)
    }
}

// ==================== 笔迹生成模块 ====================

pub struct StrokeGenerator {
    config: BrushConfig,
}

impl StrokeGenerator {
    pub fn new(config: BrushConfig) -> Self {
        Self { config }
    }

    /// 从输入点集生成矢量路径
    pub fn generate_path(&self, points: &[StrokePoint]) -> Option<tiny_skia::Path> {
        if points.is_empty() {
            return None;
        }

        match self.config.brush_type {
            BrushType::Basic => self.generate_basic_path(points),
            BrushType::Pencil => self.generate_pencil_path(points),
            BrushType::Marker => self.generate_marker_path(points),
            BrushType::Watercolor => self.generate_watercolor_path(points),
            BrushType::Eraser => self.generate_basic_path(points),
        }
    }

    /// 生成基础路径
    fn generate_basic_path(&self, points: &[StrokePoint]) -> Option<tiny_skia::Path> {
        if points.len() < 2 {
            // 至少需要两个点才能形成路径
            return None;
        }

        let mut path_builder = tiny_skia::PathBuilder::new();

        if let Some(first) = points.first() {
            path_builder.move_to(first.x, first.y);

            if self.config.smoothness > 0.0 {
                // 使用二次贝塞尔曲线进行平滑
                for i in 1..points.len() {
                    let current = &points[i];
                    if i < points.len() - 1 {
                        let next = &points[i + 1];
                        // 控制点是当前点，终点是当前和下一点的中点
                        let mid_x = (current.x + next.x) / 2.0;
                        let mid_y = (current.y + next.y) / 2.0;
                        path_builder.quad_to(current.x, current.y, mid_x, mid_y);
                    } else {
                        // 最后一个点直接连接
                        path_builder.line_to(current.x, current.y);
                    }
                }
            } else {
                for point in points.iter().skip(1) {
                    path_builder.line_to(point.x, point.y);
                }
            }
        }

        path_builder.finish()
    }

    /// 生成铅笔风格路径（带轻微抖动）
    fn generate_pencil_path(&self, points: &[StrokePoint]) -> Option<tiny_skia::Path> {
        if points.len() < 2 {
            // 至少需要两个点才能形成路径
            return None;
        }

        let mut path_builder = tiny_skia::PathBuilder::new();

        if let Some(first) = points.first() {
            path_builder.move_to(first.x, first.y);

            for i in 1..points.len() {
                let point = &points[i];
                // 添加轻微随机抖动
                let jitter = 0.5;
                let jx = ((js_sys::Math::random() - 0.5) * 2.0 * jitter) as f32;
                let jy = ((js_sys::Math::random() - 0.5) * 2.0 * jitter) as f32;
                path_builder.line_to(point.x + jx, point.y + jy);
            }
        }

        path_builder.finish()
    }

    /// 生成马克笔风格路径（更宽更平滑）
    fn generate_marker_path(&self, points: &[StrokePoint]) -> Option<tiny_skia::Path> {
        if points.len() < 2 {
            return None;
        }

        let mut path_builder = tiny_skia::PathBuilder::new();

        if let Some(first) = points.first() {
            path_builder.move_to(first.x, first.y);

            // 马克笔使用更强的平滑处理
            for i in 1..points.len() {
                let current = &points[i];
                if i < points.len() - 1 {
                    let next = &points[i + 1];
                    // 控制点和终点都使用中点，产生更平滑的曲线
                    let mid_x = (current.x + next.x) / 2.0;
                    let mid_y = (current.y + next.y) / 2.0;
                    let prev = &points[i - 1];
                    let ctrl_x = (prev.x + current.x) / 2.0;
                    let ctrl_y = (prev.y + current.y) / 2.0;
                    path_builder.quad_to(ctrl_x, ctrl_y, mid_x, mid_y);
                } else {
                    path_builder.line_to(current.x, current.y);
                }
            }
        }

        path_builder.finish()
    }

    /// 生成水彩笔风格路径（带透明度和扩散效果）
    fn generate_watercolor_path(&self, points: &[StrokePoint]) -> Option<tiny_skia::Path> {
        if points.len() < 2 {
            return None;
        }

        let mut path_builder = tiny_skia::PathBuilder::new();

        if let Some(first) = points.first() {
            path_builder.move_to(first.x, first.y);

            // 水彩笔使用较平滑的曲线
            for i in 1..points.len() {
                let current = &points[i];
                if i < points.len() - 1 {
                    let next = &points[i + 1];
                    let mid_x = (current.x + next.x) / 2.0;
                    let mid_y = (current.y + next.y) / 2.0;
                    path_builder.quad_to(current.x, current.y, mid_x, mid_y);
                } else {
                    path_builder.line_to(current.x, current.y);
                }
            }
        }

        path_builder.finish()
    }
}

// ==================== 渲染模块 ====================

pub struct BrushRenderer;

impl BrushRenderer {
    /// 渲染一笔到图像
    pub fn render_stroke(
        image: &mut PhotonImage,
        stroke: &BrushStroke,
    ) -> Result<(), String> {
        if stroke.points.len() < 2 {
            // 至少需要两个点才能绘制线条
            return Ok(());
        }

        // 1. 将 PhotonImage 转换为 Pixmap
        let mut pixmap = DataConverter::photon_to_pixmap(image)
            .ok_or("Failed to create pixmap")?;

        // 2. 获取路径（优先使用缓存）
        let path = match &stroke.cached_path {
            Some(cached) => cached.clone(),
            None => {
                let generator = StrokeGenerator::new(stroke.config);
                let new_path = generator.generate_path(&stroke.points)
                    .ok_or("Failed to generate path")?;
                new_path
            }
        };

        // 3. 配置画笔样式
        let paint = Self::create_paint(&stroke.config);
        let stroke_style = Self::create_stroke_style(&stroke.config);

        // 4. 渲染路径
        pixmap.stroke_path(
            &path,
            &paint,
            &stroke_style,
            tiny_skia::Transform::identity(),
            None,
        );

        // 5. 将结果写回 PhotonImage（使用已知尺寸避免重复获取）
        let width = pixmap.width() as u32;
        let height = pixmap.height() as u32;
        *image = DataConverter::pixmap_to_photon_with_size(&pixmap, width, height);

        Ok(())
    }

    /// 创建画笔
    fn create_paint(config: &BrushConfig) -> tiny_skia::Paint<'static> {
        let mut paint = tiny_skia::Paint::default();

        // 根据笔刷类型设置颜色和透明度
        let (r, g, b, alpha) = match config.brush_type {
            BrushType::Eraser => {
                // 橡皮擦：使用白色
                (255, 255, 255, 255)
            }
            BrushType::Watercolor => {
                // 水彩笔：半透明效果
                (config.color_r, config.color_g, config.color_b,
                 (config.color_a as f32 * 0.5) as u8)
            }
            BrushType::Marker => {
                // 马克笔：稍微半透明
                (config.color_r, config.color_g, config.color_b,
                 (config.color_a as f32 * 0.85) as u8)
            }
            _ => {
                // 其他笔刷
                (config.color_r, config.color_g, config.color_b, config.color_a)
            }
        };

        paint.set_color_rgba8(r, g, b, alpha);

        // 设置抗锯齿
        paint.anti_alias = true;

        // 设置混合模式
        paint.blend_mode = Self::map_blend_mode(config.blend_mode);

        paint
    }

    /// 创建描边样式
    fn create_stroke_style(config: &BrushConfig) -> tiny_skia::Stroke {
        match config.brush_type {
            BrushType::Pencil => {
                // 铅笔：较细，方形线帽
                tiny_skia::Stroke {
                    width: config.base_width * 0.7,
                    line_cap: tiny_skia::LineCap::Square,
                    line_join: tiny_skia::LineJoin::Miter,
                    miter_limit: 4.0,
                    ..Default::default()
                }
            }
            BrushType::Marker => {
                // 马克笔：更宽，圆形线帽
                tiny_skia::Stroke {
                    width: config.base_width * 1.5,
                    line_cap: tiny_skia::LineCap::Round,
                    line_join: tiny_skia::LineJoin::Round,
                    ..Default::default()
                }
            }
            BrushType::Watercolor => {
                // 水彩笔：宽且柔和
                tiny_skia::Stroke {
                    width: config.base_width * 2.0,
                    line_cap: tiny_skia::LineCap::Round,
                    line_join: tiny_skia::LineJoin::Round,
                    ..Default::default()
                }
            }
            BrushType::Eraser => {
                // 橡皮擦：与基础相同
                tiny_skia::Stroke {
                    width: config.base_width,
                    line_cap: tiny_skia::LineCap::Round,
                    line_join: tiny_skia::LineJoin::Round,
                    ..Default::default()
                }
            }
            BrushType::Basic => {
                // 基础画笔
                tiny_skia::Stroke {
                    width: config.base_width,
                    line_cap: tiny_skia::LineCap::Round,
                    line_join: tiny_skia::LineJoin::Round,
                    ..Default::default()
                }
            }
        }
    }

    /// 映射混合模式
    fn map_blend_mode(mode: BlendMode) -> tiny_skia::BlendMode {
        match mode {
            BlendMode::Normal => tiny_skia::BlendMode::SourceOver,
            BlendMode::Multiply => tiny_skia::BlendMode::Multiply,
            BlendMode::Screen => tiny_skia::BlendMode::Screen,
            BlendMode::Overlay => tiny_skia::BlendMode::Overlay,
            BlendMode::SoftLight => tiny_skia::BlendMode::SoftLight,
            BlendMode::HardLight => tiny_skia::BlendMode::HardLight,
            BlendMode::Difference => tiny_skia::BlendMode::Difference,
            BlendMode::Exclusion => tiny_skia::BlendMode::Exclusion,
            BlendMode::Lighten => tiny_skia::BlendMode::Lighten,
            BlendMode::Darken => tiny_skia::BlendMode::Darken,
        }
    }
}