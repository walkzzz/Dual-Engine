// Dual-Engine 示例程序
//
// 本目录包含可直接运行的示例代码

mod basic_chat;
mod code_generation;
mod engine_switch;

fn main() {
    println!("Dual-Engine 示例程序集合");
    println!("========================");
    println!("");
    println!("可用示例:");
    println!("  1. basic_chat        - 基础对话示例");
    println!("  2. code_generation   - 代码生成示例");
    println!("  3. engine_switch     - 引擎切换示例");
    println!("");
    println!("运行方式:");
    println!("  cargo run --example basic_chat");
    println!("  cargo run --example code_generation");
    println!("  cargo run --example engine_switch");
    println!("");
    println!("使用 CLI:");
    println!("  cargo run --bin de -- run -p \"hello\"");
    println!("  cargo run --bin de -- switch claude");
    println!("  cargo run --bin de -- status");
}