#[cfg(test)]
mod tests {
    use super::super::{Layer, LayerStack, BlendMode};

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new(1, "Test Layer".to_string(), 100, 100);
        assert_eq!(layer.id(), 1);
        assert_eq!(layer.name(), "Test Layer");
        assert_eq!(layer.width(), 100);
        assert_eq!(layer.height(), 100);
        assert!(layer.visible());
        assert!(!layer.locked());
        assert_eq!(layer.opacity(), 255);
        assert_eq!(layer.blend_mode(), BlendMode::Normal);
    }

    #[test]
    fn test_layer_properties() {
        let mut layer = Layer::new(1, "Test".to_string(), 100, 100);

        layer.set_visible(false);
        assert!(!layer.visible());

        layer.set_locked(true);
        assert!(layer.locked());

        layer.set_opacity(128);
        assert_eq!(layer.opacity(), 128);

        layer.set_blend_mode(BlendMode::Multiply);
        assert_eq!(layer.blend_mode(), BlendMode::Multiply);

        layer.set_name("Renamed".to_string());
        assert_eq!(layer.name(), "Renamed");
    }

    #[test]
    fn test_layer_stack_creation() {
        let stack = LayerStack::new(800, 600);
        assert_eq!(stack.canvas_width(), 800);
        assert_eq!(stack.canvas_height(), 600);
        assert_eq!(stack.layer_count(), 0);
    }

    #[test]
    fn test_add_layer() {
        let mut stack = LayerStack::new(100, 100);
        let id = stack.add_layer(Some("Layer 1".to_string()));
        assert_eq!(stack.layer_count(), 1);
        assert_eq!(id, 1);

        let layer = stack.get_layer(id);
        assert!(layer.is_some());
        assert_eq!(layer.unwrap().name(), "Layer 1");
    }

    #[test]
    fn test_add_layer_auto_name() {
        let mut stack = LayerStack::new(100, 100);
        let id = stack.add_layer(None);
        assert_eq!(stack.layer_count(), 1);

        let layer = stack.get_layer(id);
        assert!(layer.is_some());
        assert_eq!(layer.unwrap().name(), "Layer 1");
    }

    #[test]
    fn test_add_multiple_layers() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(None);
        stack.add_layer(None);
        stack.add_layer(None);

        assert_eq!(stack.layer_count(), 3);
        let ids = stack.get_layer_ids();
        assert_eq!(ids.len(), 3);
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_layer() {
        let mut stack = LayerStack::new(100, 100);
        let id = stack.add_layer(None);

        let removed = stack.remove_layer(id);
        assert!(removed);
        assert_eq!(stack.layer_count(), 0);

        // 删除不存在的图层
        let removed = stack.remove_layer(999);
        assert!(!removed);
    }

    #[test]
    fn test_remove_layer_by_index() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(None);
        stack.add_layer(None);
        stack.add_layer(None);

        let removed = stack.remove_layer_by_index(1);
        assert!(removed);
        assert_eq!(stack.layer_count(), 2);

        // 删除无效索引
        let removed = stack.remove_layer_by_index(10);
        assert!(!removed);
    }

    #[test]
    fn test_move_layer() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(None); // id=1, index=0
        stack.add_layer(None); // id=2, index=1
        stack.add_layer(None); // id=3, index=2

        // 将图层0移到末尾
        let moved = stack.move_layer(0, 2);
        assert!(moved);

        let ids = stack.get_layer_ids();
        assert_eq!(ids, vec![2, 3, 1]);
    }

    #[test]
    fn test_duplicate_layer() {
        let mut stack = LayerStack::new(100, 100);
        let id = stack.add_layer(Some("Original".to_string()));

        let new_id = stack.duplicate_layer(id);
        assert!(new_id.is_some());
        assert_eq!(stack.layer_count(), 2);

        let new_layer = stack.get_layer(new_id.unwrap());
        assert!(new_layer.is_some());
        assert_eq!(new_layer.unwrap().name(), "Original Copy");
    }

    #[test]
    fn test_get_layer_by_index() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(Some("Layer 1".to_string()));
        stack.add_layer(Some("Layer 2".to_string()));
        stack.add_layer(Some("Layer 3".to_string()));

        let layer = stack.get_layer_by_index(1);
        assert!(layer.is_some());
        assert_eq!(layer.unwrap().name(), "Layer 2");

        let layer = stack.get_layer_by_index(10);
        assert!(layer.is_none());
    }

    #[test]
    fn test_layer_visibility() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(None);

        // 新添加的图层默认是可见的
        let visibility = stack.get_layer_visibility();
        assert_eq!(visibility.len(), 1);
        assert_eq!(visibility[0], 1); // 1 = 可见
    }

    #[test]
    fn test_layer_opacity() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(None);

        // 新添加的图层默认不透明度是 255
        let opacities = stack.get_layer_opacities();
        assert_eq!(opacities.len(), 1);
        assert_eq!(opacities[0], 255); // 默认不透明度
    }

    #[test]
    fn test_layer_names() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(Some("Layer A".to_string()));
        stack.add_layer(Some("Layer B".to_string()));

        let names = stack.get_layer_names();
        assert_eq!(names.len(), 2);
        assert_eq!(names[0], "Layer A");
        assert_eq!(names[1], "Layer B");
    }

    #[test]
    fn test_background_color() {
        let mut stack = LayerStack::new(100, 100);

        // 获取默认背景色（白色）
        let bg = stack.background_color();
        assert_eq!(bg, vec![255, 255, 255, 255]);

        // 设置背景色为红色
        stack.set_background_color(255, 0, 0, 255);
        let bg = stack.background_color();
        assert_eq!(bg, vec![255, 0, 0, 255]);
    }

    #[test]
    fn test_render_composite() {
        let mut stack = LayerStack::new(100, 100);

        // 添加一个图层（默认透明）
        stack.add_layer(None);

        // 渲染合成图像
        let composite = stack.render_composite();
        assert_eq!(composite.get_width(), 100);
        assert_eq!(composite.get_height(), 100);

        let pixels = composite.get_raw_pixels();
        // 检查第一个像素（应该是背景色，白色）
        assert_eq!(pixels[0], 255); // R
        assert_eq!(pixels[1], 255); // G
        assert_eq!(pixels[2], 255); // B
        assert_eq!(pixels[3], 255); // A
    }

    #[test]
    fn test_clear_all() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(None);
        stack.add_layer(None);
        stack.add_layer(None);

        assert_eq!(stack.layer_count(), 3);

        stack.clear_all();
        assert_eq!(stack.layer_count(), 0);
    }

    #[test]
    fn test_merge_down() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(Some("Bottom".to_string()));
        stack.add_layer(Some("Middle".to_string()));
        stack.add_layer(Some("Top".to_string()));

        // 设置上层可见性
        let ids = stack.get_layer_ids();
        if let Some(mut layer) = stack.get_layer(ids[1]) {
            layer.set_visible(true);
        }

        assert_eq!(stack.layer_count(), 3);

        // 将中间层向下合并
        let merged = stack.merge_down(1);
        assert!(merged);
        assert_eq!(stack.layer_count(), 2);

        // 尝试合并第一层（应该失败）
        let merged = stack.merge_down(0);
        assert!(!merged);
    }

    #[test]
    fn test_flatten() {
        let mut stack = LayerStack::new(100, 100);
        stack.add_layer(Some("Layer 1".to_string()));
        stack.add_layer(Some("Layer 2".to_string()));
        stack.add_layer(Some("Layer 3".to_string()));

        assert_eq!(stack.layer_count(), 3);

        // 合并所有图层
        let flattened = stack.flatten();
        assert!(flattened);
        assert_eq!(stack.layer_count(), 1);

        // 尝试再次合并（应该失败）
        let flattened = stack.flatten();
        assert!(!flattened);
    }

    // 跳过这些测试，因为它们在非 WASM 环境中使用 JsValue
    // #[test]
    // fn test_add_layer_from_pixels() {
    //     let mut stack = LayerStack::new(100, 100);
    //
    //     // 创建像素数据
    //     let pixel_count = (100 * 100) as usize;
    //     let mut pixels = vec![0u8; pixel_count * 4];
    //     for i in (0..pixels.len()).step_by(4) {
    //         pixels[i] = 255;     // R
    //         pixels[i + 1] = 0;   // G
    //         pixels[i + 2] = 0;   // B
    //         pixels[i + 3] = 255; // A
    //     }
    //
    //     // 从像素数据添加图层
    //     let id = stack.add_layer_from_pixels(pixels, Some("From Pixels".to_string()));
    //     assert!(id.is_ok());
    //     assert_eq!(stack.layer_count(), 1);
    //
    //     let layer = stack.get_layer(id.unwrap());
    //     assert!(layer.is_some());
    //     assert_eq!(layer.unwrap().name(), "From Pixels");
    // }
    //
    // #[test]
    // fn test_add_layer_from_pixels_invalid_size() {
    //     let mut stack = LayerStack::new(100, 100);
    //
    //     // 创建错误大小的像素数据
    //     let pixels = vec![0u8; 100];
    //
    //     // 应该返回错误
    //     let result = stack.add_layer_from_pixels(pixels, None);
    //     assert!(result.is_err());
    // }
}