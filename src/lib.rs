use wasm_bindgen::prelude::*;
use photon_rs::{PhotonImage, filters, native, monochrome, transform, effects, colour_spaces, text};
use base64::Engine;

#[wasm_bindgen]
pub struct ImageProcessor {
    image: PhotonImage,
    original_image: PhotonImage,
    original_bytes: Vec<u8>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl ImageProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: &[u8]) -> Result<ImageProcessor, JsValue> {
        let image = PhotonImage::new(data.to_vec(), width, height);
        Ok(ImageProcessor {
            image: image.clone(),
            original_image: image,
            original_bytes: data.to_vec(),
            width,
            height,
        })
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Result<ImageProcessor, JsValue> {
        let image = native::open_image_from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to open image: {}", e)))?;
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

    pub fn to_base64(&self) -> String {
        self.image.get_base64()
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.image.get_bytes()
    }

    // 获取估算的文件大小（字节）
    pub fn get_estimated_filesize(&self) -> u64 {
        self.image.get_estimated_filesize()
    }

    // 图像格式转换
    pub fn to_jpeg(&mut self, quality: u8) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.image.get_bytes_jpeg(quality))
    }

    pub fn to_png(&mut self) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.image.get_bytes())
    }

    pub fn to_webp(&mut self, quality: u8) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.image.get_bytes_webp_with_quality(quality))
    }

    // 文本绘制功能
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, font_size: f32) {
        text::draw_text(&mut self.image, text, x, y, font_size);
    }

    /// 绘制文本，支持选择字体类型
    /// font_type: 0-2, 对应不同的字体
    pub fn draw_text_with_font(&mut self, text: &str, x: i32, y: i32, font_size: f32, font_type: u8) {
        let ft = match font_type {
            1 => text::FontType::AlibabaRegular,
            2 => text::FontType::HongLeiXiaoZhiTiao,
            _ => text::FontType::RobotoRegular,
        };
        text::draw_text_with_font(&mut self.image, text, x, y, font_size, ft);
    }

    pub fn draw_text_with_shadow(&mut self, text: &str, x: i32, y: i32, font_size: f32) {
        text::draw_text_with_border(&mut self.image, text, x, y, font_size);
    }

    /// 绘制带阴影的文本，支持选择字体类型
    /// font_type: 0-2, 对应不同的字体
    pub fn draw_text_with_shadow_and_font(&mut self, text: &str, x: i32, y: i32, font_size: f32, font_type: u8) {
        let ft = match font_type {
            1 => text::FontType::AlibabaRegular,
            2 => text::FontType::HongLeiXiaoZhiTiao,
            _ => text::FontType::RobotoRegular,
        };
        text::draw_text_with_border_with_font(&mut self.image, text, x, y, font_size, ft);
    }

    pub fn draw_text_with_color(&mut self, text: &str, x: i32, y: i32, font_size: f32, r: u8, g: u8, b: u8) {
        text::draw_text_with_color(&mut self.image, text, x, y, font_size, r, g, b);
    }

    /// 绘制带颜色的文本，支持选择字体类型
    /// font_type: 0-2, 对应不同的字体
    pub fn draw_text_with_color_and_font(&mut self, text: &str, x: i32, y: i32, font_size: f32, r: u8, g: u8, b: u8, font_type: u8) {
        let ft = match font_type {
            1 => text::FontType::AlibabaRegular,
            2 => text::FontType::HongLeiXiaoZhiTiao,
            _ => text::FontType::RobotoRegular,
        };
        text::draw_text_with_color_and_font(&mut self.image, text, x, y, font_size, r, g, b, ft);
    }

    pub fn draw_text_with_shadow_and_color(&mut self, text: &str, x: i32, y: i32, font_size: f32, r: u8, g: u8, b: u8) {
        text::draw_text_with_border_and_color(&mut self.image, text, x, y, font_size, r, g, b);
    }

    /// 绘制带阴影和颜色的文本，支持选择字体类型
    /// font_type: 0-2, 对应不同的字体
    pub fn draw_text_with_shadow_and_color_and_font(&mut self, text: &str, x: i32, y: i32, font_size: f32, r: u8, g: u8, b: u8, font_type: u8) {
        let ft = match font_type {
            1 => text::FontType::AlibabaRegular,
            2 => text::FontType::HongLeiXiaoZhiTiao,
            _ => text::FontType::RobotoRegular,
        };
        text::draw_text_with_border_and_color_and_font(&mut self.image, text, x, y, font_size, r, g, b, ft);
    }

    // 图像滤镜
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

    // 更多预设滤镜
    pub fn apply_preset_filter(&mut self, filter_name: &str) {
        filters::filter(&mut self.image, filter_name);
    }

    // 特殊效果
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

    // 重置到原始图像
    pub fn reset(&mut self) {
        // 使用 open_image_from_bytes 重新从原始字节创建图像
        // 注意: 这里需要从原始字节数组中重建图像，但 original_bytes 是加载时的完整图像数据
        // 由于 PhotonImage 没有直接从字节数组克隆的方法，我们需要创建一个新的 ImageProcessor
        if let Ok(new_image) = native::open_image_from_bytes(&self.original_bytes) {
            self.image = new_image.clone();
            self.original_image = new_image;
        }
    }

    // 可调节参数的滤镜 - 直接调用 photon-rs 函数
    pub fn apply_brightness(&mut self, level: i32) {
        // brightness 范围: -255 到 255
        let clamped_level = level.clamp(-255, 255) as i16;
        effects::adjust_brightness(&mut self.image, clamped_level);
    }

    pub fn apply_contrast(&mut self, level: f32) {
        // contrast 范围: -255 到 255 (JavaScript 端已转换)
        let clamped_level = level.clamp(-255.0, 255.0);
        effects::adjust_contrast(&mut self.image, clamped_level);
    }

    pub fn apply_saturation(&mut self, level: f32) {
        // saturation 范围: -1 到 1 (JavaScript 端已转换)
        let clamped_level = level.clamp(-1.0, 1.0);
        
        if clamped_level >= 0.0 {
            colour_spaces::saturate_hsl(&mut self.image, clamped_level);
        } else {
            colour_spaces::desaturate_hsl(&mut self.image, -clamped_level);
        }
    }

    pub fn apply_hue(&mut self, level: i32) {
        // hue 范围: -360 到 360
        let clamped_level = level.clamp(-360, 360);
        colour_spaces::hsl(&mut self.image, "shift_hue", clamped_level as f32);
    }

    pub fn apply_lightness(&mut self, level: f32, color_space: &str) {
        // lightness 范围: -1 到 1 (负值变暗，正值变亮)
        let clamped_level = level.clamp(-1.0, 1.0);
        
        match color_space {
            "hsl" => {
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_hsl(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_hsl(&mut self.image, -clamped_level);
                }
            }
            "lch" => {
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_lch(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_lch(&mut self.image, -clamped_level);
                }
            }
            "hsv" => {
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_hsv(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_hsv(&mut self.image, -clamped_level);
                }
            }
            "hsluv" => {
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_hsluv(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_hsluv(&mut self.image, -clamped_level);
                }
            }
            _ => {
                // 默认使用 HSL
                if clamped_level >= 0.0 {
                    colour_spaces::lighten_hsl(&mut self.image, clamped_level);
                } else {
                    colour_spaces::darken_hsl(&mut self.image, -clamped_level);
                }
            }
        }
    }

    pub fn apply_gamma(&mut self, red: f32, green: f32, blue: f32) {
        // gamma 范围: 0.1 到 10.0，1.0 表示无变化
        let clamped_red = red.clamp(0.1, 10.0);
        let clamped_green = green.clamp(0.1, 10.0);
        let clamped_blue = blue.clamp(0.1, 10.0);
        colour_spaces::gamma_correction(&mut self.image, clamped_red, clamped_green, clamped_blue);
    }

    pub fn apply_sharpen(&mut self, strength: f32) {
        // strength 范围: 0.0 到 10.0
        let clamped_strength = strength.clamp(0.0, 10.0);
        photon_rs::conv::sharpen_with_strength(&mut self.image, clamped_strength);
    }

    pub fn apply_noise_reduction(&mut self, strength: f32) {
        // strength 范围: 0.0 到 10.0
        let clamped_strength = strength.clamp(0.0, 10.0);
        photon_rs::conv::noise_reduction_with_strength(&mut self.image, clamped_strength);
    }

    // 批量应用多个调节（避免重复重置）
    pub fn apply_all_adjustments(&mut self, brightness: i32, contrast: f32, saturation: f32, hue: i32, lightness: f32, lightness_color_space: &str, gamma_red: f32, gamma_green: f32, gamma_blue: f32, sharpen_strength: f32, noise_reduction_strength: f32) {
        // 基于原始图像应用所有调整
        let mut temp_img = self.original_image.clone();

        // 亮度: 输入 -255 到 255
        if brightness != 0 {
            let clamped_brightness = brightness.clamp(-255, 255) as i16;
            effects::adjust_brightness(&mut temp_img, clamped_brightness);
        }

        // 对比度: 输入 -255 到 255 (已经转换过)
        if contrast != 0.0 {
            let clamped_contrast = contrast.clamp(-255.0, 255.0);
            effects::adjust_contrast(&mut temp_img, clamped_contrast);
        }

        // 饱和度: 输入 -1 到 1 (已经转换过)
        if saturation != 0.0 {
            let clamped_saturation = saturation.clamp(-1.0, 1.0);
            if clamped_saturation >= 0.0 {
                colour_spaces::saturate_hsl(&mut temp_img, clamped_saturation);
            } else {
                colour_spaces::desaturate_hsl(&mut temp_img, -clamped_saturation);
            }
        }

        // 色相: 输入 -360 到 360
        if hue != 0 {
            let clamped_hue = hue.clamp(-360, 360);
            colour_spaces::hsl(&mut temp_img, "shift_hue", clamped_hue as f32);
        }

        // 明度: 输入 -1 到 1
        if lightness != 0.0 {
            let clamped_lightness = lightness.clamp(-1.0, 1.0);
            match lightness_color_space {
                "hsl" => {
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_hsl(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_hsl(&mut temp_img, -clamped_lightness);
                    }
                }
                "lch" => {
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_lch(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_lch(&mut temp_img, -clamped_lightness);
                    }
                }
                "hsv" => {
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_hsv(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_hsv(&mut temp_img, -clamped_lightness);
                    }
                }
                "hsluv" => {
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_hsluv(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_hsluv(&mut temp_img, -clamped_lightness);
                    }
                }
                _ => {
                    // 默认使用 HSL
                    if clamped_lightness >= 0.0 {
                        colour_spaces::lighten_hsl(&mut temp_img, clamped_lightness);
                    } else {
                        colour_spaces::darken_hsl(&mut temp_img, -clamped_lightness);
                    }
                }
            }
        }

        // 伽马校正
        if gamma_red != 1.0 || gamma_green != 1.0 || gamma_blue != 1.0 {
            let clamped_red = gamma_red.clamp(0.1, 10.0);
            let clamped_green = gamma_green.clamp(0.1, 10.0);
            let clamped_blue = gamma_blue.clamp(0.1, 10.0);
            colour_spaces::gamma_correction(&mut temp_img, clamped_red, clamped_green, clamped_blue);
        }

        // 锐化
        if sharpen_strength != 0.0 {
            let clamped_strength = sharpen_strength.clamp(0.0, 10.0);
            photon_rs::conv::sharpen_with_strength(&mut temp_img, clamped_strength);
        }

        // 降噪
        if noise_reduction_strength != 0.0 {
            let clamped_strength = noise_reduction_strength.clamp(0.0, 10.0);
            photon_rs::conv::noise_reduction_with_strength(&mut temp_img, clamped_strength);
        }

        self.image = temp_img;
    }

    // 变换操作
    pub fn rotate_90(&mut self) {
        self.image = transform::rotate(&self.image, 90.0);
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

    // 单色效果
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

    // 模糊效果
    pub fn apply_box_blur(&mut self) {
        photon_rs::conv::box_blur(&mut self.image);
    }

    pub fn apply_gaussian_blur(&mut self, radius: i32) {
        photon_rs::conv::gaussian_blur(&mut self.image, radius);
    }

    // 边缘检测
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

    // 卷积效果
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

    // 线条检测
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

    // 特殊效果
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

    // 条纹效果
    pub fn apply_horizontal_strips(&mut self, num_strips: u8) {
        effects::horizontal_strips(&mut self.image, num_strips);
    }

    pub fn apply_vertical_strips(&mut self, num_strips: u8) {
        effects::vertical_strips(&mut self.image, num_strips);
    }

    pub fn apply_color_horizontal_strips(&mut self, num_strips: u8, r: u8, g: u8, b: u8) {
        effects::color_horizontal_strips(&mut self.image, num_strips, photon_rs::Rgb::new(r, g, b));
    }

    pub fn apply_color_vertical_strips(&mut self, num_strips: u8, r: u8, g: u8, b: u8) {
        effects::color_vertical_strips(&mut self.image, num_strips, photon_rs::Rgb::new(r, g, b));
    }

    // 通道调整
    pub fn offset_red(&mut self, offset_amt: u32) {
        effects::offset_red(&mut self.image, offset_amt);
    }

    pub fn offset_green(&mut self, offset_amt: u32) {
        effects::offset_green(&mut self.image, offset_amt);
    }

    pub fn offset_blue(&mut self, offset_amt: u32) {
        effects::offset_blue(&mut self.image, offset_amt);
    }

    // 归一化
    pub fn apply_normalize(&mut self) {
        effects::normalize(&mut self.image);
    }

    // ==================== Channels（通道操作）====================

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

    // ==================== Colour Spaces（色彩空间）====================

    pub fn hue_rotate_hsl(&mut self, degrees: f32) {
        colour_spaces::hue_rotate_hsl(&mut self.image, degrees);
    }

    pub fn hue_rotate_hsv(&mut self, degrees: f32) {
        colour_spaces::hue_rotate_hsv(&mut self.image, degrees);
    }

    pub fn hue_rotate_lch(&mut self, degrees: f32) {
        colour_spaces::hue_rotate_lch(&mut self.image, degrees);
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

    // ==================== Effects（特效）====================

    pub fn apply_lix(&mut self) {
        filters::lix(&mut self.image);
    }

    pub fn apply_neue(&mut self) {
        filters::neue(&mut self.image);
    }

    pub fn apply_ryo(&mut self) {
        filters::ryo(&mut self.image);
    }

    pub fn apply_inc_brightness(&mut self, brightness: u8) {
        effects::inc_brightness(&mut self.image, brightness);
    }

    pub fn apply_gradient(&mut self) {
        photon_rs::multiple::apply_gradient(&mut self.image);
    }

    // 水印功能
    pub fn apply_watermark(&mut self, watermark_bytes: &[u8], x: i64, y: i64) {
        let watermark = native::open_image_from_bytes(watermark_bytes)
            .expect("Failed to open watermark image");
        photon_rs::multiple::watermark(&mut self.image, &watermark, x, y);
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}