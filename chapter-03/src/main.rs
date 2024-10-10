use std::error::Error;

use clap::Parser; // clap 是 Rust 社区开发的命令行参数解析库
use reqwest::blocking::Client; // reqwest 是 Rust 社区开发的 HTTP 客户端库
use reqwest::header::HeaderMap;

#[derive(Parser)] // 宏，暂时不需要了解
#[command(
    author,
    version,
    about = "Sends HTTP requests and prints detailed information."
)]

struct Cli {
    #[arg(short, long, help = "The URL to send the request to", required = true)]
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let response = send_request(&cli.url)?;
    print_response_info(response)?;

    Ok(())
}

// 发起 HTTP 请求
fn send_request(url: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
    let client = Client::builder().build()?;
    let response = client.get(url).send()?;

    Ok(response)
}

// 打印出 HTTP 响应的详细信息
fn print_response_info(response: reqwest::blocking::Response) -> Result<(), Box<dyn Error>> {
    println!("Status: {}", response.status());
    println!("Headers:");
    print_headers(response.headers());
    let body = response.text()?;
    println!("Body:\n{}", body);

    Ok(())
}

// 打印出 HTTP 响应的头部信息
fn print_headers(headers: &HeaderMap) {
    for (key, value) in headers.iter() {
        println!(" {}: {}", key, value.to_str().unwrap_or("[unprintable]"));
    }
}
