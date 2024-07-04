use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::io::Read;
use sysinfo::{System, SystemExt, CpuExt};
use serde_json::json;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut system = System::new_all();
    system.refresh_all();

    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let memory_usage_percentage = (used_memory as f64 / total_memory as f64) * 100.0;

    system.refresh_cpu();
    let global_cpu_usage = system.global_cpu_info().cpu_usage();

    let stats = json!({
        "memory_usage": memory_usage_percentage,
        "cpu_usage": global_cpu_usage
    });

    let response = stats.to_string();
    stream.write_all(response.as_bytes()).unwrap();
}
