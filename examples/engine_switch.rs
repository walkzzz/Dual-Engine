// 示例 3: 引擎切换
// 运行：cargo run --bin de -- switch claude

fn main() {
    println!("引擎切换示例");
    println!("============");
    println!("");
    println!("查看当前引擎:");
    println!("  cargo run --bin de -- status");
    println!("");
    println!("切换到 Claude:");
    println!("  cargo run --bin de -- switch claude");
    println!("");
    println!("切换回 OpenCode:");
    println!("  cargo run --bin de -- switch opencode");
    println!("");
    println!("运行对话:");
    println!("  cargo run --bin de -- run -p \"hello\"");
}