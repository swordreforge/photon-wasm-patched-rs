use wasm_bindgen::prelude::*;

/// 创建多边形遮罩
/// 
/// # 参数
/// * `width` - 图像宽度
/// * `height` - 图像高度
/// * `vertices` - 多边形顶点坐标数组 [x1, y1, x2, y2, ...]
/// * `anti_aliased` - 是否启用抗锯齿
/// 
/// # 返回
/// 遮罩像素数据（灰度图，每个像素 1 字节）
#[wasm_bindgen]
pub fn create_polygon_mask(
    width: u32,
    height: u32,
    vertices: Vec<f32>,
    anti_aliased: bool,
) -> Vec<u8> {
    let pixel_count = (width * height) as usize;
    let mut mask = vec![0u8; pixel_count];

    // 将顶点转换为点对
    let mut points = Vec::new();
    for i in (0..vertices.len()).step_by(2) {
        if i + 1 < vertices.len() {
            points.push((vertices[i], vertices[i + 1]));
        }
    }

    if points.len() < 3 {
        // 至少需要3个点才能形成多边形
        return mask;
    }

    if anti_aliased {
        // 使用抗锯齿的扫描线算法
        create_aa_polygon_mask(&mut mask, width, height, &points);
    } else {
        // 使用传统扫描线填充算法
        create_polygon_mask_naive(&mut mask, width, height, &points);
    }

    mask
}

/// 创建抗锯齿多边形遮罩
fn create_aa_polygon_mask(mask: &mut [u8], width: u32, height: u32, points: &[(f32, f32)]) {
    use std::cmp::{max, min};

    let width = width as usize;
    let height = height as usize;

    // 遍历每一行
    for y in 0..height {
        let mut intersections = Vec::new();

        // 计算该行与多边形所有边的交点
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % points.len()];

            // 检查边是否与当前行相交
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            if y as f32 >= min_y && y as f32 <= max_y {
                // 计算交点的 x 坐标
                if (y2 - y1).abs() > 0.001 {
                    let t = (y as f32 - y1) / (y2 - y1);
                    let x = x1 + t * (x2 - x1);
                    intersections.push((x, t));
                }
            }
        }

        // 排序交点
        intersections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // 填充交点之间的区域
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i].0;
                let x_end = intersections[i + 1].0;

                // 计算抗锯齿的 Alpha 值
                let x_start_idx = max(0, x_start.floor() as i32) as usize;
                let x_end_idx = min(width, x_end.ceil() as usize) as usize;

                for x in x_start_idx..x_end_idx {
                    let idx = y * width + x;
                    if idx < mask.len() {
                        // 计算该像素到边缘的距离
                        let dist_left = x as f32 - x_start;
                        let dist_right = x_end - x as f32;
                        let min_dist = dist_left.min(dist_right);

                        // 边缘羽化（2像素）
                        let feather = 2.0;
                        let alpha = if min_dist >= feather {
                            255
                        } else {
                            ((min_dist / feather) * 255.0).min(255.0).max(0.0) as u8
                        };

                        mask[idx] = alpha.max(mask[idx]);
                    }
                }
            }
        }
    }
}

/// 创建简单多边形遮罩（无抗锯齿）
fn create_polygon_mask_naive(mask: &mut [u8], width: u32, height: u32, points: &[(f32, f32)]) {
    let width = width as usize;
    let height = height as usize;

    // 使用射线法判断点是否在多边形内
    for y in 0..height {
        for x in 0..width {
            let point_inside = point_in_polygon(x as f32, y as f32, points);
            if point_inside {
                let idx = y * width + x;
                mask[idx] = 255;
            }
        }
    }
}

/// 判断点是否在多边形内（射线法）
fn point_in_polygon(x: f32, y: f32, points: &[(f32, f32)]) -> bool {
    let mut inside = false;
    let n = points.len();

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        // 检查点是否在边的同一侧
        if (y1 > y) != (y2 > y) {
            let x_intersect = (x2 - x1) * (y - y1) / (y2 - y1) + x1;
            if x < x_intersect {
                inside = !inside;
            }
        }
    }

    inside
}

/// 创建圆形遮罩（带抗锯齿）
#[wasm_bindgen]
pub fn create_circular_mask(
    width: u32,
    height: u32,
    center_x: f32,
    center_y: f32,
    radius: f32,
    feather_radius: f32,
) -> Vec<u8> {
    let pixel_count = (width * height) as usize;
    let mut mask = vec![0u8; pixel_count];

    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();

            let idx = (y * width + x) as usize;

            let alpha = if distance <= radius - feather_radius {
                255.0
            } else if distance >= radius + feather_radius {
                0.0
            } else {
                // 线性插值实现羽化
                let normalized_dist = (distance - (radius - feather_radius)) / (2.0 * feather_radius);
                255.0 * (1.0 - normalized_dist)
            };

            mask[idx] = alpha.max(0.0).min(255.0) as u8;
        }
    }

    mask
}

/// 应用遮罩到图像
#[wasm_bindgen]
pub fn apply_mask_to_image(image_bytes: &mut [u8], mask: &[u8], width: u32, height: u32) {
    let pixel_count = (width * height) as usize;

    for i in 0..pixel_count {
        let idx = i * 4; // RGBA
        if idx + 3 < image_bytes.len() && i < mask.len() {
            // 将遮罩的灰度值作为 Alpha 通道
            image_bytes[idx + 3] = mask[i];
        }
    }
}

/// 自动边缘优化 - 对遮罩进行平滑处理（使用可分离高斯模糊优化）
#[wasm_bindgen]
pub fn refine_mask_edges(
    mask: &mut [u8],
    width: u32,
    height: u32,
    smoothing_radius: u32,
) {
    let width = width as usize;
    let height = height as usize;
    let radius = smoothing_radius as usize;

    if radius == 0 {
        return;
    }

    // 创建临时缓冲区
    let mut temp = vec![0u8; width * height];

    // 第一步：水平方向模糊
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0u32;
            let mut count = 0u32;

            for dx in -(radius as i32)..=(radius as i32) {
                let nx = (x as i32 + dx) as usize;
                if nx < width {
                    let idx = y * width + nx;
                    sum += mask[idx] as u32;
                    count += 1;
                }
            }

            temp[y * width + x] = (sum / count) as u8;
        }
    }

    // 第二步：垂直方向模糊（从 temp 读，写入 mask）
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0u32;
            let mut count = 0u32;

            for dy in -(radius as i32)..=(radius as i32) {
                let ny = (y as i32 + dy) as usize;
                if ny < height {
                    let idx = ny * width + x;
                    sum += temp[idx] as u32;
                    count += 1;
                }
            }

            mask[y * width + x] = (sum / count) as u8;
        }
    }
}

/// 自动抠图 - 基于颜色的智能抠图
#[wasm_bindgen]
pub fn auto_crop_by_color(
    image_bytes: &[u8],
    width: u32,
    height: u32,
    target_r: u8,
    target_g: u8,
    target_b: u8,
    tolerance: u8,
    feather_radius: f32,
) -> Vec<u8> {
    let pixel_count = (width * height) as usize;
    let mut mask = vec![0u8; pixel_count];

    let tolerance_sq = tolerance as u32 * tolerance as u32;

    for i in 0..pixel_count {
        let idx = i * 4;
        if idx + 2 < image_bytes.len() {
            let r = image_bytes[idx] as i32;
            let g = image_bytes[idx + 1] as i32;
            let b = image_bytes[idx + 2] as i32;

            let dr = (r - target_r as i32).abs();
            let dg = (g - target_g as i32).abs();
            let db = (b - target_b as i32).abs();

            let distance_sq = (dr * dr + dg * dg + db * db) as u32;

            // 如果颜色在容差范围内，保留（Alpha=255）
            if distance_sq <= tolerance_sq {
                mask[i] = 255;
            } else if feather_radius > 0.0 {
                // 边缘羽化
                let distance = (distance_sq as f32).sqrt();
                let normalized_dist = (distance - tolerance as f32) / feather_radius;
                if normalized_dist < 1.0 {
                    mask[i] = ((1.0 - normalized_dist) * 255.0) as u8;
                } else {
                    mask[i] = 0;
                }
            } else {
                mask[i] = 0;
            }
        }
    }

    mask
}