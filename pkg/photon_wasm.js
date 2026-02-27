/* @ts-self-types="./photon_wasm.d.ts" */
import { startWorkers } from './snippets/wasm-bindgen-rayon-38edf6e439f6d70d/src/workerHelpers.js';

/**
 * 混合模式
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}
 */
export const BlendMode = Object.freeze({
    Normal: 0, "0": "Normal",
    Multiply: 1, "1": "Multiply",
    Screen: 2, "2": "Screen",
    Overlay: 3, "3": "Overlay",
    SoftLight: 4, "4": "SoftLight",
    HardLight: 5, "5": "HardLight",
    Difference: 6, "6": "Difference",
    Exclusion: 7, "7": "Exclusion",
    Lighten: 8, "8": "Lighten",
    Darken: 9, "9": "Darken",
});

/**
 * 笔刷配置
 */
export class BrushConfig {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(BrushConfig.prototype);
        obj.__wbg_ptr = ptr;
        BrushConfigFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BrushConfigFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_brushconfig_free(ptr, 0);
    }
    /**
     * 创建默认基础画笔配置
     * @returns {BrushConfig}
     */
    static default_basic() {
        const ret = wasm.brushconfig_default_basic();
        return BrushConfig.__wrap(ret);
    }
    /**
     * @param {BrushType} brush_type
     * @param {number} base_width
     * @param {number} color_r
     * @param {number} color_g
     * @param {number} color_b
     * @param {number} color_a
     * @param {BlendMode} blend_mode
     * @param {number} smoothness
     * @param {number} pressure_sensitivity
     */
    constructor(brush_type, base_width, color_r, color_g, color_b, color_a, blend_mode, smoothness, pressure_sensitivity) {
        const ret = wasm.brushconfig_new(brush_type, base_width, color_r, color_g, color_b, color_a, blend_mode, smoothness, pressure_sensitivity);
        this.__wbg_ptr = ret >>> 0;
        BrushConfigFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * 基础宽度（像素）
     * @returns {number}
     */
    get base_width() {
        const ret = wasm.__wbg_get_brushconfig_base_width(this.__wbg_ptr);
        return ret;
    }
    /**
     * 混合模式
     * @returns {BlendMode}
     */
    get blend_mode() {
        const ret = wasm.__wbg_get_brushconfig_blend_mode(this.__wbg_ptr);
        return ret;
    }
    /**
     * 笔刷类型
     * @returns {BrushType}
     */
    get brush_type() {
        const ret = wasm.__wbg_get_brushconfig_brush_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get color_a() {
        const ret = wasm.__wbg_get_brushconfig_color_a(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get color_b() {
        const ret = wasm.__wbg_get_brushconfig_color_b(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get color_g() {
        const ret = wasm.__wbg_get_brushconfig_color_g(this.__wbg_ptr);
        return ret;
    }
    /**
     * 颜色（RGBA）
     * @returns {number}
     */
    get color_r() {
        const ret = wasm.__wbg_get_brushconfig_color_r(this.__wbg_ptr);
        return ret;
    }
    /**
     * 压感强度（0.0 - 1.0）
     * @returns {number}
     */
    get pressure_sensitivity() {
        const ret = wasm.__wbg_get_brushconfig_pressure_sensitivity(this.__wbg_ptr);
        return ret;
    }
    /**
     * 平滑度（0.0 - 1.0）
     * @returns {number}
     */
    get smoothness() {
        const ret = wasm.__wbg_get_brushconfig_smoothness(this.__wbg_ptr);
        return ret;
    }
    /**
     * 基础宽度（像素）
     * @param {number} arg0
     */
    set base_width(arg0) {
        wasm.__wbg_set_brushconfig_base_width(this.__wbg_ptr, arg0);
    }
    /**
     * 混合模式
     * @param {BlendMode} arg0
     */
    set blend_mode(arg0) {
        wasm.__wbg_set_brushconfig_blend_mode(this.__wbg_ptr, arg0);
    }
    /**
     * 笔刷类型
     * @param {BrushType} arg0
     */
    set brush_type(arg0) {
        wasm.__wbg_set_brushconfig_brush_type(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set color_a(arg0) {
        wasm.__wbg_set_brushconfig_color_a(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set color_b(arg0) {
        wasm.__wbg_set_brushconfig_color_b(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set color_g(arg0) {
        wasm.__wbg_set_brushconfig_color_g(this.__wbg_ptr, arg0);
    }
    /**
     * 颜色（RGBA）
     * @param {number} arg0
     */
    set color_r(arg0) {
        wasm.__wbg_set_brushconfig_color_r(this.__wbg_ptr, arg0);
    }
    /**
     * 压感强度（0.0 - 1.0）
     * @param {number} arg0
     */
    set pressure_sensitivity(arg0) {
        wasm.__wbg_set_brushconfig_pressure_sensitivity(this.__wbg_ptr, arg0);
    }
    /**
     * 平滑度（0.0 - 1.0）
     * @param {number} arg0
     */
    set smoothness(arg0) {
        wasm.__wbg_set_brushconfig_smoothness(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) BrushConfig.prototype[Symbol.dispose] = BrushConfig.prototype.free;

/**
 * 笔划数据
 */
export class BrushStroke {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BrushStrokeFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_brushstroke_free(ptr, 0);
    }
    /**
     * 添加一个点到笔划
     * @param {StrokePoint} point
     */
    add_point(point) {
        _assertClass(point, StrokePoint);
        var ptr0 = point.__destroy_into_raw();
        wasm.brushstroke_add_point(this.__wbg_ptr, ptr0);
    }
    /**
     * 清除所有点
     */
    clear_points() {
        wasm.brushstroke_clear_points(this.__wbg_ptr);
    }
    /**
     * 获取笔刷配置
     * @returns {BrushConfig}
     */
    get_config() {
        const ret = wasm.brushstroke_get_config(this.__wbg_ptr);
        return BrushConfig.__wrap(ret);
    }
    /**
     * 获取点数量
     * @returns {number}
     */
    get_points_count() {
        const ret = wasm.brushstroke_get_points_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * 清除路径缓存（当点集改变时调用）
     */
    invalidate_path_cache() {
        wasm.brushstroke_invalidate_path_cache(this.__wbg_ptr);
    }
    /**
     * @param {BrushConfig} config
     */
    constructor(config) {
        _assertClass(config, BrushConfig);
        var ptr0 = config.__destroy_into_raw();
        const ret = wasm.brushstroke_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        BrushStrokeFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * 获取点数量
     * @returns {number}
     */
    point_count() {
        const ret = wasm.brushstroke_point_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * 笔刷配置
     * @returns {BrushConfig}
     */
    get config() {
        const ret = wasm.__wbg_get_brushstroke_config(this.__wbg_ptr);
        return BrushConfig.__wrap(ret);
    }
    /**
     * 笔刷配置
     * @param {BrushConfig} arg0
     */
    set config(arg0) {
        _assertClass(arg0, BrushConfig);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_brushstroke_config(this.__wbg_ptr, ptr0);
    }
}
if (Symbol.dispose) BrushStroke.prototype[Symbol.dispose] = BrushStroke.prototype.free;

/**
 * 笔刷类型
 * @enum {0 | 1 | 2 | 3 | 4}
 */
export const BrushType = Object.freeze({
    /**
     * 基础画笔
     */
    Basic: 0, "0": "Basic",
    /**
     * 铅笔风格
     */
    Pencil: 1, "1": "Pencil",
    /**
     * 马克笔风格
     */
    Marker: 2, "2": "Marker",
    /**
     * 水彩笔风格
     */
    Watercolor: 3, "3": "Watercolor",
    /**
     * 橡皮擦
     */
    Eraser: 4, "4": "Eraser",
});

/**
 * Color struct for representing RGBA colors.
 */
export class Color {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Color.prototype);
        obj.__wbg_ptr = ptr;
        ColorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ColorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_color_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    brightness() {
        const ret = wasm.color_brightness(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} a
     * @returns {Color}
     */
    static new(r, g, b, a) {
        const ret = wasm.color_new(r, g, b, a);
        return Color.__wrap(ret);
    }
    /**
     * @param {boolean} include_alpha
     * @returns {string}
     */
    to_hex(include_alpha) {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.color_to_hex(retptr, this.__wbg_ptr, include_alpha);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    get a() {
        const ret = wasm.__wbg_get_color_a(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get b() {
        const ret = wasm.__wbg_get_color_b(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get g() {
        const ret = wasm.__wbg_get_color_g(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get r() {
        const ret = wasm.__wbg_get_color_r(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set a(arg0) {
        wasm.__wbg_set_color_a(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set b(arg0) {
        wasm.__wbg_set_color_b(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set g(arg0) {
        wasm.__wbg_set_color_g(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set r(arg0) {
        wasm.__wbg_set_color_r(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Color.prototype[Symbol.dispose] = Color.prototype.free;

/**
 * 颜色空间枚举，用于明度调整
 * @enum {0 | 1 | 2 | 3}
 */
export const ColorSpace = Object.freeze({
    /**
     * HSL 颜色空间
     */
    Hsl: 0, "0": "Hsl",
    /**
     * LCH 颜色空间
     */
    Lch: 1, "1": "Lch",
    /**
     * HSV 颜色空间
     */
    Hsv: 2, "2": "Hsv",
    /**
     * HSLuv 颜色空间
     */
    Hsluv: 3, "3": "Hsluv",
});

/**
 * 图像处理器结构体
 * 这是公共 API，封装了所有图像处理功能
 */
export class ImageProcessor {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ImageProcessor.prototype);
        obj.__wbg_ptr = ptr;
        ImageProcessorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ImageProcessorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_imageprocessor_free(ptr, 0);
    }
    /**
     * 添加点到当前笔划
     * @param {number} x
     * @param {number} y
     * @param {number} pressure
     */
    add_stroke_point(x, y, pressure) {
        wasm.imageprocessor_add_stroke_point(this.__wbg_ptr, x, y, pressure);
    }
    /**
     * @param {number} level
     */
    adjust_lightness(level) {
        wasm.imageprocessor_adjust_lightness(this.__wbg_ptr, level);
    }
    /**
     * @param {number} level
     */
    adjust_saturation(level) {
        wasm.imageprocessor_adjust_saturation(this.__wbg_ptr, level);
    }
    /**
     * @param {number} amt
     */
    alter_blue_channel(amt) {
        wasm.imageprocessor_alter_blue_channel(this.__wbg_ptr, amt);
    }
    /**
     * @param {number} amt
     */
    alter_green_channel(amt) {
        wasm.imageprocessor_alter_green_channel(this.__wbg_ptr, amt);
    }
    /**
     * @param {number} amt
     */
    alter_red_channel(amt) {
        wasm.imageprocessor_alter_red_channel(this.__wbg_ptr, amt);
    }
    /**
     * @param {number} brightness
     * @param {number} contrast
     * @param {number} saturation
     * @param {number} hue
     * @param {number} lightness
     * @param {ColorSpace} lightness_color_space
     * @param {number} gamma_red
     * @param {number} gamma_green
     * @param {number} gamma_blue
     * @param {number} sharpen_strength
     * @param {number} noise_reduction_strength
     */
    apply_all_adjustments(brightness, contrast, saturation, hue, lightness, lightness_color_space, gamma_red, gamma_green, gamma_blue, sharpen_strength, noise_reduction_strength) {
        wasm.imageprocessor_apply_all_adjustments(this.__wbg_ptr, brightness, contrast, saturation, hue, lightness, lightness_color_space, gamma_red, gamma_green, gamma_blue, sharpen_strength, noise_reduction_strength);
    }
    apply_b_grayscale() {
        wasm.imageprocessor_apply_b_grayscale(this.__wbg_ptr);
    }
    /**
     * @param {number} sigma_spatial
     * @param {number} sigma_range
     * @param {boolean} fast_mode
     */
    apply_bilateral_filter(sigma_spatial, sigma_range, fast_mode) {
        wasm.imageprocessor_apply_bilateral_filter(this.__wbg_ptr, sigma_spatial, sigma_range, fast_mode);
    }
    apply_box_blur() {
        wasm.imageprocessor_apply_box_blur(this.__wbg_ptr);
    }
    /**
     * @param {number} level
     */
    apply_brightness(level) {
        wasm.imageprocessor_apply_brightness(this.__wbg_ptr, level);
    }
    /**
     * @param {number} center_x
     * @param {number} center_y
     * @param {number} radius
     * @param {number} feather_radius
     */
    apply_circular_mask(center_x, center_y, radius, feather_radius) {
        wasm.imageprocessor_apply_circular_mask(this.__wbg_ptr, center_x, center_y, radius, feather_radius);
    }
    /**
     * @param {number} r_factor
     * @param {number} g_factor
     * @param {number} b_factor
     * @param {number} strength
     */
    apply_color_noise_with_strength(r_factor, g_factor, b_factor, strength) {
        wasm.imageprocessor_apply_color_noise_with_strength(this.__wbg_ptr, r_factor, g_factor, b_factor, strength);
    }
    apply_colorize() {
        wasm.imageprocessor_apply_colorize(this.__wbg_ptr);
    }
    /**
     * @param {number} level
     */
    apply_contrast(level) {
        wasm.imageprocessor_apply_contrast(this.__wbg_ptr, level);
    }
    apply_decompose_max() {
        wasm.imageprocessor_apply_decompose_max(this.__wbg_ptr);
    }
    apply_decompose_min() {
        wasm.imageprocessor_apply_decompose_min(this.__wbg_ptr);
    }
    apply_desaturate() {
        wasm.imageprocessor_apply_desaturate(this.__wbg_ptr);
    }
    apply_detect_135_deg_lines() {
        wasm.imageprocessor_apply_detect_135_deg_lines(this.__wbg_ptr);
    }
    apply_detect_45_deg_lines() {
        wasm.imageprocessor_apply_detect_45_deg_lines(this.__wbg_ptr);
    }
    apply_detect_horizontal_lines() {
        wasm.imageprocessor_apply_detect_horizontal_lines(this.__wbg_ptr);
    }
    apply_detect_vertical_lines() {
        wasm.imageprocessor_apply_detect_vertical_lines(this.__wbg_ptr);
    }
    /**
     * @param {number} depth
     */
    apply_dither(depth) {
        wasm.imageprocessor_apply_dither(this.__wbg_ptr, depth);
    }
    /**
     * @param {number} r1
     * @param {number} g1
     * @param {number} b1
     * @param {number} r2
     * @param {number} g2
     * @param {number} b2
     */
    apply_duotone(r1, g1, b1, r2, g2, b2) {
        wasm.imageprocessor_apply_duotone(this.__wbg_ptr, r1, g1, b1, r2, g2, b2);
    }
    apply_edge_detection() {
        wasm.imageprocessor_apply_edge_detection(this.__wbg_ptr);
    }
    apply_edge_one() {
        wasm.imageprocessor_apply_edge_one(this.__wbg_ptr);
    }
    apply_emboss() {
        wasm.imageprocessor_apply_emboss(this.__wbg_ptr);
    }
    apply_frosted_glass() {
        wasm.imageprocessor_apply_frosted_glass(this.__wbg_ptr);
    }
    /**
     * @param {number} red
     * @param {number} green
     * @param {number} blue
     */
    apply_gamma(red, green, blue) {
        wasm.imageprocessor_apply_gamma(this.__wbg_ptr, red, green, blue);
    }
    /**
     * @param {number} radius
     */
    apply_gaussian_blur(radius) {
        wasm.imageprocessor_apply_gaussian_blur(this.__wbg_ptr, radius);
    }
    apply_gradient() {
        wasm.imageprocessor_apply_gradient(this.__wbg_ptr);
    }
    apply_grayscale() {
        wasm.imageprocessor_apply_grayscale(this.__wbg_ptr);
    }
    apply_grayscale_human_corrected() {
        wasm.imageprocessor_apply_grayscale_human_corrected(this.__wbg_ptr);
    }
    /**
     * @param {number} num_shades
     */
    apply_grayscale_shades(num_shades) {
        wasm.imageprocessor_apply_grayscale_shades(this.__wbg_ptr, num_shades);
    }
    apply_halftone() {
        wasm.imageprocessor_apply_halftone(this.__wbg_ptr);
    }
    /**
     * @param {number} level
     */
    apply_hue(level) {
        wasm.imageprocessor_apply_hue(this.__wbg_ptr, level);
    }
    apply_identity() {
        wasm.imageprocessor_apply_identity(this.__wbg_ptr);
    }
    /**
     * @param {number} brightness
     */
    apply_inc_brightness(brightness) {
        wasm.imageprocessor_apply_inc_brightness(this.__wbg_ptr, brightness);
    }
    apply_invert() {
        wasm.imageprocessor_apply_invert(this.__wbg_ptr);
    }
    apply_laplace() {
        wasm.imageprocessor_apply_laplace(this.__wbg_ptr);
    }
    /**
     * @param {number} level
     * @param {ColorSpace} color_space
     */
    apply_lightness(level, color_space) {
        wasm.imageprocessor_apply_lightness(this.__wbg_ptr, level, color_space);
    }
    apply_lix() {
        wasm.imageprocessor_apply_lix(this.__wbg_ptr);
    }
    apply_neue() {
        wasm.imageprocessor_apply_neue(this.__wbg_ptr);
    }
    /**
     * @param {number} strength
     */
    apply_noise(strength) {
        wasm.imageprocessor_apply_noise(this.__wbg_ptr, strength);
    }
    /**
     * @param {number} strength
     */
    apply_noise_reduction(strength) {
        wasm.imageprocessor_apply_noise_reduction(this.__wbg_ptr, strength);
    }
    apply_normalize() {
        wasm.imageprocessor_apply_normalize(this.__wbg_ptr);
    }
    /**
     * @param {number} radius
     * @param {number} intensity
     */
    apply_oil(radius, intensity) {
        wasm.imageprocessor_apply_oil(this.__wbg_ptr, radius, intensity);
    }
    apply_pink_noise() {
        wasm.imageprocessor_apply_pink_noise(this.__wbg_ptr);
    }
    /**
     * @param {number} pixel_size
     */
    apply_pixelate(pixel_size) {
        wasm.imageprocessor_apply_pixelate(this.__wbg_ptr, pixel_size);
    }
    /**
     * @param {Float32Array} vertices
     * @param {boolean} anti_aliased
     * @param {boolean} smooth_edges
     * @param {number} smoothing_radius
     */
    apply_polygon_mask(vertices, anti_aliased, smooth_edges, smoothing_radius) {
        const ptr0 = passArrayF32ToWasm0(vertices, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.imageprocessor_apply_polygon_mask(this.__wbg_ptr, ptr0, len0, anti_aliased, smooth_edges, smoothing_radius);
    }
    /**
     * @param {string} filter_name
     */
    apply_preset_filter(filter_name) {
        const ptr0 = passStringToWasm0(filter_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.imageprocessor_apply_preset_filter(this.__wbg_ptr, ptr0, len0);
    }
    apply_prewitt_horizontal() {
        wasm.imageprocessor_apply_prewitt_horizontal(this.__wbg_ptr);
    }
    apply_primary() {
        wasm.imageprocessor_apply_primary(this.__wbg_ptr);
    }
    apply_ryo() {
        wasm.imageprocessor_apply_ryo(this.__wbg_ptr);
    }
    /**
     * @param {number} level
     */
    apply_saturation(level) {
        wasm.imageprocessor_apply_saturation(this.__wbg_ptr, level);
    }
    apply_sepia() {
        wasm.imageprocessor_apply_sepia(this.__wbg_ptr);
    }
    /**
     * @param {number} strength
     */
    apply_sharpen(strength) {
        wasm.imageprocessor_apply_sharpen(this.__wbg_ptr, strength);
    }
    apply_sobel_global() {
        wasm.imageprocessor_apply_sobel_global(this.__wbg_ptr);
    }
    apply_sobel_horizontal() {
        wasm.imageprocessor_apply_sobel_horizontal(this.__wbg_ptr);
    }
    apply_sobel_vertical() {
        wasm.imageprocessor_apply_sobel_vertical(this.__wbg_ptr);
    }
    apply_solarize() {
        wasm.imageprocessor_apply_solarize(this.__wbg_ptr);
    }
    /**
     * @param {number} num_strips
     * @param {boolean} horizontal
     * @param {number | null} [color_r]
     * @param {number | null} [color_g]
     * @param {number | null} [color_b]
     */
    apply_strips(num_strips, horizontal, color_r, color_g, color_b) {
        wasm.imageprocessor_apply_strips(this.__wbg_ptr, num_strips, horizontal, isLikeNone(color_r) ? 0xFFFFFF : color_r, isLikeNone(color_g) ? 0xFFFFFF : color_g, isLikeNone(color_b) ? 0xFFFFFF : color_b);
    }
    /**
     * @param {number} threshold
     */
    apply_threshold(threshold) {
        wasm.imageprocessor_apply_threshold(this.__wbg_ptr, threshold);
    }
    /**
     * @param {number} r
     * @param {number} g
     * @param {number} b
     */
    apply_tint(r, g, b) {
        wasm.imageprocessor_apply_tint(this.__wbg_ptr, r, g, b);
    }
    /**
     * @param {Uint8Array} watermark_bytes
     * @param {bigint} x
     * @param {bigint} y
     * @param {number | null} [scale]
     * @param {number | null} [opacity]
     * @param {number | null} [rotation]
     */
    apply_watermark(watermark_bytes, x, y, scale, opacity, rotation) {
        const ptr0 = passArray8ToWasm0(watermark_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.imageprocessor_apply_watermark(this.__wbg_ptr, ptr0, len0, x, y, isLikeNone(scale) ? 0x100000001 : Math.fround(scale), isLikeNone(opacity) ? 0x100000001 : Math.fround(opacity), isLikeNone(rotation) ? 0x100000001 : Math.fround(rotation));
    }
    /**
     * @param {Uint8Array} watermark_bytes
     * @param {bigint} x
     * @param {bigint} y
     * @param {number} scale
     * @param {string} blend_mode
     */
    apply_watermark_with_blend(watermark_bytes, x, y, scale, blend_mode) {
        const ptr0 = passArray8ToWasm0(watermark_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(blend_mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        wasm.imageprocessor_apply_watermark_with_blend(this.__wbg_ptr, ptr0, len0, x, y, scale, ptr1, len1);
    }
    /**
     * @param {number} target_r
     * @param {number} target_g
     * @param {number} target_b
     * @param {number} tolerance
     * @param {number} feather_radius
     */
    auto_crop_by_color(target_r, target_g, target_b, tolerance, feather_radius) {
        wasm.imageprocessor_auto_crop_by_color(this.__wbg_ptr, target_r, target_g, target_b, tolerance, feather_radius);
    }
    /**
     * 开始一笔新画
     * @param {BrushConfig} config
     */
    begin_stroke(config) {
        _assertClass(config, BrushConfig);
        var ptr0 = config.__destroy_into_raw();
        wasm.imageprocessor_begin_stroke(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {Uint8Array} overlay_bytes
     * @param {string} blend_mode
     * @param {number | null} [scale]
     */
    blend_images(overlay_bytes, blend_mode, scale) {
        const ptr0 = passArray8ToWasm0(overlay_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(blend_mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        wasm.imageprocessor_blend_images(this.__wbg_ptr, ptr0, len0, ptr1, len1, isLikeNone(scale) ? 0x100000001 : Math.fround(scale));
    }
    /**
     * @param {Uint8Array} overlay_bytes
     * @param {number} scale
     * @param {string} blend_mode
     */
    blend_images_with_scale(overlay_bytes, scale, blend_mode) {
        const ptr0 = passArray8ToWasm0(overlay_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(blend_mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        wasm.imageprocessor_blend_images_with_scale(this.__wbg_ptr, ptr0, len0, scale, ptr1, len1);
    }
    /**
     * 清除所有笔划
     */
    clear_strokes() {
        wasm.imageprocessor_clear_strokes(this.__wbg_ptr);
    }
    /**
     * @param {number} x1
     * @param {number} y1
     * @param {number} x2
     * @param {number} y2
     */
    crop(x1, y1, x2, y2) {
        wasm.imageprocessor_crop(this.__wbg_ptr, x1, y1, x2, y2);
    }
    /**
     * @param {number} level
     */
    darken_hsl(level) {
        wasm.imageprocessor_darken_hsl(this.__wbg_ptr, level);
    }
    /**
     * @param {number} level
     */
    desaturate_hsl(level) {
        wasm.imageprocessor_desaturate_hsl(this.__wbg_ptr, level);
    }
    /**
     * 直接绘制一笔（简化接口，向后兼容）
     * @param {any} points_js
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} a
     * @param {number} width
     */
    draw_stroke(points_js, r, g, b, a, width) {
        wasm.imageprocessor_draw_stroke(this.__wbg_ptr, addHeapObject(points_js), r, g, b, a, width);
    }
    /**
     * 绘制一笔（高性能版本，使用 Float32Array）
     * @param {Float32Array} points_array
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} a
     * @param {number} width
     */
    draw_stroke_array(points_array, r, g, b, a, width) {
        wasm.imageprocessor_draw_stroke_array(this.__wbg_ptr, addHeapObject(points_array), r, g, b, a, width);
    }
    /**
     * @param {string} text
     * @param {number} x
     * @param {number} y
     * @param {number} font_size
     * @param {number | null} [font_type]
     * @param {boolean | null} [has_shadow]
     * @param {number | null} [color_r]
     * @param {number | null} [color_g]
     * @param {number | null} [color_b]
     */
    draw_text(text, x, y, font_size, font_type, has_shadow, color_r, color_g, color_b) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.imageprocessor_draw_text(this.__wbg_ptr, ptr0, len0, x, y, font_size, isLikeNone(font_type) ? 0xFFFFFF : font_type, isLikeNone(has_shadow) ? 0xFFFFFF : has_shadow ? 1 : 0, isLikeNone(color_r) ? 0xFFFFFF : color_r, isLikeNone(color_g) ? 0xFFFFFF : color_g, isLikeNone(color_b) ? 0xFFFFFF : color_b);
    }
    /**
     * @param {string} text
     * @param {number} x
     * @param {number} y
     * @param {number} font_size
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} font_type
     */
    draw_text_with_color_and_font(text, x, y, font_size, r, g, b, font_type) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.imageprocessor_draw_text_with_color_and_font(this.__wbg_ptr, ptr0, len0, x, y, font_size, r, g, b, font_type);
    }
    /**
     * @param {string} text
     * @param {number} x
     * @param {number} y
     * @param {number} font_size
     * @param {string} font_name
     * @param {boolean | null} [has_shadow]
     * @param {number | null} [color_r]
     * @param {number | null} [color_g]
     * @param {number | null} [color_b]
     */
    draw_text_with_font_name(text, x, y, font_size, font_name, has_shadow, color_r, color_g, color_b) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(font_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        wasm.imageprocessor_draw_text_with_font_name(this.__wbg_ptr, ptr0, len0, x, y, font_size, ptr1, len1, isLikeNone(has_shadow) ? 0xFFFFFF : has_shadow ? 1 : 0, isLikeNone(color_r) ? 0xFFFFFF : color_r, isLikeNone(color_g) ? 0xFFFFFF : color_g, isLikeNone(color_b) ? 0xFFFFFF : color_b);
    }
    /**
     * @param {string} text
     * @param {number} x
     * @param {number} y
     * @param {number} font_size
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} font_type
     */
    draw_text_with_shadow_and_color_and_font(text, x, y, font_size, r, g, b, font_type) {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.imageprocessor_draw_text_with_shadow_and_color_and_font(this.__wbg_ptr, ptr0, len0, x, y, font_size, r, g, b, font_type);
    }
    /**
     * 结束当前笔划并渲染
     */
    end_stroke() {
        wasm.imageprocessor_end_stroke(this.__wbg_ptr);
    }
    flip_horizontal() {
        wasm.imageprocessor_flip_horizontal(this.__wbg_ptr);
    }
    flip_vertical() {
        wasm.imageprocessor_flip_vertical(this.__wbg_ptr);
    }
    /**
     * @returns {Uint8Array}
     */
    get_bytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.imageprocessor_get_bytes(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * 获取图像的调色板
     *
     * # 参数
     * * `num_colors` - 要提取的颜色数量
     *
     * # 返回值
     * 返回包含颜色数组的 JsValue (Array<Uint8Array>)，每个颜色是 [r, g, b, a] 格式
     * @param {number} num_colors
     * @returns {any}
     */
    get_color_palette(num_colors) {
        const ret = wasm.imageprocessor_get_color_palette(this.__wbg_ptr, num_colors);
        return takeObject(ret);
    }
    /**
     * 获取整个图像的主色调
     *
     * # 返回值
     * 返回包含 RGBA 值的 JsValue (Uint8Array)
     * @returns {any}
     */
    get_dominant_color() {
        const ret = wasm.imageprocessor_get_dominant_color(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @returns {bigint}
     */
    get_estimated_filesize() {
        const ret = wasm.imageprocessor_get_estimated_filesize(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @returns {number}
     */
    get_height() {
        const ret = wasm.imageprocessor_get_height(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * 获取指定坐标的像素亮度
     *
     * # 参数
     * * `x` - X 坐标 (0 到 width-1)
     * * `y` - Y 坐标 (0 到 height-1)
     *
     * # 返回值
     * 返回亮度值 (0-255)，如果坐标超出范围则返回 null
     * @param {number} x
     * @param {number} y
     * @returns {number | undefined}
     */
    get_pixel_brightness(x, y) {
        const ret = wasm.imageprocessor_get_pixel_brightness(this.__wbg_ptr, x, y);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * 获取指定坐标的像素颜色
     *
     * # 参数
     * * `x` - X 坐标 (0 到 width-1)
     * * `y` - Y 坐标 (0 到 height-1)
     *
     * # 返回值
     * 返回包含 RGBA 值的 JsValue (Uint8Array)，如果坐标超出范围则返回 null
     * @param {number} x
     * @param {number} y
     * @returns {any}
     */
    get_pixel_color(x, y) {
        const ret = wasm.imageprocessor_get_pixel_color(this.__wbg_ptr, x, y);
        return takeObject(ret);
    }
    /**
     * 获取指定坐标的像素颜色的十六进制表示
     *
     * # 参数
     * * `x` - X 坐标 (0 到 width-1)
     * * `y` - Y 坐标 (0 到 height-1)
     * * `include_alpha` - 是否包含 alpha 通道
     *
     * # 返回值
     * 返回十六进制颜色字符串，如果坐标超出范围则返回 null
     * @param {number} x
     * @param {number} y
     * @param {boolean} include_alpha
     * @returns {string | undefined}
     */
    get_pixel_color_hex(x, y, include_alpha) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.imageprocessor_get_pixel_color_hex(retptr, this.__wbg_ptr, x, y, include_alpha);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export4(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {Uint8Array}
     */
    get_raw_pixels() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.imageprocessor_get_raw_pixels(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * 获取指定区域的平均亮度
     *
     * # 参数
     * * `x` - 区域左上角 X 坐标
     * * `y` - 区域左上角 Y 坐标
     * * `width` - 区域宽度
     * * `height` - 区域高度
     *
     * # 返回值
     * 返回平均亮度值 (0-255)，如果区域超出范围则返回 null
     * @param {number} x
     * @param {number} y
     * @param {number} width
     * @param {number} height
     * @returns {number | undefined}
     */
    get_region_average_brightness(x, y, width, height) {
        const ret = wasm.imageprocessor_get_region_average_brightness(this.__wbg_ptr, x, y, width, height);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * 获取指定区域的平均颜色
     *
     * # 参数
     * * `x` - 区域左上角 X 坐标
     * * `y` - 区域左上角 Y 坐标
     * * `width` - 区域宽度
     * * `height` - 区域高度
     *
     * # 返回值
     * 返回包含 RGBA 平均值的 JsValue (Uint8Array)，如果区域超出范围则返回 null
     * @param {number} x
     * @param {number} y
     * @param {number} width
     * @param {number} height
     * @returns {any}
     */
    get_region_average_color(x, y, width, height) {
        const ret = wasm.imageprocessor_get_region_average_color(this.__wbg_ptr, x, y, width, height);
        return takeObject(ret);
    }
    /**
     * 获取指定区域的主色调
     *
     * # 参数
     * * `x` - 区域左上角 X 坐标
     * * `y` - 区域左上角 Y 坐标
     * * `width` - 区域宽度
     * * `height` - 区域高度
     *
     * # 返回值
     * 返回包含 RGBA 值的 JsValue (Uint8Array)，如果区域超出范围则返回 null
     * @param {number} x
     * @param {number} y
     * @param {number} width
     * @param {number} height
     * @returns {any}
     */
    get_region_dominant_color(x, y, width, height) {
        const ret = wasm.imageprocessor_get_region_dominant_color(this.__wbg_ptr, x, y, width, height);
        return takeObject(ret);
    }
    /**
     * 获取历史笔划数量
     * @returns {number}
     */
    get_stroke_count() {
        const ret = wasm.imageprocessor_get_stroke_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get_width() {
        const ret = wasm.imageprocessor_get_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} degrees
     * @param {number} color_space
     */
    hue_rotate(degrees, color_space) {
        wasm.imageprocessor_hue_rotate(this.__wbg_ptr, degrees, color_space);
    }
    /**
     * @param {number} degrees
     */
    hue_rotate_hsl(degrees) {
        wasm.imageprocessor_hue_rotate_hsl(this.__wbg_ptr, degrees);
    }
    /**
     * @param {number} level
     */
    lighten_hsl(level) {
        wasm.imageprocessor_lighten_hsl(this.__wbg_ptr, level);
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {Uint8Array} data
     */
    constructor(width, height, data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.imageprocessor_new(retptr, width, height, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            ImageProcessorFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {Uint8Array} bytes
     * @returns {ImageProcessor}
     */
    static new_from_bytes(bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.imageprocessor_new_from_bytes(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return ImageProcessor.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * 创建指定大小的白色画布
     * @param {number} width
     * @param {number} height
     * @returns {ImageProcessor}
     */
    static new_white_canvas(width, height) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.imageprocessor_new_white_canvas(retptr, width, height);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return ImageProcessor.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} offset_amt
     */
    offset_blue(offset_amt) {
        wasm.imageprocessor_offset_blue(this.__wbg_ptr, offset_amt);
    }
    /**
     * @param {number} offset_amt
     */
    offset_green(offset_amt) {
        wasm.imageprocessor_offset_green(this.__wbg_ptr, offset_amt);
    }
    /**
     * @param {number} offset_amt
     */
    offset_red(offset_amt) {
        wasm.imageprocessor_offset_red(this.__wbg_ptr, offset_amt);
    }
    /**
     * @param {number} smoothing_radius
     */
    refine_edges(smoothing_radius) {
        wasm.imageprocessor_refine_edges(this.__wbg_ptr, smoothing_radius);
    }
    remove_blue_channel() {
        wasm.imageprocessor_remove_blue_channel(this.__wbg_ptr);
    }
    remove_green_channel() {
        wasm.imageprocessor_remove_green_channel(this.__wbg_ptr);
    }
    remove_red_channel() {
        wasm.imageprocessor_remove_red_channel(this.__wbg_ptr);
    }
    reset() {
        wasm.imageprocessor_reset(this.__wbg_ptr);
    }
    /**
     * @param {number} new_width
     * @param {number} new_height
     */
    resize(new_width, new_height) {
        wasm.imageprocessor_resize(this.__wbg_ptr, new_width, new_height);
    }
    rotate_90() {
        wasm.imageprocessor_rotate_90(this.__wbg_ptr);
    }
    /**
     * @param {number} angle
     */
    rotate_any(angle) {
        wasm.imageprocessor_rotate_any(this.__wbg_ptr, angle);
    }
    /**
     * @param {number} level
     */
    saturate_hsl(level) {
        wasm.imageprocessor_saturate_hsl(this.__wbg_ptr, level);
    }
    /**
     * @param {number} threshold
     * @param {number} feather_radius
     */
    smart_crop(threshold, feather_radius) {
        wasm.imageprocessor_smart_crop(this.__wbg_ptr, threshold, feather_radius);
    }
    swap_gb_channels() {
        wasm.imageprocessor_swap_gb_channels(this.__wbg_ptr);
    }
    swap_rb_channels() {
        wasm.imageprocessor_swap_rb_channels(this.__wbg_ptr);
    }
    swap_rg_channels() {
        wasm.imageprocessor_swap_rg_channels(this.__wbg_ptr);
    }
    /**
     * @returns {string}
     */
    to_base64() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.imageprocessor_to_base64(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} quality
     * @returns {string}
     */
    to_jpeg(quality) {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.imageprocessor_to_jpeg(retptr, this.__wbg_ptr, quality);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    to_png() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.imageprocessor_to_png(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} quality
     * @returns {string}
     */
    to_webp(quality) {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.imageprocessor_to_webp(retptr, this.__wbg_ptr, quality);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * 撤销最后一笔
     * @returns {boolean}
     */
    undo_stroke() {
        const ret = wasm.imageprocessor_undo_stroke(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) ImageProcessor.prototype[Symbol.dispose] = ImageProcessor.prototype.free;

/**
 * Provides the image's height, width, and contains the image's raw pixels.
 * For use when communicating between JS and WASM, and also natively.
 */
export class PhotonImage {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PhotonImage.prototype);
        obj.__wbg_ptr = ptr;
        PhotonImageFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PhotonImageFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_photonimage_free(ptr, 0);
    }
    /**
     * Convert the PhotonImage to base64.
     * @returns {string}
     */
    get_base64() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_base64(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Convert the PhotonImage to raw bytes. Returns PNG.
     * @returns {Uint8Array}
     */
    get_bytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_bytes(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Convert the PhotonImage to raw bytes. Returns a JPEG.
     * @param {number} quality
     * @returns {Uint8Array}
     */
    get_bytes_jpeg(quality) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_bytes_jpeg(retptr, this.__wbg_ptr, quality);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Convert the PhotonImage to raw bytes. Returns a WEBP.
     * @returns {Uint8Array}
     */
    get_bytes_webp() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_bytes_webp(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Convert the PhotonImage to raw bytes. Returns a WEBP with specified quality.
     * # Arguments
     * * `quality` - WebP quality (0-100). Higher means better quality but larger file.
     *   - 0-50: Low quality, small file size
     *   - 51-75: Medium quality (recommended for web)
     *   - 76-100: High quality, larger file size
     *
     * Note: The image 0.24.x crate's WebPEncoder currently only supports lossless encoding.
     * The quality parameter is reserved for future use when the crate adds lossy encoding support.
     * Currently, all images are encoded in lossless mode regardless of the quality value.
     * For quality control, consider using JPEG format with `get_bytes_jpeg()` instead.
     * @param {number} _quality
     * @returns {Uint8Array}
     */
    get_bytes_webp_with_quality(_quality) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_bytes_webp_with_quality(retptr, this.__wbg_ptr, _quality);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the color palette of the image.
     *
     * Returns a list of the most frequent colors in the image.
     *
     * # Arguments
     * * `num_colors` - Number of colors to extract (default 5)
     *
     * # Example
     *
     * ```no_run
     * use photon_rs::PhotonImage;
     *
     * let img = PhotonImage::new(vec![255, 0, 0, 255, 0, 255, 0, 255], 2, 1);
     * let palette = img.get_color_palette(5);
     * for (i, color) in palette.iter().enumerate() {
     *     println!("Color {}: #{:02x}{:02x}{:02x}", i + 1, color.r, color.g, color.b);
     * }
     * ```
     * @param {number} num_colors
     * @returns {Color[]}
     */
    get_color_palette(num_colors) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_color_palette(retptr, this.__wbg_ptr, num_colors);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the dominant color of the entire image.
     *
     * Uses a color quantization approach to find the most frequent color.
     * Returns the RGBA color values as a Color struct.
     *
     * # Example
     *
     * ```no_run
     * use photon_rs::PhotonImage;
     *
     * let img = PhotonImage::new(vec![255, 0, 0, 255], 1, 1);
     * let color = img.get_dominant_color();
     * println!("Dominant color: R={}, G={}, B={}, A={}", color.r, color.g, color.b, color.a);
     * ```
     * @returns {Color}
     */
    get_dominant_color() {
        const ret = wasm.photonimage_get_dominant_color(this.__wbg_ptr);
        return Color.__wrap(ret);
    }
    /**
     * Calculates estimated filesize and returns number of bytes
     * @returns {bigint}
     */
    get_estimated_filesize() {
        const ret = wasm.photonimage_get_estimated_filesize(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Get the height of the PhotonImage.
     * @returns {number}
     */
    get_height() {
        const ret = wasm.photonimage_get_height(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Convert the PhotonImage's raw pixels to JS-compatible ImageData.
     * @returns {ImageData}
     */
    get_image_data() {
        const ret = wasm.photonimage_get_image_data(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * Get the brightness of a pixel at the specified coordinates.
     *
     * Returns the brightness value (0-255) using the human-corrected formula.
     * Returns None if the coordinates are out of bounds.
     *
     * # Arguments
     * * `x` - X coordinate (0 to width-1)
     * * `y` - Y coordinate (0 to height-1)
     *
     * # Example
     *
     * ```no_run
     * use photon_rs::PhotonImage;
     *
     * let img = PhotonImage::new(vec![255, 0, 0, 255], 1, 1);
     * if let Some(brightness) = img.get_pixel_brightness(0, 0) {
     *     println!("Pixel brightness: {}", brightness);
     * }
     * ```
     * @param {number} x
     * @param {number} y
     * @returns {number | undefined}
     */
    get_pixel_brightness(x, y) {
        const ret = wasm.photonimage_get_pixel_brightness(this.__wbg_ptr, x, y);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * Get the color of a pixel at the specified coordinates.
     *
     * Returns the RGBA color values as a Color struct.
     * Returns None if the coordinates are out of bounds.
     *
     * # Arguments
     * * `x` - X coordinate (0 to width-1)
     * * `y` - Y coordinate (0 to height-1)
     *
     * # Example
     *
     * ```no_run
     * use photon_rs::PhotonImage;
     *
     * let img = PhotonImage::new(vec![255, 0, 0, 255], 1, 1);
     * if let Some(color) = img.get_pixel_color(0, 0) {
     *     println!("Pixel color: R={}, G={}, B={}, A={}", color.r, color.g, color.b, color.a);
     * }
     * ```
     * @param {number} x
     * @param {number} y
     * @returns {Color | undefined}
     */
    get_pixel_color(x, y) {
        const ret = wasm.photonimage_get_pixel_color(this.__wbg_ptr, x, y);
        return ret === 0 ? undefined : Color.__wrap(ret);
    }
    /**
     * Get the color of a pixel at the specified coordinates as a hex string.
     *
     * Returns the color in hex format (#RRGGBB or #RRGGBBAA).
     * Returns None if the coordinates are out of bounds.
     *
     * # Arguments
     * * `x` - X coordinate (0 to width-1)
     * * `y` - Y coordinate (0 to height-1)
     * * `include_alpha` - Whether to include alpha channel in the hex string
     *
     * # Example
     *
     * ```no_run
     * use photon_rs::PhotonImage;
     *
     * let img = PhotonImage::new(vec![255, 0, 0, 255], 1, 1);
     * if let Some(hex) = img.get_pixel_color_hex(0, 0, false) {
     *     println!("Pixel color: {}", hex); // Output: #ff0000
     * }
     * ```
     * @param {number} x
     * @param {number} y
     * @param {boolean} include_alpha
     * @returns {string | undefined}
     */
    get_pixel_color_hex(x, y, include_alpha) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_pixel_color_hex(retptr, this.__wbg_ptr, x, y, include_alpha);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export4(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the PhotonImage's pixels as a Vec of u8s.
     *
     * **Note**: This clones the pixel data, which can be expensive for large images.
     * For read-only access, prefer `get_raw_pixels_slice()` which returns a reference without cloning.
     * @returns {Uint8Array}
     */
    get_raw_pixels() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_raw_pixels(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the average brightness of a rectangular region.
     *
     * Returns the average brightness value (0-255).
     * Returns None if the region is out of bounds.
     *
     * # Arguments
     * * `x` - X coordinate of the top-left corner
     * * `y` - Y coordinate of the top-left corner
     * * `width` - Width of the region
     * * `height` - Height of the region
     *
     * # Example
     *
     * ```no_run
     * use photon_rs::PhotonImage;
     *
     * let img = PhotonImage::new(vec![255, 0, 0, 255, 0, 255, 0, 255], 2, 1);
     * if let Some(brightness) = img.get_region_average_brightness(0, 0, 2, 1) {
     *     println!("Average brightness: {}", brightness);
     * }
     * ```
     * @param {number} x
     * @param {number} y
     * @param {number} width
     * @param {number} height
     * @returns {number | undefined}
     */
    get_region_average_brightness(x, y, width, height) {
        const ret = wasm.photonimage_get_region_average_brightness(this.__wbg_ptr, x, y, width, height);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * Get the average color of a rectangular region.
     *
     * Returns the average RGBA color values as a Color struct.
     * Returns None if the region is out of bounds.
     *
     * # Arguments
     * * `x` - X coordinate of the top-left corner
     * * `y` - Y coordinate of the top-left corner
     * * `width` - Width of the region
     * * `height` - Height of the region
     *
     * # Example
     *
     * ```no_run
     * use photon_rs::PhotonImage;
     *
     * let img = PhotonImage::new(vec![255, 0, 0, 255, 0, 255, 0, 255], 2, 1);
     * if let Some(color) = img.get_region_average_color(0, 0, 2, 1) {
     *     println!("Average color: R={}, G={}, B={}, A={}", color.r, color.g, color.b, color.a);
     * }
     * ```
     * @param {number} x
     * @param {number} y
     * @param {number} width
     * @param {number} height
     * @returns {Color | undefined}
     */
    get_region_average_color(x, y, width, height) {
        const ret = wasm.photonimage_get_region_average_color(this.__wbg_ptr, x, y, width, height);
        return ret === 0 ? undefined : Color.__wrap(ret);
    }
    /**
     * Get the dominant color of a rectangular region.
     *
     * Uses a color quantization approach to find the most frequent color in the region.
     * Returns None if the region is out of bounds.
     *
     * # Arguments
     * * `x` - X coordinate of the top-left corner
     * * `y` - Y coordinate of the top-left corner
     * * `width` - Width of the region
     * * `height` - Height of the region
     *
     * # Example
     *
     * ```no_run
     * use photon_rs::PhotonImage;
     *
     * let img = PhotonImage::new(vec![255, 0, 0, 255, 0, 255, 0, 255], 2, 1);
     * if let Some(color) = img.get_region_dominant_color(0, 0, 2, 1) {
     *     println!("Dominant color: R={}, G={}, B={}, A={}", color.r, color.g, color.b, color.a);
     * }
     * ```
     * @param {number} x
     * @param {number} y
     * @param {number} width
     * @param {number} height
     * @returns {Color | undefined}
     */
    get_region_dominant_color(x, y, width, height) {
        const ret = wasm.photonimage_get_region_dominant_color(this.__wbg_ptr, x, y, width, height);
        return ret === 0 ? undefined : Color.__wrap(ret);
    }
    /**
     * Get the width of the PhotonImage.
     * @returns {number}
     */
    get_width() {
        const ret = wasm.photonimage_get_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Initialize the thread pool for WebAssembly parallel processing.
     *
     * This function must be called from JavaScript before using any parallel processing features.
     * It sets up the worker threads for rayon parallel execution.
     *
     * # JavaScript Example
     * ```javascript
     * import { initThreadPool } from './photon_wasm.js';
     *
     * // Initialize with 4 threads
     * await initThreadPool(4);
     *
     * // Now you can use parallel processing features
     * ```
     *
     * # Arguments
     * * `num_threads` - Number of threads to use for parallel processing.
     *   If 0, it will use the hardware concurrency (number of logical CPUs).
     * @param {number} num_threads
     * @returns {Promise<void>}
     */
    static init_thread_pool(num_threads) {
        const ret = wasm.photonimage_init_thread_pool(num_threads);
        return takeObject(ret);
    }
    /**
     * Create a new PhotonImage from a Vec of u8s, which represent raw pixels.
     * @param {Uint8Array} raw_pixels
     * @param {number} width
     * @param {number} height
     */
    constructor(raw_pixels, width, height) {
        const ptr0 = passArray8ToWasm0(raw_pixels, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.photonimage_new(ptr0, len0, width, height);
        this.__wbg_ptr = ret >>> 0;
        PhotonImageFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Create a new PhotonImage from a base64 string.
     * @param {string} base64
     * @returns {PhotonImage}
     */
    static new_from_base64(base64) {
        const ptr0 = passStringToWasm0(base64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.photonimage_new_from_base64(ptr0, len0);
        return PhotonImage.__wrap(ret);
    }
    /**
     * Create a new PhotonImage from a Blob/File.
     * @param {Blob} blob
     * @returns {PhotonImage}
     */
    static new_from_blob(blob) {
        const ret = wasm.photonimage_new_from_blob(addHeapObject(blob));
        return PhotonImage.__wrap(ret);
    }
    /**
     * Create a new PhotonImage from a byteslice.
     * @param {Uint8Array} vec
     * @returns {PhotonImage}
     */
    static new_from_byteslice(vec) {
        const ptr0 = passArray8ToWasm0(vec, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.photonimage_new_from_byteslice(ptr0, len0);
        return PhotonImage.__wrap(ret);
    }
    /**
     * Create a new PhotonImage from a HTMLImageElement
     * @param {HTMLImageElement} image
     * @returns {PhotonImage}
     */
    static new_from_image(image) {
        const ret = wasm.photonimage_new_from_image(addHeapObject(image));
        return PhotonImage.__wrap(ret);
    }
    /**
     * Convert ImageData to raw pixels, and update the PhotonImage's raw pixels to this.
     * @param {ImageData} img_data
     */
    set_imgdata(img_data) {
        wasm.photonimage_set_imgdata(this.__wbg_ptr, addHeapObject(img_data));
    }
}
if (Symbol.dispose) PhotonImage.prototype[Symbol.dispose] = PhotonImage.prototype.free;

/**
 * RGB color type.
 */
export class Rgb {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RgbFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgb_free(ptr, 0);
    }
    /**
     * Get the Blue value.
     * @returns {number}
     */
    get_blue() {
        const ret = wasm.rgb_get_blue(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Green value.
     * @returns {number}
     */
    get_green() {
        const ret = wasm.rgb_get_green(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Red value.
     * @returns {number}
     */
    get_red() {
        const ret = wasm.rgb_get_red(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create a new RGB struct.
     * @param {number} r
     * @param {number} g
     * @param {number} b
     */
    constructor(r, g, b) {
        const ret = wasm.rgb_new(r, g, b);
        this.__wbg_ptr = ret >>> 0;
        RgbFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Set the Blue value.
     * @param {number} b
     */
    set_blue(b) {
        wasm.rgb_set_blue(this.__wbg_ptr, b);
    }
    /**
     * Get the Green value.
     * @param {number} g
     */
    set_green(g) {
        wasm.rgb_set_green(this.__wbg_ptr, g);
    }
    /**
     * Set the Red value.
     * @param {number} r
     */
    set_red(r) {
        wasm.rgb_set_red(this.__wbg_ptr, r);
    }
}
if (Symbol.dispose) Rgb.prototype[Symbol.dispose] = Rgb.prototype.free;

/**
 * RGBA color type.
 */
export class Rgba {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RgbaFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgba_free(ptr, 0);
    }
    /**
     * Get the alpha value for this color.
     * @returns {number}
     */
    get_alpha() {
        const ret = wasm.rgba_get_alpha(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Blue value.
     * @returns {number}
     */
    get_blue() {
        const ret = wasm.rgba_get_blue(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Green value.
     * @returns {number}
     */
    get_green() {
        const ret = wasm.rgba_get_green(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Red value.
     * @returns {number}
     */
    get_red() {
        const ret = wasm.rgba_get_red(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create a new RGBA struct.
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} a
     */
    constructor(r, g, b, a) {
        const ret = wasm.rgba_new(r, g, b, a);
        this.__wbg_ptr = ret >>> 0;
        RgbaFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Set the alpha value.
     * @param {number} a
     */
    set_alpha(a) {
        wasm.rgba_set_alpha(this.__wbg_ptr, a);
    }
    /**
     * Set the Blue value.
     * @param {number} b
     */
    set_blue(b) {
        wasm.rgba_set_blue(this.__wbg_ptr, b);
    }
    /**
     * Get the Green value.
     * @param {number} g
     */
    set_green(g) {
        wasm.rgba_set_green(this.__wbg_ptr, g);
    }
    /**
     * Set the Red value.
     * @param {number} r
     */
    set_red(r) {
        wasm.rgba_set_red(this.__wbg_ptr, r);
    }
}
if (Symbol.dispose) Rgba.prototype[Symbol.dispose] = Rgba.prototype.free;

/**
 * @enum {1 | 2 | 3 | 4 | 5}
 */
export const SamplingFilter = Object.freeze({
    Nearest: 1, "1": "Nearest",
    Triangle: 2, "2": "Triangle",
    CatmullRom: 3, "3": "CatmullRom",
    Gaussian: 4, "4": "Gaussian",
    Lanczos3: 5, "5": "Lanczos3",
});

/**
 * 笔划点
 */
export class StrokePoint {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StrokePointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_strokepoint_free(ptr, 0);
    }
    /**
     * 压感（0.0 - 1.0）
     * @returns {number}
     */
    get pressure() {
        const ret = wasm.__wbg_get_strokepoint_pressure(this.__wbg_ptr);
        return ret;
    }
    /**
     * 时间戳（毫秒）
     * @returns {bigint}
     */
    get timestamp() {
        const ret = wasm.__wbg_get_strokepoint_timestamp(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * X 坐标
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_strokepoint_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * Y 坐标
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_strokepoint_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * 压感（0.0 - 1.0）
     * @param {number} arg0
     */
    set pressure(arg0) {
        wasm.__wbg_set_strokepoint_pressure(this.__wbg_ptr, arg0);
    }
    /**
     * 时间戳（毫秒）
     * @param {bigint} arg0
     */
    set timestamp(arg0) {
        wasm.__wbg_set_strokepoint_timestamp(this.__wbg_ptr, arg0);
    }
    /**
     * X 坐标
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_strokepoint_x(this.__wbg_ptr, arg0);
    }
    /**
     * Y 坐标
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_strokepoint_y(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} pressure
     * @param {bigint} timestamp
     */
    constructor(x, y, pressure, timestamp) {
        const ret = wasm.strokepoint_new(x, y, pressure, timestamp);
        this.__wbg_ptr = ret >>> 0;
        StrokePointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) StrokePoint.prototype[Symbol.dispose] = StrokePoint.prototype.free;

/**
 * Add randomized noise to an image.
 * This function adds a Gaussian Noise Sample to each pixel through incrementing each channel by a randomized offset.
 * This randomized offset is generated by creating a randomized thread pool.
 * **[WASM SUPPORT IS AVAILABLE]**: Randomized thread pools cannot be created with WASM, but
 * a workaround using js_sys::Math::random works now.
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example:
 * use photon_rs::native::open_image;
 * use photon_rs::noise::add_noise_rand;
 * use photon_rs::PhotonImage;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * add_noise_rand(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function add_noise_rand(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.add_noise_rand(photon_image.__wbg_ptr);
}

/**
 * Add random noise to an image using parallel processing.
 *
 * This is the parallel version of the noise addition operation.
 * Each thread uses its own random number generator to avoid contention.
 *
 * # Arguments
 * * `photon_image` - A mutable reference to a PhotonImage.
 * * `strength` - Noise strength. Range: 0.0 to 10.0.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::parallel::add_noise_rand_parallel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * add_noise_rand_parallel(&mut img, 2.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} strength
 */
export function add_noise_rand_parallel(photon_image, strength) {
    _assertClass(photon_image, PhotonImage);
    wasm.add_noise_rand_parallel(photon_image.__wbg_ptr, strength);
}

/**
 * Add randomized noise to an image with adjustable strength.
 * This function adds Gaussian noise to each pixel by incrementing each channel by a randomized offset.
 * The maximum offset is controlled by the strength parameter.
 * **[WASM SUPPORT IS AVAILABLE]**
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `strength` - Noise strength. Range: 0.0 to 10.0.
 *   - 0.0: No noise
 *   - 1.0: Subtle noise (max offset 0-15)
 *   - 5.0: Moderate noise (max offset 0-75)
 *   - 10.0: Strong noise (max offset 0-150, equivalent to add_noise_rand)
 *
 * # Example
 *
 * ```no_run
 * // For example, to add noise with strength 2.0:
 * use photon_rs::native::open_image;
 * use photon_rs::noise::add_noise_rand_with_strength;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * add_noise_rand_with_strength(&mut img, 2.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} strength
 */
export function add_noise_rand_with_strength(photon_image, strength) {
    _assertClass(photon_image, PhotonImage);
    wasm.add_noise_rand_with_strength(photon_image.__wbg_ptr, strength);
}

/**
 * Adjust the brightness of an image by a factor.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `brightness` - A u8 to add or subtract to the brightness. To increase
 * the brightness, pass a positive number (up to 255). To decrease the brightness,
 * pass a negative number instead.
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::adjust_brightness;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * adjust_brightness(&mut img, 10_i16);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} brightness
 */
export function adjust_brightness(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.adjust_brightness(photon_image.__wbg_ptr, brightness);
}

/**
 * Apply brightness adjustment using parallel processing.
 *
 * This is the parallel version of the brightness adjustment operation.
 *
 * # Arguments
 * * `photon_image` - A mutable reference to a PhotonImage.
 * * `brightness` - The amount to adjust brightness by (-255 to 255).
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::parallel::adjust_brightness_parallel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * adjust_brightness_parallel(&mut img, 20);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} brightness
 */
export function adjust_brightness_parallel(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.adjust_brightness_parallel(photon_image.__wbg_ptr, brightness);
}

/**
 * Adjust the contrast of an image by a factor.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `contrast` - An f32 factor used to adjust contrast. Between [-255.0, 255.0]. The algorithm will
 * clamp results if passed factor is out of range.
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::adjust_contrast;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * adjust_contrast(&mut img, 30_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} contrast
 */
export function adjust_contrast(photon_image, contrast) {
    _assertClass(photon_image, PhotonImage);
    wasm.adjust_contrast(photon_image.__wbg_ptr, contrast);
}

/**
 * Apply contrast adjustment using parallel processing.
 *
 * This is the parallel version of the contrast adjustment operation.
 *
 * # Arguments
 * * `photon_image` - A mutable reference to a PhotonImage.
 * * `contrast` - Contrast factor between [-255.0, 255.0].
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::parallel::adjust_contrast_parallel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * adjust_contrast_parallel(&mut img, 30.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} contrast
 */
export function adjust_contrast_parallel(photon_image, contrast) {
    _assertClass(photon_image, PhotonImage);
    wasm.adjust_contrast_parallel(photon_image.__wbg_ptr, contrast);
}

/**
 * Increment or decrement every pixel's Blue channel by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `amt` - The amount to increment or decrement the channel's value by for that pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the Blue channel for all pixels by 10:
 * use photon_rs::channels::alter_blue_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_blue_channel(&mut img, 10_i16);
 * ```
 * @param {PhotonImage} img
 * @param {number} amt
 */
export function alter_blue_channel(img, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_blue_channel(img.__wbg_ptr, amt);
}

/**
 * Alter a select channel by incrementing or decrementing its value by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `channel` - The channel you wish to alter, it should be either 0, 1 or 2,
 * representing R, G, or B respectively. (O=Red, 1=Green, 2=Blue)
 * * `amount` - The amount to increment/decrement the channel's value by for that pixel.
 * A positive value will increment/decrement the channel's value, a negative value will decrement the channel's value.
 *
 * ## Example
 *
 * ```no_run
 * // For example, to increase the Red channel for all pixels by 10:
 * use photon_rs::channels::alter_channel;
 * use photon_rs::native::{open_image};
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_channel(&mut img, 0_usize, 10_i16);
 * ```
 *
 * Adds a constant to a select R, G, or B channel's value.
 *
 * ### Decrease a channel's value
 * // For example, to decrease the Green channel for all pixels by 20:
 * ```no_run
 * use photon_rs::channels::alter_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_channel(&mut img, 1_usize, -20_i16);
 * ```
 * **Note**: Note the use of a minus symbol when decreasing the channel.
 * @param {PhotonImage} img
 * @param {number} channel
 * @param {number} amt
 */
export function alter_channel(img, channel, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_channel(img.__wbg_ptr, channel, amt);
}

/**
 * Increment all 3 channels' values by adding an amt to each channel per pixel.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `r_amt` - The amount to increment/decrement the Red channel by.
 * * `g_amt` - The amount to increment/decrement the Green channel by.
 * * `b_amt` - The amount to increment/decrement the Blue channel by.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the values of the Red channel by 10, the Green channel by 20,
 * // and the Blue channel by 50:
 * use photon_rs::channels::alter_channels;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_channels(&mut img, 10_i16, 20_i16, 50_i16);
 * ```
 * @param {PhotonImage} img
 * @param {number} r_amt
 * @param {number} g_amt
 * @param {number} b_amt
 */
export function alter_channels(img, r_amt, g_amt, b_amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_channels(img.__wbg_ptr, r_amt, g_amt, b_amt);
}

/**
 * Increment or decrement every pixel's Green channel by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `amt` - The amount to increment/decrement the channel's value by for that pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the Green channel for all pixels by 20:
 * use photon_rs::channels::alter_green_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_green_channel(&mut img, 20_i16);
 * ```
 * @param {PhotonImage} img
 * @param {number} amt
 */
export function alter_green_channel(img, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_green_channel(img.__wbg_ptr, amt);
}

/**
 * Increment or decrement every pixel's Red channel by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `amt` - The amount to increment or decrement the channel's value by for that pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the Red channel for all pixels by 10:
 * use photon_rs::channels::alter_red_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_red_channel(&mut img, 10_i16);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} amt
 */
export function alter_red_channel(photon_image, amt) {
    _assertClass(photon_image, PhotonImage);
    wasm.alter_red_channel(photon_image.__wbg_ptr, amt);
}

/**
 * Increment/decrement two channels' values simultaneously by adding an amt to each channel per pixel.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `channel1` - A usize from 0 to 2 that represents either the R, G or B channels.
 * * `amt1` - The amount to increment/decrement the channel's value by for that pixel.
 * * `channel2` -A usize from 0 to 2 that represents either the R, G or B channels.
 * * `amt2` - The amount to increment/decrement the channel's value by for that pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the values of the Red and Blue channels per pixel:
 * use photon_rs::channels::alter_two_channels;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_two_channels(&mut img, 0_usize, 10_i16, 2_usize, 20_i16);
 * ```
 * @param {PhotonImage} img
 * @param {number} channel1
 * @param {number} amt1
 * @param {number} channel2
 * @param {number} amt2
 */
export function alter_two_channels(img, channel1, amt1, channel2, amt2) {
    _assertClass(img, PhotonImage);
    wasm.alter_two_channels(img.__wbg_ptr, channel1, amt1, channel2, amt2);
}

/**
 * Apply a gradient to an image.
 * @param {PhotonImage} image
 */
export function apply_gradient(image) {
    _assertClass(image, PhotonImage);
    wasm.apply_gradient(image.__wbg_ptr);
}

/**
 * 应用遮罩到图像
 * @param {Uint8Array} image_bytes
 * @param {Uint8Array} mask
 * @param {number} width
 * @param {number} height
 */
export function apply_mask_to_image(image_bytes, mask, width, height) {
    var ptr0 = passArray8ToWasm0(image_bytes, wasm.__wbindgen_export);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(mask, wasm.__wbindgen_export);
    const len1 = WASM_VECTOR_LEN;
    wasm.apply_mask_to_image(ptr0, len0, addHeapObject(image_bytes), ptr1, len1, width, height);
}

/**
 * 自动抠图 - 基于颜色的智能抠图
 * @param {Uint8Array} image_bytes
 * @param {number} width
 * @param {number} height
 * @param {number} target_r
 * @param {number} target_g
 * @param {number} target_b
 * @param {number} tolerance
 * @param {number} feather_radius
 * @returns {Uint8Array}
 */
export function auto_crop_by_color(image_bytes, width, height, target_r, target_g, target_b, tolerance, feather_radius) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(image_bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.auto_crop_by_color(retptr, ptr0, len0, width, height, target_r, target_g, target_b, tolerance, feather_radius);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 1, 1);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Blue channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::monochrome::b_grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * b_grayscale(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function b_grayscale(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.b_grayscale(photon_image.__wbg_ptr);
}

/**
 * Convert a base64 string to a PhotonImage.
 * @param {string} base64
 * @returns {PhotonImage}
 */
export function base64_to_image(base64) {
    const ptr0 = passStringToWasm0(base64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.base64_to_image(ptr0, len0);
    return PhotonImage.__wrap(ret);
}

/**
 * Convert a base64 string to a Vec of u8s.
 * @param {string} base64
 * @returns {Uint8Array}
 */
export function base64_to_vec(base64) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(base64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.base64_to_vec(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 1, 1);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Batch process multiple images efficiently.
 *
 * This function processes multiple images in a single call, reducing
 * JavaScript-WASM bridge overhead.
 *
 * # Arguments
 * * `images` - An array of PhotonImage objects.
 * * `processor` - A function that processes a single image.
 *
 * # Example
 *
 * ```javascript
 * import { batch_process_images } from 'photon_rs';
 *
 * const images = [
 *     img1,
 *     img2,
 *     img3
 * ];
 *
 * batch_process_images(images, (img) => {
 *     // Apply processing to each image
 *     return processImage(img);
 * });
 * ```
 * @param {Array<any>} images
 * @param {Function} processor
 * @returns {Array<any>}
 */
export function batch_process_images(images, processor) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.batch_process_images(retptr, addHeapObject(images), addBorrowedObject(processor));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        heap[stack_pointer++] = undefined;
    }
}

/**
 * Apply bilateral filter to an image.
 *
 * Bilateral filter is a non-linear, edge-preserving, and noise-reducing smoothing filter.
 * Unlike Gaussian blur, it preserves edges while smoothing homogeneous regions.
 *
 * # Algorithm Selection
 * - When `fast_mode=true`: Uses Domain Transform algorithm (O(n) complexity, 10-50x faster)
 * - When `fast_mode=false`: Uses standard bilateral filter with pre-computed weights (O(n*k²) complexity)
 *
 * # Performance Optimizations
 * Fast mode (Domain Transform):
 * - Time Complexity: O(n) - independent of kernel size
 * - Space Complexity: O(n)
 * - Typical Speedup: 10-50x compared to standard mode
 *
 * Standard mode:
 * - Pre-computed spatial weights: O(1) lookup instead of exp() calculation
 * - Pre-computed range weights: O(1) lookup for color similarity
 * - Direct pixel access: Avoids expensive get_pixel() calls
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to filter
 * * `sigma_spatial` - Spatial domain standard deviation (controls smoothing radius)
 * * `sigma_range` - Range domain standard deviation (controls edge sensitivity)
 * * `fast_mode` - When true, uses fast Domain Transform algorithm (default: true for performance)
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::conv::bilateral_filter;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * // Fast mode (recommended for most use cases)
 * bilateral_filter(&mut img, 5.0, 30.0, true);
 * // Standard mode (when quality is paramount)
 * bilateral_filter(&mut img, 5.0, 30.0, false);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} sigma_spatial
 * @param {number} sigma_range
 * @param {boolean} fast_mode
 */
export function bilateral_filter(photon_image, sigma_spatial, sigma_range, fast_mode) {
    _assertClass(photon_image, PhotonImage);
    wasm.bilateral_filter(photon_image.__wbg_ptr, sigma_spatial, sigma_range, fast_mode);
}

/**
 * Fast bilateral filter using Domain Transform.
 *
 * This implementation uses the Domain Transform technique, which achieves O(n) complexity
 * instead of O(n*k²) for the standard bilateral filter. It's particularly effective for
 * real-time applications and large images.
 *
 * The algorithm works by:
 * 1. Computing color-based distances between adjacent pixels
 * 2. Applying recursive filtering along horizontal and vertical passes
 * 3. Using a reference image to guide the filtering
 *
 * # Performance Characteristics
 * - Time Complexity: O(n) where n is the number of pixels
 * - Space Complexity: O(n) for temporary buffers
 * - Typical Speedup: 10-50x compared to standard bilateral filter
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to filter
 * * `sigma_spatial` - Spatial domain standard deviation (controls smoothing radius)
 * * `sigma_range` - Range domain standard deviation (controls edge sensitivity)
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::conv::bilateral_filter_fast;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * bilateral_filter_fast(&mut img, 5.0, 30.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} sigma_spatial
 * @param {number} sigma_range
 */
export function bilateral_filter_fast(photon_image, sigma_spatial, sigma_range) {
    _assertClass(photon_image, PhotonImage);
    wasm.bilateral_filter_fast(photon_image.__wbg_ptr, sigma_spatial, sigma_range);
}

/**
 * Fast bilateral filter with adjustable iterations.
 *
 * This is a more flexible version of bilateral_filter_fast that allows
 * control over the number of filtering iterations. More iterations
 * produce stronger smoothing at the cost of additional computation.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to filter
 * * `sigma_spatial` - Spatial domain standard deviation (controls smoothing radius)
 * * `sigma_range` - Range domain standard deviation (controls edge sensitivity)
 * * `iterations` - Number of filtering iterations (1-10, default 3)
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::conv::bilateral_filter_fast_iter;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * // 5 iterations for stronger smoothing
 * bilateral_filter_fast_iter(&mut img, 5.0, 30.0, 5);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} sigma_spatial
 * @param {number} sigma_range
 * @param {number} iterations
 */
export function bilateral_filter_fast_iter(photon_image, sigma_spatial, sigma_range, iterations) {
    _assertClass(photon_image, PhotonImage);
    wasm.bilateral_filter_fast_iter(photon_image.__wbg_ptr, sigma_spatial, sigma_range, iterations);
}

/**
 * Blend two images together.
 *
 * The `blend_mode` (3rd param) determines which blending mode to use; change this for varying effects.
 * The blend modes available include: `overlay`, `over`, `atop`, `xor`, `plus`, `multiply`, `burn`,
 * `difference`, `soft_light`, `screen`, `hard_light`, `dodge`, `exclusion`, `lighten`, `darken` (more to come)
 * NOTE: The first image must be smaller than the second image passed as params.
 * If the first image were larger than the second, then there would be overflowing pixels which would have no corresponding pixels
 * in the second image.
 * # Arguments
 * * `img` - A DynamicImage that contains a view into the image.
 * * `img2` - The 2nd DynamicImage to be blended with the first.
 * * `blend_mode` - The blending mode to use. See above for complete list of blend modes available.
 * # Example
 *
 * ```no_run
 * // For example, to blend two images with the `multiply` blend mode:
 * use photon_rs::multiple::blend;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let img2 = open_image("img2.jpg").expect("File should open");
 * blend(&mut img, &img2, "multiply");
 * ```
 * @param {PhotonImage} photon_image
 * @param {PhotonImage} photon_image2
 * @param {string} blend_mode
 */
export function blend(photon_image, photon_image2, blend_mode) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(photon_image2, PhotonImage);
    const ptr0 = passStringToWasm0(blend_mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.blend(photon_image.__wbg_ptr, photon_image2.__wbg_ptr, ptr0, len0);
}

/**
 * Adaptive blend function that automatically selects the optimal algorithm
 * based on image size.
 *
 * - For small images: Uses the standard blend function for maximum compatibility
 * - For medium/large images: Uses the fast version that works directly on raw pixels
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `photon_image2` - The 2nd PhotonImage to be blended with the first.
 * * `blend_mode` - The blending mode to use.
 * @param {PhotonImage} photon_image
 * @param {PhotonImage} photon_image2
 * @param {string} blend_mode
 */
export function blend_adaptive(photon_image, photon_image2, blend_mode) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(photon_image2, PhotonImage);
    const ptr0 = passStringToWasm0(blend_mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.blend_adaptive(photon_image.__wbg_ptr, photon_image2.__wbg_ptr, ptr0, len0);
}

/**
 * Optimized version of blend function that works directly on raw pixel data.
 * This avoids creating DynamicImage objects multiple times and reduces memory allocations.
 * Provides 1.3-1.5x performance improvement over the standard blend function.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `photon_image2` - The 2nd PhotonImage to be blended with the first.
 * * `blend_mode` - The blending mode to use.
 * @param {PhotonImage} photon_image
 * @param {PhotonImage} photon_image2
 * @param {string} blend_mode
 */
export function blend_fast(photon_image, photon_image2, blend_mode) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(photon_image2, PhotonImage);
    const ptr0 = passStringToWasm0(blend_mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.blend_fast(photon_image.__wbg_ptr, photon_image2.__wbg_ptr, ptr0, len0);
}

/**
 * Apply a box blur effect.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a box blur effect:
 * use photon_rs::conv::box_blur;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * box_blur(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function box_blur(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.box_blur(photon_image.__wbg_ptr);
}

/**
 * Increased contrast filter effect.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::cali;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * cali(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function cali(img) {
    _assertClass(img, PhotonImage);
    wasm.cali(img.__wbg_ptr);
}

/**
 * Horizontal strips. Divide an image into a series of equal-width strips, for an artistic effect. Sepcify a color as well.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `num_strips` - The numbder of strips
 * * `color` - Color of strips.
 * # Example
 *
 * ```no_run
 * // For example, to draw blue horizontal strips on a `PhotonImage`:
 * use photon_rs::effects::color_horizontal_strips;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgb;
 *
 * let color = Rgb::new(255u8, 0u8, 0u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * color_horizontal_strips(&mut img, 8u8, color);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_strips
 * @param {Rgb} color
 */
export function color_horizontal_strips(photon_image, num_strips, color) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color, Rgb);
    var ptr0 = color.__destroy_into_raw();
    wasm.color_horizontal_strips(photon_image.__wbg_ptr, num_strips, ptr0);
}

/**
 * Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect. Sepcify a color as well.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `num_strips` - The numbder of strips
 * * `color` - Color of strips.
 * # Example
 *
 * ```no_run
 * // For example, to draw red vertical strips on a `PhotonImage`:
 * use photon_rs::effects::color_vertical_strips;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgb;
 *
 * let color = Rgb::new(255u8, 0u8, 0u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * color_vertical_strips(&mut img, 8u8, color);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_strips
 * @param {Rgb} color
 */
export function color_vertical_strips(photon_image, num_strips, color) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color, Rgb);
    var ptr0 = color.__destroy_into_raw();
    wasm.color_vertical_strips(photon_image.__wbg_ptr, num_strips, ptr0);
}

/**
 * Colorizes the green channels of the image.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to colorize an image of type `PhotonImage`:
 * use photon_rs::effects::colorize;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * colorize(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function colorize(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.colorize(photon_image.__wbg_ptr);
}

/**
 * 创建圆形遮罩（带抗锯齿）
 * @param {number} width
 * @param {number} height
 * @param {number} center_x
 * @param {number} center_y
 * @param {number} radius
 * @param {number} feather_radius
 * @returns {Uint8Array}
 */
export function create_circular_mask(width, height, center_x, center_y, radius, feather_radius) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.create_circular_mask(retptr, width, height, center_x, center_y, radius, feather_radius);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 1, 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {number} width
 * @param {number} height
 * @returns {PhotonImage}
 */
export function create_gradient(width, height) {
    const ret = wasm.create_gradient(width, height);
    return PhotonImage.__wrap(ret);
}

/**
 * 创建多边形遮罩
 *
 * # 参数
 * * `width` - 图像宽度
 * * `height` - 图像高度
 * * `vertices` - 多边形顶点坐标数组 [x1, y1, x2, y2, ...]
 * * `anti_aliased` - 是否启用抗锯齿
 *
 * # 返回
 * 遮罩像素数据（灰度图，每个像素 1 字节）
 * @param {number} width
 * @param {number} height
 * @param {Float32Array} vertices
 * @param {boolean} anti_aliased
 * @returns {Uint8Array}
 */
export function create_polygon_mask(width, height, vertices, anti_aliased) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayF32ToWasm0(vertices, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.create_polygon_mask(retptr, width, height, ptr0, len0, anti_aliased);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 1, 1);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Crop an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to crop an image at (0, 0) to (500, 800)
 * use photon_rs::native::{open_image};
 * use photon_rs::transform::crop;
 * use photon_rs::PhotonImage;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let cropped_img: PhotonImage = crop(&img, 0_u32, 0_u32, 500_u32, 800_u32);
 * // Write the contents of this image in JPG format.
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} x1
 * @param {number} y1
 * @param {number} x2
 * @param {number} y2
 * @returns {PhotonImage}
 */
export function crop(photon_image, x1, y1, x2, y2) {
    _assertClass(photon_image, PhotonImage);
    const ret = wasm.crop(photon_image.__wbg_ptr, x1, y1, x2, y2);
    return PhotonImage.__wrap(ret);
}

/**
 * @param {HTMLCanvasElement} source_canvas
 * @param {number} width
 * @param {number} height
 * @param {number} left
 * @param {number} top
 * @returns {HTMLCanvasElement}
 */
export function crop_img_browser(source_canvas, width, height, left, top) {
    const ret = wasm.crop_img_browser(addHeapObject(source_canvas), width, height, left, top);
    return takeObject(ret);
}

/**
 * Darken the image by a specified amount in the HSL colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Darkening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to darken an image by 10% in the HSL colour space:
 * use photon_rs::colour_spaces::darken_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * darken_hsl(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function darken_hsl(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsl(img.__wbg_ptr, level);
}

/**
 * Darken the image by a specified amount in the HSLuv colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Darkening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to darken an image by 10% in the HSLuv colour space:
 * use photon_rs::colour_spaces::darken_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * darken_hsluv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function darken_hsluv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsluv(img.__wbg_ptr, level);
}

/**
 * Darken the image's colours by a specified amount in the HSV colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Darkening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to darken an image by 10% in the HSV colour space:
 * use photon_rs::colour_spaces::darken_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * darken_hsv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function darken_hsv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsv(img.__wbg_ptr, level);
}

/**
 * Darken the image by a specified amount in the LCh colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Darkening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to darken an image by 10% in the LCh colour space:
 * use photon_rs::colour_spaces::darken_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * darken_lch(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function darken_lch(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_lch(img.__wbg_ptr, level);
}

/**
 * Decrease the brightness of an image by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `brightness` - A u8 to subtract from the brightness. It should be a positive number,
 * and this value will then be subtracted from the brightness.
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::dec_brightness;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * dec_brightness(&mut img, 10_u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} brightness
 */
export function dec_brightness(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.dec_brightness(photon_image.__wbg_ptr, brightness);
}

/**
 * Uses a max. decomposition algorithm to convert an image to greyscale.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to decompose an image with max decomposition:
 * use photon_rs::monochrome::decompose_max;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * decompose_max(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function decompose_max(img) {
    _assertClass(img, PhotonImage);
    wasm.decompose_max(img.__wbg_ptr);
}

/**
 * Uses a min. decomposition algorithm to convert an image to greyscale.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to decompose an image with min decomposition:
 * use photon_rs::monochrome::decompose_min;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * decompose_min(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function decompose_min(img) {
    _assertClass(img, PhotonImage);
    wasm.decompose_min(img.__wbg_ptr);
}

/**
 * Desaturate an image by getting the min/max of each pixel's RGB values.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * // For example, to desaturate an image:
 * use photon_rs::monochrome::desaturate;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function desaturate(img) {
    _assertClass(img, PhotonImage);
    wasm.desaturate(img.__wbg_ptr);
}

/**
 * Desaturate the image by a specified amount in the HSL colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Desaturating by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to desaturate an image by 10% in the LCh colour space:
 * use photon_rs::colour_spaces::desaturate_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate_hsl(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function desaturate_hsl(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsl(img.__wbg_ptr, level);
}

/**
 * Desaturate the image by a specified amount in the HSLuv colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Desaturating by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to desaturate an image by 10% in the HSLuv colour space:
 * use photon_rs::colour_spaces::desaturate_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate_hsluv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function desaturate_hsluv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsluv(img.__wbg_ptr, level);
}

/**
 * Desaturate the image by a specified amount in the HSV colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Desaturating by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to desaturate an image by 10% in the HSV colour space:
 * use photon_rs::colour_spaces::desaturate_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate_hsv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function desaturate_hsv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsv(img.__wbg_ptr, level);
}

/**
 * Desaturate the image by a specified amount in the LCh colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Desaturating by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to desaturate an image by 10% in the LCh colour space:
 * use photon_rs::colour_spaces::desaturate_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate_lch(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function desaturate_lch(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_lch(img.__wbg_ptr, level);
}

/**
 * Detect lines at a 135 degree angle in an image, and highlight these only.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to display the lines at a 135 degree angle in an image:
 * use photon_rs::conv::detect_135_deg_lines;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * detect_135_deg_lines(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function detect_135_deg_lines(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_135_deg_lines(photon_image.__wbg_ptr);
}

/**
 * Detect lines at a forty five degree angle in an image, and highlight these only.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to display the lines at a forty five degree angle in an image:
 * use photon_rs::conv::detect_45_deg_lines;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * detect_45_deg_lines(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function detect_45_deg_lines(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_45_deg_lines(photon_image.__wbg_ptr);
}

/**
 * Detect horizontal lines in an image, and highlight these only.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to display the horizontal lines in an image:
 * use photon_rs::conv::detect_horizontal_lines;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * detect_horizontal_lines(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function detect_horizontal_lines(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_horizontal_lines(photon_image.__wbg_ptr);
}

/**
 * Detect vertical lines in an image, and highlight these only.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to display the vertical lines in an image:
 * use photon_rs::conv::detect_vertical_lines;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * detect_vertical_lines(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function detect_vertical_lines(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_vertical_lines(photon_image.__wbg_ptr);
}

/**
 * Applies Floyd-Steinberg dithering to an image.
 * Only RGB channels are processed, alpha remains unchanged.
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `depth` - bits per channel. Clamped between 1 and 8.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a dithered image:
 * use photon_rs::effects::dither;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let depth = 1;
 * dither(&mut img, depth);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} depth
 */
export function dither(photon_image, depth) {
    _assertClass(photon_image, PhotonImage);
    wasm.dither(photon_image.__wbg_ptr, depth);
}

/**
 * Applies Ordered Dithering (Bayer Matrix) to an image.
 * This is faster than Floyd-Steinberg dithering and produces better results for some images.
 * Only RGB channels are processed, alpha remains unchanged.
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `depth` - bits per channel. Clamped between 1 and 8.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a dithered image:
 * use photon_rs::effects::dither_ordered;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let depth = 1;
 * dither_ordered(&mut img, depth);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} depth
 */
export function dither_ordered(photon_image, depth) {
    _assertClass(photon_image, PhotonImage);
    wasm.dither_ordered(photon_image.__wbg_ptr, depth);
}

/**
 * Greyscale effect with increased contrast.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::dramatic;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * dramatic(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function dramatic(img) {
    _assertClass(img, PhotonImage);
    wasm.dramatic(img.__wbg_ptr);
}

/**
 * Add text to an image.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `text` - Text string to be drawn to the image.
 * * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
 * * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
 * * `font_size` - Font size in pixels of the text to be drawn.
 * * `font_name` - Name of the registered font to use.
 *
 * # Example
 *
 * ```no_run
 * // For example to draw the string "Welcome to Photon!" at 10, 10:
 * use photon_rs::native::open_image;
 * use photon_rs::text::draw_text;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * // Make sure to register a font first
 * photon_rs::text::register_font("my-font", font_data);
 * draw_text(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32, "my-font");
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 * @param {number} font_size
 * @param {string} font_name
 */
export function draw_text(photon_img, text, x, y, font_size, font_name) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(font_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    wasm.draw_text(photon_img.__wbg_ptr, ptr0, len0, x, y, font_size, ptr1, len1);
}

/**
 * Add bordered-text to an image.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `text` - Text string to be drawn to the image.
 * * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
 * * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
 * * `font_size` - Font size in pixels of the text to be drawn.
 * * `font_name` - Name of the registered font to use.
 *
 * # Example
 *
 * ```no_run
 * // For example to draw the string "Welcome to Photon!" at 10, 10:
 * use photon_rs::native::open_image;
 * use photon_rs::text::draw_text_with_border;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * // Make sure to register a font first
 * photon_rs::text::register_font("my-font", font_data);
 * draw_text_with_border(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32, "my-font");
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 * @param {number} font_size
 * @param {string} font_name
 */
export function draw_text_with_border(photon_img, text, x, y, font_size, font_name) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(font_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    wasm.draw_text_with_border(photon_img.__wbg_ptr, ptr0, len0, x, y, font_size, ptr1, len1);
}

/**
 * Add bordered-text to an image with custom color.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `text` - Text string to be drawn to the image.
 * * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
 * * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
 * * `font_size` - Font size in pixels of the text to be drawn.
 * * `r` - Red channel (0-255).
 * * `g` - Green channel (0-255).
 * * `b` - Blue channel (0-255).
 * * `font_name` - Name of the registered font to use.
 *
 * # Example
 *
 * ```no_run
 * // For example to draw red text with border at 10, 10:
 * use photon_rs::native::open_image;
 * use photon_rs::text::draw_text_with_border_and_color;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * // Make sure to register a font first
 * photon_rs::text::register_font("my-font", font_data);
 * draw_text_with_border_and_color(&mut img, "Hello!", 10_i32, 10_i32, 90_f32, 255u8, 0u8, 0u8, "my-font");
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 * @param {number} font_size
 * @param {number} r
 * @param {number} g
 * @param {number} b
 * @param {string} font_name
 */
export function draw_text_with_border_and_color(photon_img, text, x, y, font_size, r, g, b, font_name) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(font_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    wasm.draw_text_with_border_and_color(photon_img.__wbg_ptr, ptr0, len0, x, y, font_size, r, g, b, ptr1, len1);
}

/**
 * Add text to an image with custom color.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `text` - Text string to be drawn to the image.
 * * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
 * * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
 * * `font_size` - Font size in pixels of the text to be drawn.
 * * `r` - Red channel (0-255).
 * * `g` - Green channel (0-255).
 * * `b` - Blue channel (0-255).
 * * `font_name` - Name of the registered font to use.
 *
 * # Example
 *
 * ```no_run
 * // For example to draw red text at 10, 10:
 * use photon_rs::native::open_image;
 * use photon_rs::text::draw_text_with_color;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * // Make sure to register a font first
 * photon_rs::text::register_font("my-font", font_data);
 * draw_text_with_color(&mut img, "Hello!", 10_i32, 10_i32, 90_f32, 255u8, 0u8, 0u8, "my-font");
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 * @param {number} font_size
 * @param {number} r
 * @param {number} g
 * @param {number} b
 * @param {string} font_name
 */
export function draw_text_with_color(photon_img, text, x, y, font_size, r, g, b, font_name) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(font_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    wasm.draw_text_with_color(photon_img.__wbg_ptr, ptr0, len0, x, y, font_size, r, g, b, ptr1, len1);
}

/**
 * @param {PhotonImage} photon_image
 * @param {Rgb} color_a
 * @param {Rgb} color_b
 */
export function duotone(photon_image, color_a, color_b) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color_a, Rgb);
    var ptr0 = color_a.__destroy_into_raw();
    _assertClass(color_b, Rgb);
    var ptr1 = color_b.__destroy_into_raw();
    wasm.duotone(photon_image.__wbg_ptr, ptr0, ptr1);
}

/**
 * Duotone effect with purple tones.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_horizon;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * duotone_horizon(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function duotone_horizon(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_horizon(img.__wbg_ptr);
}

/**
 * Duotone effect with a lilac hue
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_lilac;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * duotone_lilac(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function duotone_lilac(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_lilac(img.__wbg_ptr);
}

/**
 * A duotone ochre tint effect
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_ochre;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * duotone_ochre(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function duotone_ochre(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_ochre(img.__wbg_ptr);
}

/**
 * A duotone filter with a user-specified color and a gray color
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `rgb_color` - RGB color
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_tint;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgb;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgb_color = Rgb::new(12, 12, 10);
 * duotone_tint(&mut img, rgb_color);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} rgb_color
 */
export function duotone_tint(img, rgb_color) {
    _assertClass(img, PhotonImage);
    _assertClass(rgb_color, Rgb);
    var ptr0 = rgb_color.__destroy_into_raw();
    wasm.duotone_tint(img.__wbg_ptr, ptr0);
}

/**
 * Duotone effect with blue and purple tones.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_violette;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * duotone_violette(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function duotone_violette(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_violette(img.__wbg_ptr);
}

/**
 * Apply edge detection to an image, to create a dark version with its edges highlighted.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the Red channel for all pixels by 10:
 * use photon_rs::conv::edge_detection;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * edge_detection(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function edge_detection(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.edge_detection(photon_image.__wbg_ptr);
}

/**
 * Preset edge effect.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply this effect:
 * use photon_rs::conv::edge_one;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * edge_one(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function edge_one(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.edge_one(photon_image.__wbg_ptr);
}

/**
 * Apply an emboss effect to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply an emboss effect:
 * use photon_rs::conv::emboss;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * emboss(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function emboss(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.emboss(photon_image.__wbg_ptr);
}

/**
 * Apply a filter to an image. Over 20 filters are available.
 * The filters are as follows:
 * * **oceanic**: Add an aquamarine-tinted hue to an image.
 * * **islands**: Aquamarine tint.
 * * **marine**: Add a green/blue mixed hue to an image.
 * * **seagreen**: Dark green hue, with tones of blue.
 * * **flagblue**: Royal blue tint
 * * **liquid**: Blue-inspired tint.
 * * **diamante**: Custom filter with a blue/turquoise tint.
 * * **radio**: Fallout-style radio effect.
 * * **twenties**: Slight-blue tinted historical effect.
 * * **rosetint**: Rose-tinted filter.
 * * **mauve**: Purple-infused filter.
 * * **bluechrome**: Blue monochrome effect.
 * * **vintage**: Vintage filter with a red tint.
 * * **perfume**: Increase the blue channel, with moderate increases in the Red and Green channels.
 * * **serenity**: Custom filter with an increase in the Blue channel's values.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `filter_name` - The filter's name. Choose from the selection above, eg: "oceanic"
 * # Example
 *
 * ```no_run
 * // For example, to add a filter called "vintage" to an image:
 * use photon_rs::filters::filter;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * filter(&mut img, "vintage");
 * ```
 * @param {PhotonImage} img
 * @param {string} filter_name
 */
export function filter(img, filter_name) {
    _assertClass(img, PhotonImage);
    const ptr0 = passStringToWasm0(filter_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.filter(img.__wbg_ptr, ptr0, len0);
}

/**
 * Apply a red hue, with increased contrast and brightness.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::firenze;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * firenze(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function firenze(img) {
    _assertClass(img, PhotonImage);
    wasm.firenze(img.__wbg_ptr);
}

/**
 * Flip an image horizontally.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to flip an image horizontally:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::fliph;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * fliph(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function fliph(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.fliph(photon_image.__wbg_ptr);
}

/**
 * Flip an image vertically.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to flip an image vertically:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::flipv;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * flipv(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function flipv(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.flipv(photon_image.__wbg_ptr);
}

/**
 * Turn an image into an frosted glass see through
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into frosted glass see through:
 * use photon_rs::effects::frosted_glass;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * frosted_glass(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function frosted_glass(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.frosted_glass(photon_image.__wbg_ptr);
}

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Green channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::monochrome::g_grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * g_grayscale(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function g_grayscale(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.g_grayscale(photon_image.__wbg_ptr);
}

/**
 * Applies gamma correction to an image.
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `red` - Gamma value for red channel.
 * * `green` - Gamma value for green channel.
 * * `blue` - Gamma value for blue channel.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a gamma corrected image:
 * use photon_rs::colour_spaces::gamma_correction;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * gamma_correction(&mut img, 2.2, 2.2, 2.2);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} red
 * @param {number} green
 * @param {number} blue
 */
export function gamma_correction(photon_image, red, green, blue) {
    _assertClass(photon_image, PhotonImage);
    wasm.gamma_correction(photon_image.__wbg_ptr, red, green, blue);
}

/**
 * Gaussian blur in linear time.
 *
 * Reference: http://blog.ivank.net/fastest-gaussian-blur.html
 *
 * This implementation uses a separable box blur approximation for optimal performance,
 * especially effective for large blur radii. The algorithm approximates Gaussian blur
 * by applying three successive box blurs with carefully calculated radii.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage
 * * `radius` - blur radius (larger values create more blur)
 * # Example
 *
 * ```no_run
 * use photon_rs::conv::gaussian_blur;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * gaussian_blur(&mut img, 3_i32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} radius
 */
export function gaussian_blur(photon_image, radius) {
    _assertClass(photon_image, PhotonImage);
    wasm.gaussian_blur(photon_image.__wbg_ptr, radius);
}

/**
 * Fast separable Gaussian blur using SIMD optimization.
 *
 * This is an optimized version of Gaussian blur that uses SIMD instructions
 * for better performance on modern CPUs and WebAssembly. It's particularly
 * effective for large blur radii and large images.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage
 * * `radius` - blur radius (larger values create more blur)
 * # Example
 *
 * ```no_run
 * use photon_rs::conv::gaussian_blur_fast;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * gaussian_blur_fast(&mut img, 5_i32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} radius
 */
export function gaussian_blur_fast(photon_image, radius) {
    _assertClass(photon_image, PhotonImage);
    wasm.gaussian_blur_fast(photon_image.__wbg_ptr, radius);
}

/**
 * Apply Gaussian blur using parallel processing for better performance on multi-core systems.
 *
 * This is the parallel-optimized version of `gaussian_blur`. It uses Rayon to process
 * the image in parallel, which can provide 2-4x speedup on multi-core CPUs.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to blur
 * * `radius` - Blur radius (larger values create more blur)
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::conv::gaussian_blur_parallel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * gaussian_blur_parallel(&mut img, 5_i32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} radius
 */
export function gaussian_blur_parallel(photon_image, radius) {
    _assertClass(photon_image, PhotonImage);
    wasm.gaussian_blur_parallel(photon_image.__wbg_ptr, radius);
}

/**
 * Tiled Gaussian blur for better cache locality on large images.
 *
 * This implementation processes the image in tiles to improve cache performance,
 * especially for large images. Each tile is processed independently, with proper
 * handling of tile boundaries.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage
 * * `radius` - blur radius
 * * `tile_size` - Size of each tile (default 256 for good cache performance)
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::conv::gaussian_blur_tiled;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * gaussian_blur_tiled(&mut img, 5_i32, 256);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} radius
 * @param {number} tile_size
 */
export function gaussian_blur_tiled(photon_image, radius, tile_size) {
    _assertClass(photon_image, PhotonImage);
    wasm.gaussian_blur_tiled(photon_image.__wbg_ptr, radius, tile_size);
}

/**
 * 获取默认字体名称
 *
 * # 返回
 * 返回默认字体的名称字符串
 * @returns {string}
 */
export function get_default_font_name() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.get_default_font_name(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Get the ImageData from a 2D canvas context
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @returns {ImageData}
 */
export function get_image_data(canvas, ctx) {
    try {
        const ret = wasm.get_image_data(addBorrowedObject(canvas), addBorrowedObject(ctx));
        return takeObject(ret);
    } finally {
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;
    }
}

/**
 * Apply a vintage, golden hue to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::golden;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * golden(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function golden(img) {
    _assertClass(img, PhotonImage);
    wasm.golden(img.__wbg_ptr);
}

/**
 * Convert an image to grayscale using the conventional averaging algorithm.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * // For example, to convert an image of type `PhotonImage` to grayscale:
 * use photon_rs::monochrome::grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * grayscale(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function grayscale(img) {
    _assertClass(img, PhotonImage);
    wasm.grayscale(img.__wbg_ptr);
}

/**
 * Convert an image to grayscale with a human corrected factor, to account for human vision.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * // For example, to convert an image of type `PhotonImage` to grayscale with a human corrected factor:
 * use photon_rs::monochrome::grayscale_human_corrected;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * grayscale_human_corrected(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function grayscale_human_corrected(img) {
    _assertClass(img, PhotonImage);
    wasm.grayscale_human_corrected(img.__wbg_ptr);
}

/**
 * Convert an image to grayscale using parallel processing.
 *
 * This is the parallel version of the grayscale operation.
 * Uses human-corrected luminance formula: 0.3*R + 0.59*G + 0.11*B
 *
 * # Arguments
 * * `photon_image` - A mutable reference to a PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::parallel::grayscale_parallel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * grayscale_parallel(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function grayscale_parallel(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.grayscale_parallel(photon_image.__wbg_ptr);
}

/**
 * Employ only a limited number of gray shades in an image.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `num_shades` - The number of grayscale shades to be displayed in the image.
 *
 * # Example
 *
 * ```no_run
 * // For example, to limit an image to four shades of gray only:
 * use photon_rs::monochrome::grayscale_shades;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * grayscale_shades(&mut img, 4_u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_shades
 */
export function grayscale_shades(photon_image, num_shades) {
    _assertClass(photon_image, PhotonImage);
    wasm.grayscale_shades(photon_image.__wbg_ptr, num_shades);
}

/**
 * Halftoning effect.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example:
 * use photon_rs::effects::halftone;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * halftone(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function halftone(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.halftone(photon_image.__wbg_ptr);
}

/**
 * Horizontal strips. Divide an image into a series of equal-height strips, for an artistic effect.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `num_strips` - The number of strips
 * # Example
 *
 * ```no_run
 * // For example, to draw horizontal strips on a `PhotonImage`:
 * use photon_rs::effects::horizontal_strips;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * horizontal_strips(&mut img, 8u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_strips
 */
export function horizontal_strips(photon_image, num_strips) {
    _assertClass(photon_image, PhotonImage);
    wasm.horizontal_strips(photon_image.__wbg_ptr, num_strips);
}

/**
 * Image manipulation effects in the HSL colour space.
 *
 * Effects include:
 * * **saturate** - Saturation increase.
 * * **desaturate** - Desaturate the image.
 * * **shift_hue** - Hue rotation by a specified number of degrees.
 * * **darken** - Decrease the brightness.
 * * **lighten** - Increase the brightness.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 * # Example
 * ```no_run
 * // For example to increase the saturation by 10%:
 * use photon_rs::colour_spaces::hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hsl(&mut img, "saturate", 0.1_f32);
 * ```
 *
 * # Performance
 * This function now uses SIMD-optimized algorithms internally for better performance (1.5-2x improvement).
 * For maximum accuracy with small images, use `hsl_with_palette()` instead.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsl(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsl(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Adaptive HSL color space manipulation that automatically selects the optimal algorithm
 * based on image size.
 *
 * - For small images: Uses the standard palette library for maximum accuracy
 * - For medium/large images: Uses the fast version with optimized conversion algorithms
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired: "saturate", "desaturate", "lighten", "darken", "shift_hue"
 * * `amt` - A float value from 0 to 1.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsl_adaptive(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsl_adaptive(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Adaptive HSL function that automatically selects the optimal algorithm
 * based on image size and available optimizations.
 *
 * - For small images: Uses the standard palette library for maximum accuracy
 * - For medium images: Uses the fast version with optimized conversion algorithms
 * - For large images: Uses the SIMD version for maximum performance
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired: "saturate", "desaturate", "lighten", "darken", "shift_hue"
 * * `amt` - A float value from 0 to 1.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsl_auto(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsl_auto(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Optimized version of HSL color space manipulation using pre-computed lookup tables.
 * This provides 1.5-2x performance improvement over the standard version for saturation and lightness operations.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired: "saturate", "desaturate", "lighten", "darken", "shift_hue"
 * * `amt` - A float value from 0 to 1.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsl_fast(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsl_fast(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * HSL color space manipulation using the palette library for maximum accuracy.
 * This is slower than the default `hsl()` function but provides more accurate color conversions.
 * Use this for small images where accuracy is more important than performance.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsl_with_palette(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsl_with_palette(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Image manipulation effects in the HSLuv colour space
 *
 * Effects include:
 * * **saturate** - Saturation increase.
 * * **desaturate** - Desaturate the image.
 * * **shift_hue** - Hue rotation by a specified number of degrees.
 * * **darken** - Decrease the brightness.
 * * **lighten** - Increase the brightness.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 * # Example
 * ```no_run
 * // For example to increase the saturation by 10%:
 * use photon_rs::colour_spaces::hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hsluv(&mut img, "saturate", 0.1_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsluv(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsluv(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Image manipulation in the HSV colour space.
 *
 * Effects include:
 * * **saturate** - Saturation increase.
 * * **desaturate** - Desaturate the image.
 * * **shift_hue** - Hue rotation by a specified number of degrees.
 * * **darken** - Decrease the brightness.
 * * **lighten** - Increase the brightness.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 *
 * # Example
 * ```no_run
 * // For example to increase the saturation by 10%:
 * use photon_rs::colour_spaces::hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hsv(&mut img, "saturate", 0.1_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsv(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsv(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Adaptive HSV color space manipulation that automatically selects the optimal algorithm
 * based on image size.
 *
 * - For small images: Uses the standard palette library for maximum accuracy
 * - For medium/large images: Uses the fast version with optimized conversion algorithms
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired: "saturate", "desaturate", "lighten", "darken", "shift_hue"
 * * `amt` - A float value from 0 to 1.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsv_adaptive(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsv_adaptive(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Adaptive HSV function that automatically selects the optimal algorithm
 * based on image size and available optimizations.
 *
 * - For small images: Uses the standard palette library for maximum accuracy
 * - For medium images: Uses the fast version with optimized conversion algorithms
 * - For large images: Uses the SIMD version for maximum performance
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired: "saturate", "desaturate", "lighten", "darken", "shift_hue"
 * * `amt` - A float value from 0 to 1.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsv_auto(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsv_auto(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Optimized version of HSV color space manipulation using fast conversion algorithms.
 * This provides 1.5-2x performance improvement over the standard version.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired: "saturate", "desaturate", "lighten", "darken", "shift_hue"
 * * `amt` - A float value from 0 to 1.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsv_fast(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsv_fast(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * HSV color space manipulation using the palette library for maximum accuracy.
 * This is slower than the default `hsv()` function but provides more accurate color conversions.
 * Use this for small images where accuracy is more important than performance.
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function hsv_with_palette(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsv_with_palette(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Shift hue by a specified number of degrees in the HSL colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
 *
 * # Example
 * ```no_run
 * // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
 * use photon_rs::colour_spaces::hue_rotate_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hue_rotate_hsl(&mut img, 120_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} degrees
 */
export function hue_rotate_hsl(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsl(img.__wbg_ptr, degrees);
}

/**
 * Shift hue by a specified number of degrees in the HSLuv colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
 *
 * # Example
 * ```no_run
 * // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
 * use photon_rs::colour_spaces::hue_rotate_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hue_rotate_hsluv(&mut img, 120_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} degrees
 */
export function hue_rotate_hsluv(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsluv(img.__wbg_ptr, degrees);
}

/**
 * Shift hue by a specified number of degrees in the HSV colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
 *
 * # Example
 * ```no_run
 * // For example to hue rotate/shift the hue by 120 degrees in the HSV colour space:
 * use photon_rs::colour_spaces::hue_rotate_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hue_rotate_hsv(&mut img, 120_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} degrees
 */
export function hue_rotate_hsv(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsv(img.__wbg_ptr, degrees);
}

/**
 * Shift hue by a specified number of degrees in the LCh colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
 *
 * # Example
 * ```no_run
 * // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
 * use photon_rs::colour_spaces::hue_rotate_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hue_rotate_lch(&mut img, 120_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} degrees
 */
export function hue_rotate_lch(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_lch(img.__wbg_ptr, degrees);
}

/**
 * Apply an identity kernel convolution to an image.
 *
 * # Arguments
 * * `img` -A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply an identity kernel convolution:
 * use photon_rs::conv::identity;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * identity(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function identity(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.identity(photon_image.__wbg_ptr);
}

/**
 * Increase the brightness of an image by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `brightness` - A u8 to add to the brightness.
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::inc_brightness;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * inc_brightness(&mut img, 10_u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} brightness
 */
export function inc_brightness(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.inc_brightness(photon_image.__wbg_ptr, brightness);
}

export function init() {
    wasm.init();
}

/**
 * @param {number} num_threads
 * @returns {Promise<any>}
 */
export function initThreadPool(num_threads) {
    const ret = wasm.initThreadPool(num_threads);
    return takeObject(ret);
}

/**
 * Initialize the thread pool for parallel processing.
 *
 * This function should be called once at the beginning of your application
 * when using parallel operations in WebAssembly.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::parallel::init_parallel;
 *
 * // Initialize the thread pool (for WASM)
 * init_parallel();
 * ```
 */
export function init_parallel() {
    wasm.init_parallel();
}

/**
 * @param {number} num_threads
 * @returns {Promise<void>}
 */
export function init_thread_pool(num_threads) {
    const ret = wasm.init_thread_pool(num_threads);
    return takeObject(ret);
}

/**
 * Invert RGB value of an image.
 *
 * # Arguments
 * * `photon_image` - A DynamicImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * use photon_rs::channels::invert;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * invert(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function invert(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.invert(photon_image.__wbg_ptr);
}

/**
 * Invert all colors in an image using parallel processing.
 *
 * This is the parallel version of the invert operation.
 *
 * # Arguments
 * * `photon_image` - A mutable reference to a PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::parallel::invert_parallel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * invert_parallel(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function invert_parallel(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.invert_parallel(photon_image.__wbg_ptr);
}

/**
 * 检查默认字体是否已初始化
 * @returns {boolean}
 */
export function is_default_font_initialized() {
    const ret = wasm.is_default_font_initialized();
    return ret !== 0;
}

/**
 * Apply a standard laplace convolution.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a laplace effect:
 * use photon_rs::conv::laplace;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * laplace(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function laplace(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.laplace(photon_image.__wbg_ptr);
}

/**
 * Image manipulation effects in the LCh colour space
 *
 * Effects include:
 * * **saturate** - Saturation increase.
 * * **desaturate** - Desaturate the image.
 * * **shift_hue** - Hue rotation by a specified number of degrees.
 * * **darken** - Decrease the brightness.
 * * **lighten** - Increase the brightness.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 * # Example
 * ```no_run
 * // For example to increase the saturation by 10%:
 * use photon_rs::colour_spaces::lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lch(&mut img, "saturate", 0.1_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
export function lch(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.lch(photon_image.__wbg_ptr, ptr0, len0, amt);
}

/**
 * Lighten an image by a specified amount in the HSL colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Lightening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to lighten an image by 10% in the HSL colour space:
 * use photon_rs::colour_spaces::lighten_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lighten_hsl(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function lighten_hsl(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsl(img.__wbg_ptr, level);
}

/**
 * Lighten an image by a specified amount in the HSLuv colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Lightening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to lighten an image by 10% in the HSLuv colour space:
 * use photon_rs::colour_spaces::lighten_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lighten_hsluv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function lighten_hsluv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsluv(img.__wbg_ptr, level);
}

/**
 * Lighten an image by a specified amount in the HSV colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Lightening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to lighten an image by 10% in the HSV colour space:
 * use photon_rs::colour_spaces::lighten_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lighten_hsv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function lighten_hsv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsv(img.__wbg_ptr, level);
}

/**
 * Lighten an image by a specified amount in the LCh colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Lightening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to lighten an image by 10% in the LCh colour space:
 * use photon_rs::colour_spaces::lighten_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lighten_lch(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function lighten_lch(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_lch(img.__wbg_ptr, level);
}

/**
 * Solarization on the Red and Green channels.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::lix;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * lix(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function lix(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.lix(photon_image.__wbg_ptr);
}

/**
 * Apply a lofi effect to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::lofi;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * lofi(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function lofi(img) {
    _assertClass(img, PhotonImage);
    wasm.lofi(img.__wbg_ptr);
}

/**
 * Mix image with a single color, supporting passing `opacity`.
 * The algorithm comes from Jimp. See `function mix` and `function colorFn` at following link:
 * https://github.com/oliver-moran/jimp/blob/29679faa597228ff2f20d34c5758e4d2257065a3/packages/plugin-color/src/index.js
 * Specifically, result_value = (mix_color_value - origin_value) * opacity + origin_value =
 * mix_color_value * opacity + (1 - opacity) * origin_value for each
 * of RGB channel.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `mix_color` - the color to be mixed in, as an RGB value.
 * * `opacity` - the opacity of color when mixed to image. Float value from 0 to 1.
 * # Example
 *
 * ```no_run
 * // For example, to mix an image with rgb (50, 255, 254) and opacity 0.4:
 * use photon_rs::Rgb;
 * use photon_rs::colour_spaces::mix_with_colour;
 * use photon_rs::native::open_image;
 *
 * let mix_colour = Rgb::new(50_u8, 255_u8, 254_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * mix_with_colour(&mut img, mix_colour, 0.4_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Rgb} mix_colour
 * @param {number} opacity
 */
export function mix_with_colour(photon_image, mix_colour, opacity) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(mix_colour, Rgb);
    var ptr0 = mix_colour.__destroy_into_raw();
    wasm.mix_with_colour(photon_image.__wbg_ptr, ptr0, opacity);
}

/**
 * Apply a monochrome effect of a certain colour.
 *
 * It does so by averaging the R, G, and B values of a pixel, and then adding a
 * separate value to that averaged value for each channel to produce a tint.
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `r_offset` - The value to add to the Red channel per pixel.
 * * `g_offset` - The value to add to the Green channel per pixel.
 * * `b_offset` - The value to add to the Blue channel per pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a monochrome effect to an image:
 * use photon_rs::monochrome::monochrome;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * monochrome(&mut img, 40_u32, 50_u32, 100_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} r_offset
 * @param {number} g_offset
 * @param {number} b_offset
 */
export function monochrome(img, r_offset, g_offset, b_offset) {
    _assertClass(img, PhotonImage);
    wasm.monochrome(img.__wbg_ptr, r_offset, g_offset, b_offset);
}

/**
 * Monochrome tint effect with increased contrast
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `rgb_color` - RGB color
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::monochrome_tint;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgb;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgb_color = Rgb::new(12, 12, 10);
 * monochrome_tint(&mut img, rgb_color);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} rgb_color
 */
export function monochrome_tint(img, rgb_color) {
    _assertClass(img, PhotonImage);
    _assertClass(rgb_color, Rgb);
    var ptr0 = rgb_color.__destroy_into_raw();
    wasm.monochrome_tint(img.__wbg_ptr, ptr0);
}

/**
 * Adds multiple offsets to the image by a certain number of pixels (on two channels).
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `offset` - The offset is added to the pixels in the image.
 * # Example
 *
 * ```no_run
 * // For example, to add a 30-pixel offset to both the red and blue channels:
 * use photon_rs::effects::multiple_offsets;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * multiple_offsets(&mut img, 30_u32, 0_usize, 2_usize);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} offset
 * @param {number} channel_index
 * @param {number} channel_index2
 */
export function multiple_offsets(photon_image, offset, channel_index, channel_index2) {
    _assertClass(photon_image, PhotonImage);
    wasm.multiple_offsets(photon_image.__wbg_ptr, offset, channel_index, channel_index2);
}

/**
 * Solarization on the Blue channel.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::neue;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * neue(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function neue(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.neue(photon_image.__wbg_ptr);
}

/**
 * Noise reduction.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to noise reduct an image:
 * use photon_rs::conv::noise_reduction;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * noise_reduction(&mut img);
 * ```
 * Adds a constant to a select R, G, or B channel's value.
 * @param {PhotonImage} photon_image
 */
export function noise_reduction(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.noise_reduction(photon_image.__wbg_ptr);
}

/**
 * Noise reduction with adjustable strength.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `strength` - Noise reduction strength. Range: 0.0 to 10.0.
 *   - 0.0: No noise reduction
 *   - 1.0: Standard noise reduction (equivalent to noise_reduction())
 *   - >1.0: Stronger noise reduction (more smoothing)
 *   - <1.0: Subtle noise reduction (preserves more detail)
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply noise reduction with strength 2.0:
 * use photon_rs::conv::noise_reduction_with_strength;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * noise_reduction_with_strength(&mut img, 2.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} strength
 */
export function noise_reduction_with_strength(photon_image, strength) {
    _assertClass(photon_image, PhotonImage);
    wasm.noise_reduction_with_strength(photon_image.__wbg_ptr, strength);
}

/**
 * Normalizes an image by remapping its range of pixels values. Only RGB
 * channels are processed and each channel is stretched to \[0, 255\] range
 * independently. This process is also known as contrast stretching.
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a normalized image:
 * use photon_rs::effects::normalize;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * normalize(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function normalize(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.normalize(photon_image.__wbg_ptr);
}

/**
 * Apply a greyscale effect with increased contrast.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::obsidian;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * obsidian(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function obsidian(img) {
    _assertClass(img, PhotonImage);
    wasm.obsidian(img.__wbg_ptr);
}

/**
 * Adds an offset to the image by a certain number of pixels.
 *
 * This creates an RGB shift effect.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `channel_index`: The index of the channel to increment. 0 for red, 1 for green and 2 for blue.
 * * `offset` - The offset is added to the pixels in the image.
 * # Example
 *
 * ```no_run
 * // For example, to offset pixels by 30 pixels on the red channel:
 * use photon_rs::effects::offset;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * offset(&mut img, 0_usize, 30_u32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} channel_index
 * @param {number} offset
 */
export function offset(photon_image, channel_index, offset) {
    _assertClass(photon_image, PhotonImage);
    wasm.offset(photon_image.__wbg_ptr, channel_index, offset);
}

/**
 * Adds an offset to the blue channel by a certain number of pixels.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `offset_amt` - The offset you want to move the blue channel by.
 * # Example
 * // For example, to add an offset to the green channel by 40 pixels.
 *
 * ```no_run
 * use photon_rs::effects::offset_blue;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * offset_blue(&mut img, 40_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} offset_amt
 */
export function offset_blue(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_blue(img.__wbg_ptr, offset_amt);
}

/**
 * Adds an offset to the green channel by a certain number of pixels.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `offset` - The offset you want to move the green channel by.
 * # Example
 *
 * ```no_run
 * // For example, to add an offset to the green channel by 30 pixels.
 * use photon_rs::effects::offset_green;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * offset_green(&mut img, 30_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} offset_amt
 */
export function offset_green(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_green(img.__wbg_ptr, offset_amt);
}

/**
 * Adds an offset to the red channel by a certain number of pixels.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `offset` - The offset you want to move the red channel by.
 * # Example
 *
 * ```no_run
 * // For example, to add an offset to the red channel by 30 pixels.
 * use photon_rs::effects::offset_red;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * offset_red(&mut img, 30_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} offset_amt
 */
export function offset_red(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_red(img.__wbg_ptr, offset_amt);
}

/**
 * Turn an image into an oil painting
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `radius` - Radius of each paint particle
 * * `intesnity` - How artsy an Image should be
 * # Example
 *
 * ```no_run
 * // For example, to oil an image of type `PhotonImage`:
 * use photon_rs::effects::oil;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * oil(&mut img, 4i32, 55.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} radius
 * @param {number} intensity
 */
export function oil(photon_image, radius, intensity) {
    _assertClass(photon_image, PhotonImage);
    wasm.oil(photon_image.__wbg_ptr, radius, intensity);
}

/**
 * Convert a HTML5 Canvas Element to a PhotonImage.
 *
 * This converts the ImageData found in the canvas context to a PhotonImage,
 * which can then have effects or filters applied to it.
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @returns {PhotonImage}
 */
export function open_image(canvas, ctx) {
    const ret = wasm.open_image(addHeapObject(canvas), addHeapObject(ctx));
    return PhotonImage.__wrap(ret);
}

/**
 * Apply padding on the left side of the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels on the bottom of a PhotonImage:
 * use photon_rs::transform::padding_bottom;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_bottom(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
export function padding_bottom(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_bottom(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}

/**
 * Apply padding on the left side of the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels on the left side of a PhotonImage:
 * use photon_rs::transform::padding_left;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_left(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
export function padding_left(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_left(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}

/**
 * Apply padding on the left side of the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels on the right side of a PhotonImage:
 * use photon_rs::transform::padding_right;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_right(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
export function padding_right(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_right(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}

/**
 * Apply padding on the left side of the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels on the top of a PhotonImage:
 * use photon_rs::transform::padding_top;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_top(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
export function padding_top(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_top(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}

/**
 * Apply uniform padding around the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels around a PhotonImage:
 * use photon_rs::transform::padding_uniform;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_uniform(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
export function padding_uniform(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_uniform(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}

/**
 * Apply a rose tint to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::pastel_pink;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * pastel_pink(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function pastel_pink(img) {
    _assertClass(img, PhotonImage);
    wasm.pastel_pink(img.__wbg_ptr);
}

/**
 * Create a PhotonImage from a Uint8ClampedArray with zero-copy.
 *
 * This function creates a PhotonImage directly from JavaScript's Uint8ClampedArray
 * without copying the data, enabling efficient data transfer between JavaScript and WASM.
 *
 * # Arguments
 * * `data` - A Uint8ClampedArray containing RGBA pixel data.
 * * `width` - The image width.
 * * `height` - The image height.
 *
 * # Example
 *
 * ```javascript
 * import { PhotonImage } from 'photon_rs';
 *
 * const canvas = document.getElementById('myCanvas');
 * const ctx = canvas.getContext('2d');
 * const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
 *
 * // Create PhotonImage without copying
 * const photonImg = PhotonImage.from_uint8_array(imageData.data, canvas.width, canvas.height);
 * ```
 * @param {Uint8ClampedArray} data
 * @param {number} width
 * @param {number} height
 * @returns {PhotonImage}
 */
export function photon_image_from_uint8_clamped_array(data, width, height) {
    const ret = wasm.photon_image_from_uint8_clamped_array(addHeapObject(data), width, height);
    return PhotonImage.__wrap(ret);
}

/**
 * Get pixel data as a Uint8ClampedArray for efficient transfer to JavaScript.
 *
 * This function provides the image's pixel data as a Uint8ClampedArray,
 * which can be directly used with Canvas API without additional copying.
 *
 * # Arguments
 * * `img` - A reference to a PhotonImage.
 *
 * # Returns
 * A Uint8ClampedArray containing the RGBA pixel data.
 *
 * # Example
 *
 * ```javascript
 * import { PhotonImage } from 'photon_rs';
 *
 * const canvas = document.getElementById('myCanvas');
 * const ctx = canvas.getContext('2d');
 *
 * // After processing an image
 * const pixelData = photonImg.get_uint8_clamped_array();
 * const imageData = new ImageData(pixelData, photonImg.width, photonImg.height);
 * ctx.putImageData(imageData, 0, 0);
 * ```
 * @param {PhotonImage} img
 * @returns {Uint8ClampedArray}
 */
export function photon_image_get_uint8_clamped_array(img) {
    _assertClass(img, PhotonImage);
    const ret = wasm.photon_image_get_uint8_clamped_array(img.__wbg_ptr);
    return takeObject(ret);
}

/**
 * Add pink-tinted noise to an image.
 *
 * **[WASM SUPPORT IS AVAILABLE]**: Randomized thread pools cannot be created with WASM, but
 * a workaround using js_sys::Math::random works now.
 * # Arguments
 * * `name` - A PhotonImage that contains a view into the image.
 *
 * # Example
 *
 * ```no_run
 * // For example, to add pink-tinted noise to an image:
 * use photon_rs::native::open_image;
 * use photon_rs::noise::pink_noise;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * pink_noise(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function pink_noise(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.pink_noise(photon_image.__wbg_ptr);
}

/**
 * Pixelize an image.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `pixel_size` - Targeted pixel size of generated image.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a pixelized image with 50 pixels blocks:
 * use photon_rs::effects::pixelize;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * pixelize(&mut img, 50);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} pixel_size
 */
export function pixelize(photon_image, pixel_size) {
    _assertClass(photon_image, PhotonImage);
    wasm.pixelize(photon_image.__wbg_ptr, pixel_size);
}

/**
 * Apply a horizontal Prewitt convolution to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a horizontal Prewitt convolution effect:
 * use photon_rs::conv::prewitt_horizontal;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * prewitt_horizontal(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function prewitt_horizontal(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.prewitt_horizontal(photon_image.__wbg_ptr);
}

/**
 * Reduces an image to the primary colours.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to add a primary colour effect to an image of type `DynamicImage`:
 * use photon_rs::effects::primary;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * primary(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function primary(img) {
    _assertClass(img, PhotonImage);
    wasm.primary(img.__wbg_ptr);
}

/**
 * Process image data in-place for zero-copy operations.
 *
 * This function processes ImageData directly without creating intermediate
 * PhotonImage objects, reducing memory allocations and copying.
 *
 * # Arguments
 * * `image_data` - A mutable reference to ImageData.
 * * `processor` - A function that processes the pixel data.
 *
 * # Example
 *
 * ```javascript
 * import { process_image_data_inplace } from 'photon_rs';
 *
 * const canvas = document.getElementById('myCanvas');
 * const ctx = canvas.getContext('2d');
 * const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
 *
 * // Process image data in-place
 * process_image_data_inplace(imageData, (pixels, width, height) => {
 *     // Custom processing logic
 *     for (let i = 0; i < pixels.length; i += 4) {
 *         pixels[i] = 255 - pixels[i];     // Invert R
 *         pixels[i + 1] = 255 - pixels[i + 1]; // Invert G
 *         pixels[i + 2] = 255 - pixels[i + 2]; // Invert B
 *     }
 * });
 *
 * ctx.putImageData(imageData, 0, 0);
 * ```
 * @param {ImageData} image_data
 * @param {Function} processor
 */
export function process_image_data_inplace(image_data, processor) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.process_image_data_inplace(retptr, addBorrowedObject(image_data), addBorrowedObject(processor));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;
    }
}

/**
 * Place a PhotonImage onto a 2D canvas.
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @param {PhotonImage} new_image
 */
export function putImageData(canvas, ctx, new_image) {
    _assertClass(new_image, PhotonImage);
    var ptr0 = new_image.__destroy_into_raw();
    wasm.putImageData(addHeapObject(canvas), addHeapObject(ctx), ptr0);
}

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Red channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::monochrome::r_grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * r_grayscale(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function r_grayscale(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.r_grayscale(photon_image.__wbg_ptr);
}

/**
 * 自动边缘优化 - 对遮罩进行平滑处理（使用可分离高斯模糊优化）
 * @param {Uint8Array} mask
 * @param {number} width
 * @param {number} height
 * @param {number} smoothing_radius
 */
export function refine_mask_edges(mask, width, height, smoothing_radius) {
    var ptr0 = passArray8ToWasm0(mask, wasm.__wbindgen_export);
    var len0 = WASM_VECTOR_LEN;
    wasm.refine_mask_edges(ptr0, len0, addHeapObject(mask), width, height, smoothing_radius);
}

/**
 * Remove the Blue channel's influence in an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
 *
 * # Example
 *
 * ```no_run
 * // For example, to remove the blue channel for blue channel pixel values less than 50:
 * use photon_rs::channels::remove_blue_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * remove_blue_channel(&mut img, 50_u8);
 * ```
 * @param {PhotonImage} img
 * @param {number} min_filter
 */
export function remove_blue_channel(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_blue_channel(img.__wbg_ptr, min_filter);
}

/**
 * Set a certain channel to zero, thus removing the channel's influence in the pixels' final rendered colour.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `channel` - The channel to be removed; must be a usize from 0 to 2, with 0 representing Red, 1 representing Green, and 2 representing Blue.
 * * `min_filter` - Minimum filter. Value between 0 and 255. Only remove the channel if the current pixel's channel value is less than this minimum filter. To completely
 * remove the channel, set this value to 255, to leave the channel as is, set to 0, and to set a channel to zero for a pixel whose red value is greater than 50,
 * then channel would be 0 and min_filter would be 50.
 *
 * # Example
 *
 * ```no_run
 * // For example, to remove the Red channel with a min_filter of 100:
 * use photon_rs::channels::remove_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * remove_channel(&mut img, 0_usize, 100_u8);
 * ```
 * @param {PhotonImage} img
 * @param {number} channel
 * @param {number} min_filter
 */
export function remove_channel(img, channel, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_channel(img.__wbg_ptr, channel, min_filter);
}

/**
 * Remove the Green channel's influence in an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
 *
 * # Example
 *
 * ```no_run
 * // For example, to remove the green channel for green channel pixel values less than 50:
 * use photon_rs::channels::remove_green_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * remove_green_channel(&mut img, 50_u8);
 * ```
 * @param {PhotonImage} img
 * @param {number} min_filter
 */
export function remove_green_channel(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_green_channel(img.__wbg_ptr, min_filter);
}

/**
 * Remove the Red channel's influence in an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
 *
 * # Example
 *
 * ```no_run
 * // For example, to remove the red channel for red channel pixel values less than 50:
 * use photon_rs::channels::remove_red_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * remove_red_channel(&mut img, 50_u8);
 * ```
 * @param {PhotonImage} img
 * @param {number} min_filter
 */
export function remove_red_channel(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_red_channel(img.__wbg_ptr, min_filter);
}

/**
 * Resample the PhotonImage.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `dst_width` - Target width.
 * * `dst_height` - Target height.
 *
 * # Example
 *
 * ```no_run
 * // For example, to resample a PhotonImage to 1920x1080 size:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::resample;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let rotated_img = resample(&img, 1920, 1080);
 * ```
 * @param {PhotonImage} img
 * @param {number} dst_width
 * @param {number} dst_height
 * @returns {PhotonImage}
 */
export function resample(img, dst_width, dst_height) {
    _assertClass(img, PhotonImage);
    const ret = wasm.resample(img.__wbg_ptr, dst_width, dst_height);
    return PhotonImage.__wrap(ret);
}

/**
 * Resize an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `width` - New width.
 * * `height` - New height.
 * * `sampling_filter` - Nearest = 1, Triangle = 2, CatmullRom = 3, Gaussian = 4, Lanczos3 = 5
 * @param {PhotonImage} photon_img
 * @param {number} width
 * @param {number} height
 * @param {SamplingFilter} sampling_filter
 * @returns {PhotonImage}
 */
export function resize(photon_img, width, height, sampling_filter) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.resize(photon_img.__wbg_ptr, width, height, sampling_filter);
    return PhotonImage.__wrap(ret);
}

/**
 * Resize an image on the web.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `width` - New width.
 * * `height` - New height.
 * * `sampling_filter` - Nearest = 1, Triangle = 2, CatmullRom = 3, Gaussian = 4, Lanczos3 = 5
 * @param {PhotonImage} photon_img
 * @param {number} width
 * @param {number} height
 * @param {SamplingFilter} sampling_filter
 * @returns {HTMLCanvasElement}
 */
export function resize_img_browser(photon_img, width, height, sampling_filter) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.resize_img_browser(photon_img.__wbg_ptr, width, height, sampling_filter);
    return takeObject(ret);
}

/**
 * Rotate the PhotonImage on an arbitrary angle
 * A rotated PhotonImage is returned.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `angle` - Rotation angle in degrees.
 *
 * # Example
 *
 * ```no_run
 * // For example, to rotate a PhotonImage by 30 degrees:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::rotate;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let rotated_img = rotate(&img, 30.0);
 * ```
 * @param {PhotonImage} photon_img
 * @param {number} angle
 * @returns {PhotonImage}
 */
export function rotate(photon_img, angle) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.rotate(photon_img.__wbg_ptr, angle);
    return PhotonImage.__wrap(ret);
}

/**
 * ! [temp] Check if WASM is supported.
 */
export function run() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.run(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Solarization on the Red and Blue channels.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::ryo;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * ryo(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function ryo(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.ryo(photon_image.__wbg_ptr);
}

/**
 * Increase the image's saturation by converting each pixel's colour to the HSL colour space
 * and increasing the colour's saturation.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Increasing saturation by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to increase saturation by 10% in the HSL colour space:
 * use photon_rs::colour_spaces::saturate_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * saturate_hsl(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function saturate_hsl(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsl(img.__wbg_ptr, level);
}

/**
 * Increase the image's saturation in the HSLuv colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Increasing saturation by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to increase saturation by 40% in the HSLuv colour space:
 * use photon_rs::colour_spaces::saturate_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * saturate_hsluv(&mut img, 0.4_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function saturate_hsluv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsluv(img.__wbg_ptr, level);
}

/**
 * Increase the image's saturation in the HSV colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level by which to increase the saturation by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Increasing saturation by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to increase saturation by 30% in the HSV colour space:
 * use photon_rs::colour_spaces::saturate_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * saturate_hsv(&mut img, 0.3_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function saturate_hsv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsv(img.__wbg_ptr, level);
}

/**
 * Increase the image's saturation in the LCh colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Increasing saturation by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to increase saturation by 40% in the Lch colour space:
 * use photon_rs::colour_spaces::saturate_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * saturate_lch(&mut img, 0.4_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
export function saturate_lch(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_lch(img.__wbg_ptr, level);
}

/**
 * Resize image using seam carver.
 * Resize only if new dimensions are smaller, than original image.
 * # NOTE: This is an optimized parallel implementation with significant performance improvements.
 *
 * # Performance Improvements
 * - **Parallel Energy Computation**: Uses Rayon for multi-threaded energy map calculation
 * - **Batch Seam Removal**: Processes multiple seams in a single pass (up to 8 at a time)
 * - **Reduced Memory Allocations**: Pre-allocates all necessary memory upfront
 * - **Optimized Dynamic Programming**: Efficient min-path computation with SIMD-friendly patterns
 * - **Smart Rotation Strategy**: Minimizes rotation operations for horizontal seams
 *
 * # Expected Performance
 * - Native (Rayon): 2-4x faster than sequential version on multi-core systems
 * - WASM (wasm-bindgen-rayon): 1.5-2.5x faster on browsers with thread support
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `width` - New width.
 * * `height` - New height.
 *
 * # Example
 *
 * ```no_run
 * // For example, resize image using seam carver:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::seam_carve;
 * use photon_rs::PhotonImage;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let result: PhotonImage = seam_carve(&img, 100_u32, 100_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} width
 * @param {number} height
 * @returns {PhotonImage}
 */
export function seam_carve(img, width, height) {
    _assertClass(img, PhotonImage);
    const ret = wasm.seam_carve(img.__wbg_ptr, width, height);
    return PhotonImage.__wrap(ret);
}

/**
 * Selectively change pixel colours which are similar to the reference colour provided.
 *
 * Similarity between two colours is calculated via the CIE76 formula.
 * Only changes the color of a pixel if its similarity to the reference colour is within the range in the algorithm.
 * For example, with this function, a user can change the color of all blue pixels by mixing them with red by 10%.
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `new_color` - The `RGB` value of the new color (to be mixed with the matched pixels)
 * * `fraction` - The amount of mixing the new colour with the matched pixels
 *
 * # Example
 *
 * ```no_run
 * // For example, to only change the color of pixels that are similar to the RGB value RGB{200, 120, 30} by mixing RGB{30, 120, 200} with 25%:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_color_convert;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(200, 120, 30);
 * let new_color = Rgb::new(30, 120, 200);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_color_convert(&mut img, ref_color, new_color, 0.25);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Rgb} ref_color
 * @param {Rgb} new_color
 * @param {number} fraction
 */
export function selective_color_convert(photon_image, ref_color, new_color, fraction) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    _assertClass(new_color, Rgb);
    var ptr1 = new_color.__destroy_into_raw();
    wasm.selective_color_convert(photon_image.__wbg_ptr, ptr0, ptr1, fraction);
}

/**
 * Selectively desaturate pixel colours which are similar to the reference colour provided.
 *
 * Similarity between two colours is calculated via the CIE76 formula.
 * Only desaturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm.
 * For example, if a user wishes all pixels that are blue to be desaturated by 0.1, they can selectively specify  only the blue pixels to be changed.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `amt` - The amount of desaturate the colour by.
 *
 * # Example
 *
 * ```no_run
 * // For example, to only desaturate the pixels that are similar to the RGB value RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_desaturate;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_desaturate(&mut img, ref_color, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} ref_color
 * @param {number} amt
 */
export function selective_desaturate(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_desaturate(img.__wbg_ptr, ptr0, amt);
}

/**
 * Selectively changes a pixel to greyscale if it is *not* visually similar or close to the colour specified.
 * Only changes the colour of a pixel if its RGB values are within a specified range.
 *
 * (Similarity between two colours is calculated via the CIE76 formula.)
 * For example, if a user wishes all pixels that are *NOT* blue to be displayed in greyscale, they can selectively specify only the blue pixels to be
 * kept in the photo.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 *
 * # Example
 *
 * ```no_run
 * // For example, to greyscale all pixels that are *not* visually similar to the RGB colour RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_greyscale;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_greyscale(img, ref_color);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Rgb} ref_color
 */
export function selective_greyscale(photon_image, ref_color) {
    _assertClass(photon_image, PhotonImage);
    var ptr0 = photon_image.__destroy_into_raw();
    _assertClass(ref_color, Rgb);
    var ptr1 = ref_color.__destroy_into_raw();
    wasm.selective_greyscale(ptr0, ptr1);
}

/**
 * Selective hue rotation.
 *
 * Only rotate the hue of a pixel if its RGB values are within a specified range.
 * This function only rotates a pixel's hue to another  if it is visually similar to the colour specified.
 * For example, if a user wishes all pixels that are blue to be changed to red, they can selectively specify  only the blue pixels to be changed.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `degrees` - The amount of degrees to hue rotate by.
 *
 * # Example
 *
 * ```no_run
 * // For example, to only rotate the pixels that are of RGB value RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_hue_rotate;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_hue_rotate(&mut img, ref_color, 180_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Rgb} ref_color
 * @param {number} degrees
 */
export function selective_hue_rotate(photon_image, ref_color, degrees) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_hue_rotate(photon_image.__wbg_ptr, ptr0, degrees);
}

/**
 * Selectively lighten an image.
 *
 * Only lighten the hue of a pixel if its colour matches or is similar to the RGB colour specified.
 * For example, if a user wishes all pixels that are blue to be lightened, they can selectively specify  only the blue pixels to be changed.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `amt` - The level from 0 to 1 to lighten the hue by. Increasing by 10% would have an `amt` of 0.1
 *
 * # Example
 *
 * ```no_run
 * // For example, to only lighten the pixels that are of or similar to RGB value RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_lighten;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_lighten(&mut img, ref_color, 0.2_f32);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} ref_color
 * @param {number} amt
 */
export function selective_lighten(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_lighten(img.__wbg_ptr, ptr0, amt);
}

/**
 * Selectively saturate pixel colours which are similar to the reference colour provided.
 *
 * Similarity between two colours is calculated via the CIE76 formula.
 * Only saturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm.
 * For example, if a user wishes all pixels that are blue to have an increase in saturation by 10%, they can selectively specify only the blue pixels to be changed.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `amt` - The amount of saturate the colour by.
 *
 * # Example
 *
 * ```no_run
 * // For example, to only increase the saturation of pixels that are similar to the RGB value RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_saturate;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_saturate(&mut img, ref_color, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} ref_color
 * @param {number} amt
 */
export function selective_saturate(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_saturate(img.__wbg_ptr, ptr0, amt);
}

/**
 * Convert an image to sepia.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * // For example, to sepia an image of type `PhotonImage`:
 * use photon_rs::monochrome::sepia;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sepia(&mut img);
 * ```
 * @param {PhotonImage} img
 */
export function sepia(img) {
    _assertClass(img, PhotonImage);
    wasm.sepia(img.__wbg_ptr);
}

/**
 * Sharpen an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to sharpen an image:
 * use photon_rs::conv::sharpen;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sharpen(&mut img);
 * ```
 * Adds a constant to a select R, G, or B channel's value.
 * @param {PhotonImage} photon_image
 */
export function sharpen(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sharpen(photon_image.__wbg_ptr);
}

/**
 * Sharpen an image with adjustable strength.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `strength` - Sharpening strength. Range: 0.0 to 10.0.
 *   - 0.0: No sharpening effect
 *   - 1.0: Standard sharpening (equivalent to sharpen())
 *   - >1.0: Stronger sharpening
 *   - <1.0: Subtle sharpening
 *
 * # Example
 *
 * ```no_run
 * // For example, to sharpen an image with strength 2.0:
 * use photon_rs::conv::sharpen_with_strength;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sharpen_with_strength(&mut img, 2.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} strength
 */
export function sharpen_with_strength(photon_image, strength) {
    _assertClass(photon_image, PhotonImage);
    wasm.sharpen_with_strength(photon_image.__wbg_ptr, strength);
}

/**
 * Shear the image along the X axis.
 * A sheared PhotonImage is returned.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `shear` - Amount to shear.
 *
 * # Example
 *
 * ```no_run
 * // For example, to shear an image by 0.5:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::shearx;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let sheared_img = shearx(&img, 0.5);
 * ```
 * @param {PhotonImage} photon_img
 * @param {number} shear
 * @returns {PhotonImage}
 */
export function shearx(photon_img, shear) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.shearx(photon_img.__wbg_ptr, shear);
    return PhotonImage.__wrap(ret);
}

/**
 * Shear the image along the Y axis.
 * A sheared PhotonImage is returned.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `shear` - Amount to shear.
 *
 * # Example
 *
 * ```no_run
 * // For example, to shear an image by 0.5:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::sheary;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let sheared_img = sheary(&img, 0.5);
 * ```
 * @param {PhotonImage} photon_img
 * @param {number} shear
 * @returns {PhotonImage}
 */
export function sheary(photon_img, shear) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.sheary(photon_img.__wbg_ptr, shear);
    return PhotonImage.__wrap(ret);
}

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to a chosen channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `channel` - A usize representing the channel from 0 to 2. O represents the Red channel, 1 the Green channel, and 2 the Blue channel.
 *
 * # Example
 * To grayscale using only values from the Red channel:
 * ```no_run
 * use photon_rs::monochrome::single_channel_grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * single_channel_grayscale(&mut img, 0_usize);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} channel
 */
export function single_channel_grayscale(photon_image, channel) {
    _assertClass(photon_image, PhotonImage);
    wasm.single_channel_grayscale(photon_image.__wbg_ptr, channel);
}

/**
 * Apply a global Sobel filter to an image
 *
 * Each pixel is calculated as the magnitude of the horizontal and vertical components of the Sobel filter,
 * ie if X is the horizontal sobel and Y is the vertical, for each pixel, we calculate sqrt(X^2 + Y^2)
 *
 * This optimized version calculates both horizontal and vertical gradients in a single pass,
 * avoiding image cloning and reducing memory usage by 50%.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a global Sobel filter:
 * use photon_rs::conv::sobel_global;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sobel_global(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function sobel_global(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_global(photon_image.__wbg_ptr);
}

/**
 * Apply a horizontal Sobel filter to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a horizontal Sobel filter:
 * use photon_rs::conv::sobel_horizontal;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sobel_horizontal(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function sobel_horizontal(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_horizontal(photon_image.__wbg_ptr);
}

/**
 * Apply a vertical Sobel filter to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a vertical Sobel filter:
 * use photon_rs::conv::sobel_vertical;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sobel_vertical(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function sobel_vertical(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_vertical(photon_image.__wbg_ptr);
}

/**
 * Applies a solarizing effect to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to colorize an image of type `PhotonImage`:
 * use photon_rs::effects::solarize;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * solarize(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
export function solarize(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.solarize(photon_image.__wbg_ptr);
}

/**
 * Applies a solarizing effect to an image and returns the resulting PhotonImage.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to solarize "retimg" an image of type `PhotonImage`:
 * use photon_rs::effects::solarize_retimg;
 * use photon_rs::native::open_image;
 * use photon_rs::PhotonImage;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let result: PhotonImage = solarize_retimg(&img);
 * ```
 * @param {PhotonImage} photon_image
 * @returns {PhotonImage}
 */
export function solarize_retimg(photon_image) {
    _assertClass(photon_image, PhotonImage);
    const ret = wasm.solarize_retimg(photon_image.__wbg_ptr);
    return PhotonImage.__wrap(ret);
}

/**
 * Swap two channels.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `channel1` - An index from 0 to 2, representing the Red, Green or Blue channels respectively. Red would be represented by 0, Green by 1, and Blue by 2.
 * * `channel2` - An index from 0 to 2, representing the Red, Green or Blue channels respectively. Same as above.
 *
 * # Example
 *
 * ```no_run
 * // For example, to swap the values of the Red channel with the values of the Blue channel:
 * use photon_rs::channels::swap_channels;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * swap_channels(&mut img, 0_usize, 2_usize);
 * ```
 * @param {PhotonImage} img
 * @param {number} channel1
 * @param {number} channel2
 */
export function swap_channels(img, channel1, channel2) {
    _assertClass(img, PhotonImage);
    wasm.swap_channels(img.__wbg_ptr, channel1, channel2);
}

/**
 * Threshold an image using a standard thresholding algorithm.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `threshold` - The amount the image should be thresholded by from 0 to 255.
 * # Example
 *
 * ```no_run
 * // For example, to threshold an image of type `PhotonImage`:
 * use photon_rs::monochrome::threshold;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * threshold(&mut img, 30_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} threshold
 */
export function threshold(img, threshold) {
    _assertClass(img, PhotonImage);
    wasm.threshold(img.__wbg_ptr, threshold);
}

/**
 * Apply threshold operation using parallel processing.
 *
 * This is the parallel version of the threshold operation.
 * Pixels above threshold become white (255), below become black (0).
 *
 * # Arguments
 * * `photon_image` - A mutable reference to a PhotonImage.
 * * `threshold` - The threshold value (0-255).
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::parallel::threshold_parallel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * threshold_parallel(&mut img, 128);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} threshold
 */
export function threshold_parallel(photon_image, threshold) {
    _assertClass(photon_image, PhotonImage);
    wasm.threshold_parallel(photon_image.__wbg_ptr, threshold);
}

/**
 * Tint an image by adding an offset to averaged RGB channel values.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `r_offset` - The amount the R channel should be incremented by.
 * * `g_offset` - The amount the G channel should be incremented by.
 * * `b_offset` - The amount the B channel should be incremented by.
 * # Example
 *
 * ```no_run
 * // For example, to tint an image of type `PhotonImage`:
 * use photon_rs::effects::tint;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * tint(&mut img, 10_u32, 20_u32, 15_u32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} r_offset
 * @param {number} g_offset
 * @param {number} b_offset
 */
export function tint(photon_image, r_offset, g_offset, b_offset) {
    _assertClass(photon_image, PhotonImage);
    wasm.tint(photon_image.__wbg_ptr, r_offset, g_offset, b_offset);
}

/**
 * Convert a PhotonImage to JS-compatible ImageData.
 * @param {PhotonImage} photon_image
 * @returns {ImageData}
 */
export function to_image_data(photon_image) {
    _assertClass(photon_image, PhotonImage);
    var ptr0 = photon_image.__destroy_into_raw();
    const ret = wasm.to_image_data(ptr0);
    return takeObject(ret);
}

/**
 * Convert ImageData to a raw pixel vec of u8s.
 * @param {ImageData} imgdata
 * @returns {Uint8Array}
 */
export function to_raw_pixels(imgdata) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.to_raw_pixels(retptr, addHeapObject(imgdata));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 1, 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `num_strips` - The numbder of strips
 * # Example
 *
 * ```no_run
 * // For example, to draw vertical strips on a `PhotonImage`:
 * use photon_rs::effects::vertical_strips;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * vertical_strips(&mut img, 8u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_strips
 */
export function vertical_strips(photon_image, num_strips) {
    _assertClass(photon_image, PhotonImage);
    wasm.vertical_strips(photon_image.__wbg_ptr, num_strips);
}

/**
 * 在 WASM 环境中清空所有已注册的字体
 */
export function wasm_clear_fonts() {
    wasm.wasm_clear_fonts();
}

/**
 * 在 WASM 环境中获取已注册字体列表
 * @returns {string[]}
 */
export function wasm_get_registered_fonts() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.wasm_get_registered_fonts(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * 在 WASM 环境中检查字体是否已注册
 * @param {string} font_name
 * @returns {boolean}
 */
export function wasm_is_font_registered(font_name) {
    const ptr0 = passStringToWasm0(font_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_is_font_registered(ptr0, len0);
    return ret !== 0;
}

/**
 * 在 WASM 环境中注册字体
 *
 * # 参数
 * * `font_name` - 字体名称，用于后续引用
 * * `font_data` - 字体文件的二进制数据（Uint8Array）
 * @param {string} font_name
 * @param {Uint8Array} font_data
 */
export function wasm_register_font(font_name, font_data) {
    const ptr0 = passStringToWasm0(font_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(font_data, wasm.__wbindgen_export);
    const len1 = WASM_VECTOR_LEN;
    wasm.wasm_register_font(ptr0, len0, ptr1, len1);
}

/**
 * 在 WASM 环境中移除已注册的字体
 * @param {string} font_name
 * @returns {boolean}
 */
export function wasm_unregister_font(font_name) {
    const ptr0 = passStringToWasm0(font_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_unregister_font(ptr0, len0);
    return ret !== 0;
}

/**
 * Add a watermark to an image.
 *
 * # Arguments
 * * `img` - A DynamicImage that contains a view into the image.
 * * `watermark` - The watermark to be placed onto the `img` image.
 * * `x` - The x coordinate where the watermark's top corner should be positioned.
 * * `y` - The y coordinate where the watermark's top corner should be positioned.
 * # Example
 *
 * ```no_run
 * // For example, to add a watermark to an image at x: 30, y: 40:
 * use photon_rs::multiple::watermark;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let water_mark = open_image("watermark.jpg").expect("File should open");
 * watermark(&mut img, &water_mark, 30_i64, 40_i64);
 * ```
 * @param {PhotonImage} img
 * @param {PhotonImage} watermark
 * @param {bigint} x
 * @param {bigint} y
 */
export function watermark(img, watermark, x, y) {
    _assertClass(img, PhotonImage);
    _assertClass(watermark, PhotonImage);
    wasm.watermark(img.__wbg_ptr, watermark.__wbg_ptr, x, y);
}

/**
 * Adaptive watermark function that automatically selects the optimal algorithm
 * based on image size.
 *
 * - For small images: Uses the standard watermark function
 * - For medium/large images: Uses the fast version that works directly on raw pixels
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `watermark` - The watermark to be placed onto the `img` image.
 * * `x` - The x coordinate where the watermark's top corner should be positioned.
 * * `y` - The y coordinate where the watermark's top corner should be positioned.
 * @param {PhotonImage} img
 * @param {PhotonImage} watermark
 * @param {bigint} x
 * @param {bigint} y
 */
export function watermark_adaptive(img, watermark, x, y) {
    _assertClass(img, PhotonImage);
    _assertClass(watermark, PhotonImage);
    wasm.watermark_adaptive(img.__wbg_ptr, watermark.__wbg_ptr, x, y);
}

/**
 * Optimized version of watermark function that works directly on raw pixel data.
 * Avoids creating DynamicImage objects multiple times.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `watermark` - The watermark to be placed onto the `img` image.
 * * `x` - The x coordinate where the watermark's top corner should be positioned.
 * * `y` - The y coordinate where the watermark's top corner should be positioned.
 * @param {PhotonImage} img
 * @param {PhotonImage} watermark
 * @param {bigint} x
 * @param {bigint} y
 */
export function watermark_fast(img, watermark, x, y) {
    _assertClass(img, PhotonImage);
    _assertClass(watermark, PhotonImage);
    wasm.watermark_fast(img.__wbg_ptr, watermark.__wbg_ptr, x, y);
}

export class wbg_rayon_PoolBuilder {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(wbg_rayon_PoolBuilder.prototype);
        obj.__wbg_ptr = ptr;
        wbg_rayon_PoolBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        wbg_rayon_PoolBuilderFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wbg_rayon_poolbuilder_free(ptr, 0);
    }
    build() {
        wasm.wbg_rayon_poolbuilder_build(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    numThreads() {
        const ret = wasm.wbg_rayon_poolbuilder_numThreads(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    receiver() {
        const ret = wasm.wbg_rayon_poolbuilder_receiver(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) wbg_rayon_PoolBuilder.prototype[Symbol.dispose] = wbg_rayon_PoolBuilder.prototype.free;

/**
 * @param {number} receiver
 */
export function wbg_rayon_start_worker(receiver) {
    wasm.wbg_rayon_start_worker(receiver);
}

function __wbg_get_imports(memory) {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_copy_to_typed_array_5294f8e46aecc086: function(arg0, arg1, arg2) {
            new Uint8Array(getObject(arg2).buffer, getObject(arg2).byteOffset, getObject(arg2).byteLength).set(getArrayU8FromWasm0(arg0, arg1));
        },
        __wbg___wbindgen_debug_string_ddde1867f49c2442: function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_d633e708baf0d146: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_object_4b3de556756ee8a8: function(arg0) {
            const val = getObject(arg0);
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_7debe47dc1e045c2: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_c18285b9fc34cb7d: function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        },
        __wbg___wbindgen_memory_f1258f0b3cab52b2: function() {
            const ret = wasm.memory;
            return addHeapObject(ret);
        },
        __wbg___wbindgen_module_39ff3d28752148a9: function() {
            const ret = wasmModule;
            return addHeapObject(ret);
        },
        __wbg___wbindgen_number_get_5854912275df1894: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_rethrow_0803fa3da1b498f1: function(arg0) {
            throw takeObject(arg0);
        },
        __wbg___wbindgen_throw_39bc967c0e5a9b58: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_b6d832240a919168: function(arg0) {
            getObject(arg0)._wbg_cb_unref();
        },
        __wbg_appendChild_f8784f6270d097cd: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_async_d823d36f294f15c4: function(arg0) {
            const ret = getObject(arg0).async;
            return ret;
        },
        __wbg_body_4eb4906314b12ac0: function(arg0) {
            const ret = getObject(arg0).body;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_buffer_0501472a2adb62a1: function(arg0) {
            const ret = getObject(arg0).buffer;
            return addHeapObject(ret);
        },
        __wbg_call_08ad0d89caa7cb79: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_call_c974f0bf2231552e: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).call(getObject(arg1), getObject(arg2), getObject(arg3));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_color_new: function(arg0) {
            const ret = Color.__wrap(arg0);
            return addHeapObject(ret);
        },
        __wbg_createElement_c28be812ac2ffe84: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_crypto_38df2bab126b63dc: function(arg0) {
            const ret = getObject(arg0).crypto;
            return addHeapObject(ret);
        },
        __wbg_data_826b7d645a40043f: function(arg0) {
            const ret = getObject(arg0).data;
            return addHeapObject(ret);
        },
        __wbg_data_a8576aa36473ad45: function(arg0, arg1) {
            const ret = getObject(arg1).data;
            const ptr1 = passArray8ToWasm0(ret, wasm.__wbindgen_export);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_document_0b7613236d782ccc: function(arg0) {
            const ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_drawImage_3f68cb34a5e3700b: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            getObject(arg0).drawImage(getObject(arg1), arg2, arg3);
        }, arguments); },
        __wbg_drawImage_6f33340e37778efb: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
            getObject(arg0).drawImage(getObject(arg1), arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
        }, arguments); },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_export4(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_error_ad28debb48b5c6bb: function(arg0) {
            console.error(getObject(arg0));
        },
        __wbg_getContext_04fd91bf79400077: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments); },
        __wbg_getImageData_ad1f4b73ac9d853c: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = getObject(arg0).getImageData(arg1, arg2, arg3, arg4);
            return addHeapObject(ret);
        }, arguments); },
        __wbg_getRandomValues_c44a50d8cfdaebeb: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).getRandomValues(getObject(arg1));
        }, arguments); },
        __wbg_get_18349afdb36339a9: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_get_f09c3a16f8848381: function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return addHeapObject(ret);
        },
        __wbg_get_index_36f92638a16561e8: function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return ret;
        },
        __wbg_height_21ecb9dcc0472f5d: function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        },
        __wbg_height_a2a793f8a2363a46: function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        },
        __wbg_height_a904b80afa6e2a37: function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        },
        __wbg_instanceof_Array_461ec6d7afd45fda: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_CanvasRenderingContext2d_125f869ccf2f7649: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof CanvasRenderingContext2D;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Float32Array_00770a0487b98c06: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Float32Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlCanvasElement_d8fa699a8663ca1b: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLCanvasElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Object_813a194d6e249bee: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Object;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_4aba49e4d1a12365: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_length_20b2161ab2bea256: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_326999dcd07f2163: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_5855c1f289dfffc1: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_a31e05262e09b7f8: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_msCrypto_bd5a034af96bcba6: function(arg0) {
            const ret = getObject(arg0).msCrypto;
            return addHeapObject(ret);
        },
        __wbg_new_09959f7b4c92c246: function(arg0) {
            const ret = new Uint8Array(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return addHeapObject(ret);
        },
        __wbg_new_5249bab2d955c841: function(arg0) {
            const ret = new Int32Array(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_new_6eed8f87fc95618e: function() { return handleError(function (arg0, arg1) {
            const ret = new Worker(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_79ce7968119cfd96: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return __wasm_bindgen_func_elem_2314(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return addHeapObject(ret);
            } finally {
                state0.a = state0.b = 0;
            }
        },
        __wbg_new_cbee8c0d5c479eac: function() {
            const ret = new Array();
            return addHeapObject(ret);
        },
        __wbg_new_ed69e637b553a997: function() {
            const ret = new Object();
            return addHeapObject(ret);
        },
        __wbg_new_from_slice_d7e202fdbee3c396: function(arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg_new_typed_8258a0d8488ef2a2: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return __wasm_bindgen_func_elem_2314(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return addHeapObject(ret);
            } finally {
                state0.a = state0.b = 0;
            }
        },
        __wbg_new_with_length_c8449d782396d344: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return addHeapObject(ret);
        },
        __wbg_new_with_u8_clamped_array_and_sh_b673725621b5c7e7: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
            return addHeapObject(ret);
        }, arguments); },
        __wbg_node_84ea875411254db1: function(arg0) {
            const ret = getObject(arg0).node;
            return addHeapObject(ret);
        },
        __wbg_now_edd718b3004d8631: function() {
            const ret = Date.now();
            return ret;
        },
        __wbg_of_332bf1b25b068982: function(arg0, arg1, arg2) {
            const ret = Array.of(getObject(arg0), getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        },
        __wbg_postMessage_af6209ddad5840b9: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).postMessage(getObject(arg1));
        }, arguments); },
        __wbg_process_44c7a14e11e9f69e: function(arg0) {
            const ret = getObject(arg0).process;
            return addHeapObject(ret);
        },
        __wbg_prototypesetcall_e02cc4f04479d253: function(arg0, arg1, arg2) {
            Uint8ClampedArray.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
        },
        __wbg_prototypesetcall_f034d444741426c3: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
        },
        __wbg_push_a6f9488ffd3fae3b: function(arg0, arg1) {
            const ret = getObject(arg0).push(getObject(arg1));
            return ret;
        },
        __wbg_putImageData_afec9ab1493ac23a: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            getObject(arg0).putImageData(getObject(arg1), arg2, arg3);
        }, arguments); },
        __wbg_queueMicrotask_2c8dfd1056f24fdc: function(arg0) {
            const ret = getObject(arg0).queueMicrotask;
            return addHeapObject(ret);
        },
        __wbg_queueMicrotask_8985ad63815852e7: function(arg0) {
            queueMicrotask(getObject(arg0));
        },
        __wbg_randomFillSync_6c25eac9869eb53c: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).randomFillSync(takeObject(arg1));
        }, arguments); },
        __wbg_random_2b7bed8995d680fb: function() {
            const ret = Math.random();
            return ret;
        },
        __wbg_require_b4edbdcf3e2a1ef0: function() { return handleError(function () {
            const ret = module.require;
            return addHeapObject(ret);
        }, arguments); },
        __wbg_resolve_5d61e0d10c14730a: function(arg0) {
            const ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_set_bad5c505cc70b5f8: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
            return ret;
        }, arguments); },
        __wbg_set_height_ed13c7b896d93a3b: function(arg0, arg1) {
            getObject(arg0).height = arg1 >>> 0;
        },
        __wbg_set_onmessage_a073f657459fcfe6: function(arg0, arg1) {
            getObject(arg0).onmessage = getObject(arg1);
        },
        __wbg_set_textContent_ccd33eab05add227: function(arg0, arg1, arg2) {
            getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_width_7f65ced2ffeee343: function(arg0, arg1) {
            getObject(arg0).width = arg1 >>> 0;
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = getObject(arg1).stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_startWorkers_8b582d57e92bd2d4: function(arg0, arg1, arg2) {
            const ret = startWorkers(takeObject(arg0), takeObject(arg1), wbg_rayon_PoolBuilder.__wrap(arg2));
            return addHeapObject(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_14325d8cca34bb77: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_GLOBAL_f3a1e69f9c5a7e8e: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_SELF_50cdb5b517789aca: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_WINDOW_d6c4126e4c244380: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_subarray_7ad5f01d4a9c1c4d: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
            return addHeapObject(ret);
        },
        __wbg_then_6e88c9d5b003f517: function(arg0, arg1) {
            const ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        },
        __wbg_then_d4163530723f56f4: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        },
        __wbg_then_f1c954fe00733701: function(arg0, arg1) {
            const ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        },
        __wbg_value_ad1c1726993ce63e: function(arg0) {
            const ret = getObject(arg0).value;
            return addHeapObject(ret);
        },
        __wbg_versions_276b2795b1c6a219: function(arg0) {
            const ret = getObject(arg0).versions;
            return addHeapObject(ret);
        },
        __wbg_waitAsync_207a52eee200ef0a: function(arg0, arg1, arg2) {
            const ret = Atomics.waitAsync(getObject(arg0), arg1 >>> 0, arg2);
            return addHeapObject(ret);
        },
        __wbg_waitAsync_f6c74926be2d0dac: function() {
            const ret = Atomics.waitAsync;
            return addHeapObject(ret);
        },
        __wbg_width_60f44a816d7f9267: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbg_width_7e8257ca6cbf875f: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbg_width_b56e4eeade1cc8f6: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 240, function: Function { arguments: [Externref], shim_idx: 241, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_907, __wasm_bindgen_func_elem_908);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 393, function: Function { arguments: [Externref], shim_idx: 396, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_1686, __wasm_bindgen_func_elem_1689);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 393, function: Function { arguments: [NamedExternref("MessageEvent")], shim_idx: 394, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_1686, __wasm_bindgen_func_elem_1687);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000004: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000005: function(arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000006: function(arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8ClampedArray")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000007: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000008: function(arg0, arg1) {
            var v0 = getClampedArrayU8FromWasm0(arg0, arg1).slice();
            wasm.__wbindgen_export4(arg0, arg1 * 1, 1);
            // Cast intrinsic for `Vector(ClampedU8) -> Externref`.
            const ret = v0;
            return addHeapObject(ret);
        },
        __wbindgen_link_922dd8fcb05d94cd: function(arg0) {
            const val = `onmessage = function (ev) {
                let [ia, index, value] = ev.data;
                ia = new Int32Array(ia.buffer);
                let result = Atomics.wait(ia, index, value);
                postMessage(result);
            };
            `;
            const ret = typeof URL.createObjectURL === 'undefined' ? "data:application/javascript," + encodeURIComponent(val) : URL.createObjectURL(new Blob([val], { type: "text/javascript" }));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
        memory: memory || new WebAssembly.Memory({initial:21,maximum:16384,shared:true}),
    };
    return {
        __proto__: null,
        "./photon_wasm_bg.js": import0,
    };
}

function __wasm_bindgen_func_elem_908(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_908(arg0, arg1, addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_1687(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_1687(arg0, arg1, addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_1689(arg0, arg1, arg2) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.__wasm_bindgen_func_elem_1689(retptr, arg0, arg1, addHeapObject(arg2));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function __wasm_bindgen_func_elem_2314(arg0, arg1, arg2, arg3) {
    wasm.__wasm_bindgen_func_elem_2314(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

const BrushConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_brushconfig_free(ptr >>> 0, 1));
const BrushStrokeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_brushstroke_free(ptr >>> 0, 1));
const ColorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_color_free(ptr >>> 0, 1));
const ImageProcessorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_imageprocessor_free(ptr >>> 0, 1));
const PhotonImageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_photonimage_free(ptr >>> 0, 1));
const RgbFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rgb_free(ptr >>> 0, 1));
const RgbaFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rgba_free(ptr >>> 0, 1));
const StrokePointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_strokepoint_free(ptr >>> 0, 1));
const wbg_rayon_PoolBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wbg_rayon_poolbuilder_free(ptr >>> 0, 1));

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function dropObject(idx) {
    if (idx < 1028) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.buffer !== wasm.memory.buffer) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.buffer !== wasm.memory.buffer) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedUint8ClampedArrayMemory0 = null;
function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.buffer !== wasm.memory.buffer) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export3(addHeapObject(e));
    }
}

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let stack_pointer = 1024;

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : undefined);
if (cachedTextDecoder) cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().slice(ptr, ptr + len));
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder() : undefined);

if (cachedTextEncoder) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module, thread_stack_size) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    cachedUint8ClampedArrayMemory0 = null;
    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) {
        throw new Error('invalid stack size');
    }

    wasm.__wbindgen_start(thread_stack_size);
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module, memory) {
    if (wasm !== undefined) return wasm;

    let thread_stack_size
    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module, memory, thread_stack_size} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports(memory);
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module, thread_stack_size);
}

async function __wbg_init(module_or_path, memory) {
    if (wasm !== undefined) return wasm;

    let thread_stack_size
    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path, memory, thread_stack_size} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('photon_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports(memory);

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module, thread_stack_size);
}

export { initSync, __wbg_init as default };
