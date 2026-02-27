#!/bin/bash

# Photon WASM 并行化版本构建脚本
# 使用 wasm-bindgen-rayon 启用多线程支持

set -e  # 遇到错误时退出

echo "========================================"
echo "Photon WASM 并行化版本构建脚本"
echo "========================================"
echo ""

# 检查必要的工具
echo "检查必要的工具..."

if ! command -v wasm-pack &> /dev/null; then
    echo "错误: wasm-pack 未安装"
    echo "请运行: cargo install wasm-pack"
    exit 1
fi

if ! command -v wasm-opt &> /dev/null; then
    echo "错误: wasm-opt 未安装"
    echo "请安装 Binaryen: https://github.com/WebAssembly/binaryen"
    exit 1
fi

echo "✓ wasm-pack 已安装"
echo "✓ wasm-opt 已安装"
echo ""

# 检查 Rust 工具链
echo "检查 Rust 工具链..."

if ! command -v rustup &> /dev/null; then
    echo "错误: rustup 未安装"
    exit 1
fi

CURRENT_TOOLCHAIN=$(rustup show active-toolchain | cut -d' ' -f1)
echo "当前工具链: $CURRENT_TOOLCHAIN"

if [[ ! "$CURRENT_TOOLCHAIN" =~ nightly ]]; then
    echo "警告: 当前不是 nightly 工具链"
    echo "请确保 rust-toolchain.toml 中配置了 nightly-2025-11-15"
fi

# 检查 wasm32-unknown-unknown 目标
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "正在安装 wasm32-unknown-unknown 目标..."
    rustup target add wasm32-unknown-unknown
fi

echo "✓ Rust 工具链配置正确"
echo ""

# 清理之前的构建
echo "清理之前的构建..."
rm -rf pkg/
echo "✓ 清理完成"
echo ""

# 构建
echo "开始构建并行化版本..."
echo "使用 wasm-bindgen-rayon 启用多线程支持"
echo ""

wasm-pack build --target web

BUILD_STATUS=$?

if [ $BUILD_STATUS -eq 0 ]; then
    echo ""
    echo "========================================"
    echo "✓ 构建成功！"
    echo "========================================"
    echo ""
    echo "构建产物:"
    ls -lh pkg/
    echo ""
    echo "WASM 文件信息:"
    wasm-objdump -h pkg/photon_wasm_bg.wasm | head -15
    echo ""
    echo "导出的关键函数:"
    echo "  - initThreadPool: 初始化线程池"
    echo "  - ImageProcessor: 图像处理器"
    echo ""
    echo "下一步:"
    echo "  1. 查看 WASM-PARALLEL-BUILD-GUIDE.md 了解使用方法"
    echo "  2. 在浏览器中打开 test-parallel.html 进行测试"
    echo "  3. 确保 Web 服务器配置了 COOP/COEP 响应头"
    echo ""
else
    echo ""
    echo "========================================"
    echo "✗ 构建失败"
    echo "========================================"
    exit 1
fi