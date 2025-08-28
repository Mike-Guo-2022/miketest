mod result_demo;

fn main() {
    if let Err(e) = result_demo::run() {
        eprintln!("demo 运行错误: {}", e);
    }
}
