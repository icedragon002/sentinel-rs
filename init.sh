#!/bin/bash

# 初始化
cargo init --edition 2024

# 1. 创建一级模块文件
touch src/scanner.rs
touch src/monitor.rs
touch src/common.rs
touch src/core.rs

# 2. 创建对应的文件夹（放子逻辑）
mkdir -p src/scanner
mkdir -p src/monitor
mkdir -p src/common

# 3. 创建二级子模块
touch src/scanner/rules.rs
touch src/monitor/proc_parser.rs
touch src/common/metrics.rs

# 4. 写入一个现代化的 main.rs 示例
cat <<EOT > src/main.rs
// 在 2024 版中，这些模块会自动根据文件名挂载
// 我们只需要在这里直接使用即可

fn main() {
    println!("Sentinel-RS starting...");
    
    // 示例调用：
    // let metrics = monitor::proc_parser::fetch();
}
EOT

echo "✅ 现代化文件夹结构已就绪！"
echo "提示：不再需要 mod.rs，目录组织清晰，符合 2024 规范。"