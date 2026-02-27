fn main() {
    // 在构建时嵌入字体文件
    println!("cargo:rerun-if-changed=crate/fonts/Minikin-1.ttf");
    println!("cargo:rerun-if-changed=crate/fonts");
}