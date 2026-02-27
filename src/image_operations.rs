use crate::types::ColorSpace;
use photon_rs::{PhotonImage, filters, native, monochrome, transform, effects, colour_spaces, text};
use base64::Engine;

/// 图像处理器 - 处理图像的各种操作
pub struct ImageProcessor {
    pub image: PhotonImage,
    pub original_image: PhotonImage,
    pub original_bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ImageProcessor {
    /// 创建新的图像处理器
    pub fn new(width: u32, height: u32, data: &[u8]) -> Result<ImageProcessor, String> {
        let image = PhotonImage::new(data.to_vec(), width, height);
        Ok(ImageProcessor {
            image: image.clone(),
            original_image: image,
            original_bytes: data.to_vec(),
            width,
            height,
        })
    }

    /// 从字节数据创建图像处理器
    pub fn new_from_bytes(bytes: &[u8]) -> Result<ImageProcessor, String> {
        let image = native::open_image_from_bytes(bytes)
            .map_err(|e| format!("Failed to open image: {}", e))?;
        let width = image.get_width();
        let height = image.get_height();
        Ok(ImageProcessor {
            image: image.clone(),
            original_image: image,
            original_bytes: bytes.to_vec(),
            width,
            height,
        })
    }

    /// 重置到原始图像
    pub fn reset(&mut self) {
        if let Ok(new_image) = native::open_image_from_bytes(&self.original_bytes) {
            self.original_image = new_image.clone();
            self.image = new_image;
        }
    }

    /// 转换为 Base64
    pub fn to_base64(&self) -> String {
        self.image.get_base64()
    }

    /// 获取宽度
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// 获取高度
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// 获取字节数据
    pub fn get_bytes(&self) -> Vec<u8> {
        self.image.get_bytes()
    }

    /// 获取原始像素数据（RGBA 格式，每个像素 4 字节）
    pub fn get_raw_pixels(&self) -> Vec<u8> {
        self.image.get_raw_pixels()
    }

    /// 获取估算的文件大小（字节）
    pub fn get_estimated_filesize(&self) -> u64 {
        self.image.get_estimated_filesize()
    }

    // ==================== 图像格式转换 ====================

    /// 转换为 JPEG
    pub fn to_jpeg(&mut self, quality: u8) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.image.get_bytes_jpeg(quality))
    }

    /// 转换为 PNG
    pub fn to_png(&mut self) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.image.get_bytes())
    }

    /// 转换为 WebP
    pub fn to_webp(&mut self, quality: u8) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.image.get_bytes_webp_with_quality(quality))
    }

    // ==================== 文本绘制功能 ====================

    /// 绘制文本，支持可选参数控制字体、阴影和颜色
    pub fn draw_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: f32,
        _font_type: Option<u8>,
        has_shadow: Option<bool>,
        color_r: Option<u8>,
        color_g: Option<u8>,
        color_b: Option<u8>,
    ) {
        let font_name = "default";
        let shadow = has_shadow.unwrap_or(false);
        let r = color_r.unwrap_or(0);
        let g = color_g.unwrap_or(0);
        let b = color_b.unwrap_or(0);

        match (shadow, color_r.is_some()) {
            (false, false) => text::draw_text(&mut self.image, text, x, y, font_size, font_name),
            (true, false) => text::draw_text_with_border(&mut self.image, text, x, y, font_size, font_name),
            (false, true) => text::draw_text_with_color(&mut self.image, text, x, y, font_size, r, g, b, font_name),
            (true, true) => text::draw_text_with_border_and_color(&mut self.image, text, x, y, font_size, r, g, b, font_name),
        }
    }

    /// 绘制带颜色的文本，支持选择字体类型（便捷方法）
    pub fn draw_text_with_color_and_font(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: f32,
        r: u8,
        g: u8,
        b: u8,
        _font_type: u8,
    ) {
        let font_name = "default";
        text::draw_text_with_color(&mut self.image, text, x, y, font_size, r, g, b, font_name);
    }

    /// 绘制带阴影和颜色的文本，支持选择字体类型（便捷方法）
    pub fn draw_text_with_shadow_and_color_and_font(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: f32,
        r: u8,
        g: u8,
        b: u8,
        _font_type: u8,
    ) {
        let font_name = "default";
        text::draw_text_with_border_and_color(&mut self.image, text, x, y, font_size, r, g, b, font_name);
    }

    /// 使用动态注册的字体名称绘制文本（支持阴影和颜色）
    pub fn draw_text_with_font_name(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: f32,
        font_name: &str,
        has_shadow: Option<bool>,
        color_r: Option<u8>,
        color_g: Option<u8>,
        color_b: Option<u8>,
    ) {
        let actual_font_name = if text::is_font_registered(font_name) {
            font_name
        } else {
            text::init_default_font();
            "default"
        };

        let shadow = has_shadow.unwrap_or(false);
        let r = color_r.unwrap_or(0);
        let g = color_g.unwrap_or(0);
        let b = color_b.unwrap_or(0);

        match (shadow, color_r.is_some()) {
            (false, false) => text::draw_text(&mut self.image, text, x, y, font_size, actual_font_name),
            (true, false) => text::draw_text_with_border(&mut self.image, text, x, y, font_size, actual_font_name),
            (false, true) => text::draw_text_with_color(&mut self.image, text, x, y, font_size, r, g, b, actual_font_name),
            (true, true) => text::draw_text_with_border_and_color(&mut self.image, text, x, y, font_size, r, g, b, actual_font_name),
        }
    }

    // ==================== 图像滤镜 ====================

    pub fn apply_grayscale(&mut self) {
        filters::filter(&mut self.image, "grayscale");
    }

    pub fn apply_sepia(&mut self) {
        filters::filter(&mut self.image, "sepia");
    }

    pub fn apply_invert(&mut self) {
        filters::filter(&mut self.image, "invert");
    }

    pub fn apply_threshold(&mut self, threshold: u32) {
        monochrome::threshold(&mut self.image, threshold);
    }

    pub fn apply_preset_filter(&mut self, filter_name: &str) {
        filters::filter(&mut self.image, filter_name);
    }

    pub fn apply_lix(&mut self) {
        filters::lix(&mut self.image);
    }

    pub fn apply_neue(&mut self) {
        filters::neue(&mut self.image);
    }

    pub fn apply_ryo(&mut self) {
        filters::ryo(&mut self.image);
    }

    // ==================== 特殊效果 ====================

    pub fn apply_pixelate(&mut self, pixel_size: i32) {
        effects::pixelize(&mut self.image, pixel_size);
    }

    pub fn apply_halftone(&mut self) {
        effects::halftone(&mut self.image);
    }

    pub fn apply_oil(&mut self, radius: i32, intensity: f64) {
        effects::oil(&mut self.image, radius, intensity);
    }

    pub fn apply_solarize(&mut self) {
        effects::solarize(&mut self.image);
    }

    pub fn apply_dither(&mut self, depth: u32) {
        effects::dither(&mut self.image, depth);
    }

    pub fn apply_duotone(&mut self, r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) {
        effects::duotone(&mut self.image, photon_rs::Rgb::new(r1, g1, b1), photon_rs::Rgb::new(r2, g2, b2));
    }

    pub fn apply_inc_brightness(&mut self, brightness: u8) {
        effects::inc_brightness(&mut self.image, brightness);
    }

    pub fn apply_gradient(&mut self) {
        photon_rs::multiple::apply_gradient(&mut self.image);
    }

    pub fn apply_primary(&mut self) {
        effects::primary(&mut self.image);
    }

    pub fn apply_colorize(&mut self) {
        effects::colorize(&mut self.image);
    }

    pub fn apply_frosted_glass(&mut self) {
        effects::frosted_glass(&mut self.image);
    }

    pub fn apply_tint(&mut self, r: u32, g: u32, b: u32) {
        effects::tint(&mut self.image, r, g, b);
    }

    pub fn apply_normalize(&mut self) {
        effects::normalize(&mut self.image);
    }

    pub fn offset_red(&mut self, offset_amt: u32) {
        effects::offset_red(&mut self.image, offset_amt);
    }

    pub fn offset_green(&mut self, offset_amt: u32) {
        effects::offset_green(&mut self.image, offset_amt);
    }

    pub fn offset_blue(&mut self, offset_amt: u32) {
        effects::offset_blue(&mut self.image, offset_amt);
    }

    // ==================== 条纹效果 ====================

    pub fn apply_strips(
        &mut self,
        num_strips: u8,
        horizontal: bool,
        color_r: Option<u8>,
        color_g: Option<u8>,
        color_b: Option<u8>,
    ) {
        let has_color = color_r.is_some();
        let r = color_r.unwrap_or(255);
        let g = color_g.unwrap_or(255);
        let b = color_b.unwrap_or(255);

        if has_color {
            if horizontal {
                effects::color_horizontal_strips(&mut self.image, num_strips, photon_rs::Rgb::new(r, g, b));
            } else {
                effects::color_vertical_strips(&mut self.image, num_strips, photon_rs::Rgb::new(r, g, b));
            }
        } else {
            if horizontal {
                effects::horizontal_strips(&mut self.image, num_strips);
            } else {
                effects::vertical_strips(&mut self.image, num_strips);
            }
        }
    }

    // ==================== 可调节参数的滤镜 ====================

    pub fn apply_brightness(&mut self, level: i32) {
        let clamped_level = level.clamp(-255, 255) as i16;
        effects::adjust_brightness(&mut self.image, clamped_level);
    }

    pub fn apply_contrast(&mut self, level: f32) {
        let clamped_level = level.clamp(-255.0, 255.0);
        effects::adjust_contrast(&mut self.image, clamped_level);
    }

    pub fn apply_saturation(&mut self, level: f32) {
        let clamped_level = level.clamp(-1.0, 1.0);
        
        if clamped_level >= 0.0 {
            colour_spaces::saturate_hsl(&mut self.image, clamped_level);
        } else {
            colour_spaces::desaturate_hsl(&mut self.image, -clamped_level);
        }
    }

    pub fn apply_hue(&mut self, level: i32) {
        let clamped_level = level.clamp(-360, 360);
        colour_spaces::hsl(&mut self.image, "shift_hue", clamped_level as f32);
    }

    pub fn apply_lightness(&mut self, level: f32, color_space: ColorSpace) {
        let clamped_level = level.clamp(-1.0, 1.0);

        match color_space {
            ColorSpace::Hsl => {
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_hsl(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_hsl(&mut self.image, -clamped_level);
                }
            }
            ColorSpace::Lch => {
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_lch(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_lch(&mut self.image, -clamped_level);
                }
            }
            ColorSpace::Hsv => {
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_hsv(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_hsv(&mut self.image, -clamped_level);
                }
            }
            ColorSpace::Hsluv => {
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_hsluv(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_hsluv(&mut self.image, -clamped_level);
                }
            }
        }
    }

    pub fn apply_gamma(&mut self, red: f32, green: f32, blue: f32) {
        let clamped_red = red.clamp(0.1, 10.0);
        let clamped_green = green.clamp(0.1, 10.0);
        let clamped_blue = blue.clamp(0.1, 10.0);
        colour_spaces::gamma_correction(&mut self.image, clamped_red, clamped_green, clamped_blue);
    }

    pub fn apply_sharpen(&mut self, strength: f32) {
        let clamped_strength = strength.clamp(0.0, 10.0);
        photon_rs::conv::sharpen_with_strength(&mut self.image, clamped_strength);
    }

    pub fn apply_noise_reduction(&mut self, strength: f32) {
        let clamped_strength = strength.clamp(0.0, 10.0);
        photon_rs::conv::noise_reduction_with_strength(&mut self.image, clamped_strength);
    }

    pub fn apply_bilateral_filter(&mut self, sigma_spatial: f32, sigma_range: f32, fast_mode: bool) {
        photon_rs::conv::bilateral_filter(&mut self.image, sigma_spatial, sigma_range, fast_mode);
    }

    // ==================== 噪点效果 ====================

    pub fn apply_noise(&mut self, strength: f32) {
        let clamped_strength = strength.clamp(0.0, 10.0);
        photon_rs::noise::add_noise_rand_with_strength(&mut self.image, clamped_strength);
    }

    pub fn apply_color_noise_with_strength(&mut self, r_factor: f32, g_factor: f32, b_factor: f32, strength: f32) {
        let r_factor = r_factor.clamp(0.0, 1.0);
        let g_factor = g_factor.clamp(0.0, 1.0);
        let b_factor = b_factor.clamp(0.0, 1.0);
        let strength = strength.clamp(0.0, 10.0);
        let max_offset = (15.0 * strength) as u8;
        
        if max_offset == 0 {
            return;
        }
        
        let mut pixels = self.image.get_raw_pixels();
        let pixel_count = pixels.len() / 4;
        let mut rng_state: u64 = (js_sys::Date::now() as u64).wrapping_mul(0x9e3779b97f4a7c15);
        
        for i in 0..pixel_count {
            let idx = i * 4;
            
            rng_state = rng_state.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(0xbf58476d1ce4e5b9);
            rng_state ^= rng_state >> 27;
            rng_state = rng_state.wrapping_mul(0x94d049bb133111eb);
            
            let r_offset = ((rng_state >> 16) & 0xFF) as u8 % max_offset;
            let g_offset = ((rng_state >> 24) & 0xFF) as u8 % max_offset;
            let b_offset = ((rng_state >> 32) & 0xFF) as u8 % max_offset;
            
            let new_r = (pixels[idx] as f32 + r_offset as f32 * r_factor).min(255.0) as u8;
            let new_g = (pixels[idx + 1] as f32 + g_offset as f32 * g_factor).min(255.0) as u8;
            let new_b = (pixels[idx + 2] as f32 + b_offset as f32 * b_factor).min(255.0) as u8;
            
            pixels[idx] = new_r;
            pixels[idx + 1] = new_g;
            pixels[idx + 2] = new_b;
        }
        
        self.image = photon_rs::PhotonImage::new(pixels, self.width, self.height);
    }

    pub fn apply_pink_noise(&mut self) {
        photon_rs::noise::pink_noise(&mut self.image);
    }

    // ==================== 批量应用多个调节 ====================

    pub fn apply_all_adjustments(
        &mut self,
        brightness: i32,
        contrast: f32,
        saturation: f32,
        hue: i32,
        lightness: f32,
        lightness_color_space: ColorSpace,
        gamma_red: f32,
        gamma_green: f32,
        gamma_blue: f32,
        sharpen_strength: f32,
        noise_reduction_strength: f32,
    ) {
        let mut temp_img = self.original_image.clone();

        if brightness != 0 {
            let clamped_brightness = brightness.clamp(-255, 255) as i16;
            effects::adjust_brightness(&mut temp_img, clamped_brightness);
        }

        if contrast != 0.0 {
            let clamped_contrast = contrast.clamp(-255.0, 255.0);
            effects::adjust_contrast(&mut temp_img, clamped_contrast);
        }

        if saturation != 0.0 {
            let clamped_saturation = saturation.clamp(-1.0, 1.0);
            if clamped_saturation >= 0.0 {
                colour_spaces::saturate_hsl(&mut temp_img, clamped_saturation);
            } else {
                colour_spaces::desaturate_hsl(&mut temp_img, -clamped_saturation);
            }
        }

        if hue != 0 {
            let clamped_hue = hue.clamp(-360, 360);
            colour_spaces::hsl(&mut temp_img, "shift_hue", clamped_hue as f32);
        }

        if lightness != 0.0 {
            let clamped_lightness = lightness.clamp(-1.0, 1.0);
            match lightness_color_space {
                ColorSpace::Hsl => {
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_hsl(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_hsl(&mut temp_img, -clamped_lightness);
                    }
                }
                ColorSpace::Lch => {
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_lch(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_lch(&mut temp_img, -clamped_lightness);
                    }
                }
                ColorSpace::Hsv => {
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_hsv(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_hsv(&mut temp_img, -clamped_lightness);
                    }
                }
                ColorSpace::Hsluv => {
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_hsluv(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_hsluv(&mut temp_img, -clamped_lightness);
                    }
                }
            }
        }

        if gamma_red != 1.0 || gamma_green != 1.0 || gamma_blue != 1.0 {
            let clamped_red = gamma_red.clamp(0.1, 10.0);
            let clamped_green = gamma_green.clamp(0.1, 10.0);
            let clamped_blue = gamma_blue.clamp(0.1, 10.0);
            colour_spaces::gamma_correction(&mut temp_img, clamped_red, clamped_green, clamped_blue);
        }

        if sharpen_strength != 0.0 {
            let clamped_strength = sharpen_strength.clamp(0.0, 10.0);
            photon_rs::conv::sharpen_with_strength(&mut temp_img, clamped_strength);
        }

        if noise_reduction_strength != 0.0 {
            let clamped_strength = noise_reduction_strength.clamp(0.0, 10.0);
            photon_rs::conv::noise_reduction_with_strength(&mut temp_img, clamped_strength);
        }

        self.image = temp_img;
    }

    // ==================== 变换操作 ====================

    pub fn rotate_90(&mut self) {
        self.image = transform::rotate(&self.image, 90.0);
        self.width = self.image.get_width();
        self.height = self.image.get_height();
    }

    pub fn rotate_any(&mut self, angle: f32) {
        if angle.abs() % 90.0 < 0.01 {
            let right_angle_count = ((angle.abs() as i32) / 90) % 4;
            let is_negative = angle < 0.0;
            
            for _ in 0..right_angle_count {
                if is_negative {
                    self.image = transform::rotate(&self.image, -90.0);
                } else {
                    self.image = transform::rotate(&self.image, 90.0);
                }
            }
        } else {
            self.image = transform::rotate(&self.image, angle);
        }
        self.width = self.image.get_width();
        self.height = self.image.get_height();
    }

    pub fn flip_horizontal(&mut self) {
        transform::fliph(&mut self.image);
    }

    pub fn flip_vertical(&mut self) {
        transform::flipv(&mut self.image);
    }

    pub fn crop(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.image = transform::crop(&self.image, x1, y1, x2, y2);
        self.width = self.image.get_width();
        self.height = self.image.get_height();
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.image = transform::resize(&self.image, new_width, new_height, photon_rs::transform::SamplingFilter::Nearest);
        self.width = self.image.get_width();
        self.height = self.image.get_height();
    }

    // ==================== 单色效果 ====================

    pub fn apply_b_grayscale(&mut self) {
        monochrome::b_grayscale(&mut self.image);
    }

    pub fn apply_desaturate(&mut self) {
        monochrome::desaturate(&mut self.image);
    }

    pub fn apply_decompose_max(&mut self) {
        monochrome::decompose_max(&mut self.image);
    }

    pub fn apply_decompose_min(&mut self) {
        monochrome::decompose_min(&mut self.image);
    }

    pub fn apply_grayscale_human_corrected(&mut self) {
        monochrome::grayscale_human_corrected(&mut self.image);
    }

    pub fn apply_grayscale_shades(&mut self, num_shades: u8) {
        monochrome::grayscale_shades(&mut self.image, num_shades);
    }

    // ==================== 模糊效果 ====================

    pub fn apply_box_blur(&mut self) {
        photon_rs::conv::box_blur(&mut self.image);
    }

    pub fn apply_gaussian_blur(&mut self, radius: i32) {
        photon_rs::conv::gaussian_blur(&mut self.image, radius);
    }

    // ==================== 边缘检测 ====================

    pub fn apply_sobel_horizontal(&mut self) {
        photon_rs::conv::sobel_horizontal(&mut self.image);
    }

    pub fn apply_sobel_vertical(&mut self) {
        photon_rs::conv::sobel_vertical(&mut self.image);
    }

    pub fn apply_sobel_global(&mut self) {
        photon_rs::conv::edge_detection(&mut self.image);
    }

    pub fn apply_prewitt_horizontal(&mut self) {
        photon_rs::conv::detect_horizontal_lines(&mut self.image);
    }

    // ==================== 卷积效果 ====================

    pub fn apply_laplace(&mut self) {
        photon_rs::conv::laplace(&mut self.image);
    }

    pub fn apply_emboss(&mut self) {
        photon_rs::conv::emboss(&mut self.image);
    }

    pub fn apply_identity(&mut self) {
        photon_rs::conv::identity(&mut self.image);
    }

    pub fn apply_edge_one(&mut self) {
        photon_rs::conv::edge_one(&mut self.image);
    }

    pub fn apply_edge_detection(&mut self) {
        photon_rs::conv::edge_detection(&mut self.image);
    }

    // ==================== 线条检测 ====================

    pub fn apply_detect_horizontal_lines(&mut self) {
        photon_rs::conv::detect_horizontal_lines(&mut self.image);
    }

    pub fn apply_detect_vertical_lines(&mut self) {
        photon_rs::conv::detect_vertical_lines(&mut self.image);
    }

    pub fn apply_detect_45_deg_lines(&mut self) {
        photon_rs::conv::detect_45_deg_lines(&mut self.image);
    }

    pub fn apply_detect_135_deg_lines(&mut self) {
        photon_rs::conv::detect_135_deg_lines(&mut self.image);
    }

    // ==================== 通道操作 ====================

    pub fn alter_red_channel(&mut self, amt: i16) {
        photon_rs::channels::alter_red_channel(&mut self.image, amt);
    }

    pub fn alter_green_channel(&mut self, amt: i16) {
        photon_rs::channels::alter_green_channel(&mut self.image, amt);
    }

    pub fn alter_blue_channel(&mut self, amt: i16) {
        photon_rs::channels::alter_blue_channel(&mut self.image, amt);
    }

    pub fn remove_red_channel(&mut self) {
        photon_rs::channels::remove_red_channel(&mut self.image, 0);
    }

    pub fn remove_green_channel(&mut self) {
        photon_rs::channels::remove_green_channel(&mut self.image, 0);
    }

    pub fn remove_blue_channel(&mut self) {
        photon_rs::channels::remove_blue_channel(&mut self.image, 0);
    }

    pub fn swap_rg_channels(&mut self) {
        photon_rs::channels::swap_channels(&mut self.image, 0, 1);
    }

    pub fn swap_gb_channels(&mut self) {
        photon_rs::channels::swap_channels(&mut self.image, 1, 2);
    }

    pub fn swap_rb_channels(&mut self) {
        photon_rs::channels::swap_channels(&mut self.image, 0, 2);
    }

    // ==================== 色彩空间 ====================

    pub fn hue_rotate(&mut self, degrees: f32, color_space: u8) {
        match color_space {
            1 => colour_spaces::hue_rotate_hsv(&mut self.image, degrees),
            2 => colour_spaces::hue_rotate_lch(&mut self.image, degrees),
            _ => colour_spaces::hue_rotate_hsl(&mut self.image, degrees),
        }
    }

    pub fn adjust_lightness(&mut self, level: f32) {
        if level >= 0.0 {
            colour_spaces::lighten_hsl(&mut self.image, level);
        } else {
            colour_spaces::darken_hsl(&mut self.image, -level);
        }
    }

    pub fn adjust_saturation(&mut self, level: f32) {
        if level >= 0.0 {
            colour_spaces::saturate_hsl(&mut self.image, level);
        } else {
            colour_spaces::desaturate_hsl(&mut self.image, -level);
        }
    }

    pub fn hue_rotate_hsl(&mut self, degrees: f32) {
        colour_spaces::hue_rotate_hsl(&mut self.image, degrees);
    }

    pub fn lighten_hsl(&mut self, level: f32) {
        colour_spaces::lighten_hsl(&mut self.image, level);
    }

    pub fn darken_hsl(&mut self, level: f32) {
        colour_spaces::darken_hsl(&mut self.image, level);
    }

    pub fn saturate_hsl(&mut self, level: f32) {
        colour_spaces::saturate_hsl(&mut self.image, level);
    }

    pub fn desaturate_hsl(&mut self, level: f32) {
        colour_spaces::desaturate_hsl(&mut self.image, level);
    }

    // ==================== 水印功能 ====================

    pub fn apply_watermark(
        &mut self,
        watermark_bytes: &[u8],
        x: i64,
        y: i64,
        scale: Option<f32>,
        opacity: Option<f32>,
        rotation: Option<f32>,
    ) {
        let scale = scale.unwrap_or(1.0);
        let opacity = opacity.unwrap_or(1.0);
        let rotation = rotation.unwrap_or(0.0);

        let watermark = native::open_image_from_bytes(watermark_bytes)
            .expect("Failed to open watermark image");

        let rotated_watermark = if rotation.abs() > 0.1 {
            transform::rotate(&watermark, rotation)
        } else {
            watermark
        };

        let final_watermark = if opacity < 0.9 {
            let mut pixels = rotated_watermark.get_bytes();
            let width = rotated_watermark.get_width();
            let height = rotated_watermark.get_height();

            if pixels.len() > 0 && width > 0 && height > 0 {
                let opacity_safe = opacity.max(0.01).min(1.0);
                for i in (3..pixels.len()).step_by(4) {
                    pixels[i] = ((pixels[i] as f32) * opacity_safe) as u8;
                }
                PhotonImage::new(pixels, width, height)
            } else {
                rotated_watermark
            }
        } else {
            rotated_watermark
        };

        let final_watermark = if final_watermark.get_width() > self.width || final_watermark.get_height() > self.height {
            let max_scale = (self.width as f32 / final_watermark.get_width() as f32)
                .min(self.height as f32 / final_watermark.get_height() as f32);
            let actual_scale = scale.min(max_scale);
            let new_width = (final_watermark.get_width() as f32 * actual_scale) as u32;
            let new_height = (final_watermark.get_height() as f32 * actual_scale) as u32;
            transform::resize(&final_watermark, new_width, new_height, transform::SamplingFilter::Lanczos3)
        } else if scale.abs() > 1.05 || scale.abs() < 0.95 {
            let new_width = (final_watermark.get_width() as f32 * scale) as u32;
            let new_height = (final_watermark.get_height() as f32 * scale) as u32;
            transform::resize(&final_watermark, new_width, new_height, transform::SamplingFilter::Lanczos3)
        } else {
            final_watermark
        };

        photon_rs::multiple::watermark(&mut self.image, &final_watermark, x, y);
    }

    pub fn apply_watermark_with_blend(
        &mut self,
        watermark_bytes: &[u8],
        x: i64,
        y: i64,
        scale: f32,
        _blend_mode: &str,
    ) {
        self.apply_watermark(watermark_bytes, x, y, Some(scale), None, None);
    }

    // ==================== 多图混合功能 ====================

    pub fn blend_images(
        &mut self,
        overlay_bytes: &[u8],
        blend_mode: &str,
        scale: Option<f32>,
    ) {
        let scale = scale.unwrap_or(1.0);
        let main_width = self.width;
        let main_height = self.height;

        let overlay_image = native::open_image_from_bytes(overlay_bytes)
            .expect("Failed to open overlay image");

        let scaled_width = if scale != 1.0 {
            (overlay_image.get_width() as f32 * scale).max(1.0) as u32
        } else {
            overlay_image.get_width()
        };
        let scaled_height = if scale != 1.0 {
            (overlay_image.get_height() as f32 * scale).max(1.0) as u32
        } else {
            overlay_image.get_height()
        };

        let scaled_overlay = if scale != 1.0 {
            transform::resize(
                &overlay_image,
                scaled_width,
                scaled_height,
                transform::SamplingFilter::Lanczos3
            )
        } else {
            overlay_image
        };

        let final_overlay = if scaled_width != main_width || scaled_height != main_height {
            transform::resize(
                &scaled_overlay,
                main_width,
                main_height,
                transform::SamplingFilter::Lanczos3
            )
        } else {
            scaled_overlay
        };

        photon_rs::multiple::blend(&mut self.image, &final_overlay, blend_mode);
    }

    pub fn blend_images_with_scale(
        &mut self,
        overlay_bytes: &[u8],
        scale: f32,
        blend_mode: &str,
    ) {
        self.blend_images(overlay_bytes, blend_mode, Some(scale));
    }

    // ==================== 边缘优化功能 ====================

    pub fn apply_polygon_mask(
        &mut self,
        vertices: Vec<f32>,
        anti_aliased: bool,
        smooth_edges: bool,
        smoothing_radius: u32,
    ) {
        use crate::edge_refinement::{create_polygon_mask, refine_mask_edges, apply_mask_to_image};
        
        let mut mask = create_polygon_mask(self.width, self.height, vertices, anti_aliased);

        if smooth_edges && smoothing_radius > 0 {
            refine_mask_edges(&mut mask, self.width, self.height, smoothing_radius);
        }

        let mut image_bytes = self.image.get_raw_pixels();
        apply_mask_to_image(&mut image_bytes, &mask, self.width, self.height);

        self.image = PhotonImage::new(image_bytes, self.width, self.height);
    }

    pub fn apply_circular_mask(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        feather_radius: f32,
    ) {
        use crate::edge_refinement::{create_circular_mask, apply_mask_to_image};
        
        let mask = create_circular_mask(
            self.width,
            self.height,
            center_x,
            center_y,
            radius,
            feather_radius,
        );

        let mut image_bytes = self.image.get_raw_pixels();
        apply_mask_to_image(&mut image_bytes, &mask, self.width, self.height);

        self.image = PhotonImage::new(image_bytes, self.width, self.height);
    }

    pub fn auto_crop_by_color(
        &mut self,
        target_r: u8,
        target_g: u8,
        target_b: u8,
        tolerance: u8,
        feather_radius: f32,
    ) {
        use crate::edge_refinement::{auto_crop_by_color, apply_mask_to_image};
        
        let mut image_bytes = self.image.get_raw_pixels();

        let mask = auto_crop_by_color(
            &image_bytes,
            self.width,
            self.height,
            target_r,
            target_g,
            target_b,
            tolerance,
            feather_radius,
        );

        apply_mask_to_image(&mut image_bytes, &mask, self.width, self.height);

        self.image = PhotonImage::new(image_bytes, self.width, self.height);
    }

    pub fn refine_edges(&mut self, smoothing_radius: u32) {
        use crate::edge_refinement::{refine_mask_edges, apply_mask_to_image};
        
        let image_bytes = self.image.get_raw_pixels();
        let pixel_count = (self.width * self.height) as usize;
        let mut mask = vec![0u8; pixel_count];

        for i in 0..pixel_count {
            let idx = i * 4;
            if idx + 3 < image_bytes.len() {
                mask[i] = image_bytes[idx + 3];
            }
        }

        if smoothing_radius > 0 {
            refine_mask_edges(&mut mask, self.width, self.height, smoothing_radius);
        }

        let mut image_bytes = self.image.get_raw_pixels();
        apply_mask_to_image(&mut image_bytes, &mask, self.width, self.height);

        self.image = PhotonImage::new(image_bytes, self.width, self.height);
    }

    pub fn smart_crop(&mut self, threshold: u8, feather_radius: f32) {
        use crate::edge_refinement::apply_mask_to_image;
        
        let mut mask = vec![0u8; (self.width * self.height) as usize];

        let image_bytes = self.image.get_raw_pixels();
        let pixel_count = (self.width * self.height) as usize;

        for i in 0..pixel_count {
            let idx = i * 4;
            if idx + 2 < image_bytes.len() {
                let brightness = (image_bytes[idx] as u16
                    + image_bytes[idx + 1] as u16
                    + image_bytes[idx + 2] as u16) / 3;

                if brightness < threshold as u16 {
                    mask[i] = 255;
                } else if feather_radius > 0.0 {
                    let normalized_dist = ((brightness as f32 - threshold as f32) / feather_radius).abs();
                    if normalized_dist < 1.0 {
                        mask[i] = ((1.0 - normalized_dist) * 255.0) as u8;
                    }
                }
            }
        }

        let mut image_bytes = self.image.get_raw_pixels();
        apply_mask_to_image(&mut image_bytes, &mask, self.width, self.height);

        self.image = PhotonImage::new(image_bytes, self.width, self.height);
    }
}