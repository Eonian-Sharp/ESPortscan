use espscan::ip_ext::input_file;
use espscan::ip_ext::input_ip;
use espscan::ip_ext::resolve_ips;
use structopt::StructOpt;
use std::fs::File;
use std::io;
use std::net::{IpAddr, SocketAddr};
use espscan::port_db::ExcellentPort;
use espscan::port_db::parse_ports_v2;
pub mod parse_opt;
use std::sync::Arc;
use espscan::parse_opt::show_banner;
use espscan::scanner::Scanner;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> io::Result<()> {


    let opt = parse_opt::Select::from_args();
    let output_file = match opt.output {
        Some(path) => Some(File::create(path)?),
        None => None,
    };

    if !opt.silent { show_banner();}
    let scanner = Arc::new(Scanner::new(output_file));
    let start_time = Instant::now();
    let i = if opt.input.clone().is_some() { input_ip(opt.input) } else { input_file(opt.list).await };
    let i = resolve_ips(i).await;
    let o = if opt.port.is_some() { parse_ports_v2(opt.port) } else { ExcellentPort::new().merge() };
    let socket_iterator = i.into_iter().flat_map(|ip| {let ip: IpAddr = ip.parse().unwrap(); o.iter().cloned().map(move |port| SocketAddr::new(ip, port))});
    scanner.run(socket_iterator).await; // Execute
    let duration = start_time.elapsed();
    if !opt.silent {println!("{}\n扫描完成，总耗时: {:.2?} 秒", "-".repeat(30), duration);}

    Ok(())

    
}