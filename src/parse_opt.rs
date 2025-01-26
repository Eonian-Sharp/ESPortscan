use structopt::StructOpt;
use colorful::{Color, Colorful};
use std::path::PathBuf;
#[derive(StructOpt, Debug)]
#[structopt(name = "ES::Portscan" )]
pub struct Select {
    #[structopt(short, long, help = "输入端口号，例如 80,8080-10000,20311")]
    pub port: Option<String>,
    #[structopt(short, long, help = "输入IP地址, 例如 example.com,1.1.1.1,192.168.0.1/24", conflicts_with = "list", required_unless = "list")]
    pub input: Option<String>,
    #[structopt(short, long, help = "输入IP文件列表", conflicts_with = "input", required_unless = "input")]
    pub list: Option<String>,
    #[structopt(short, long, help = "静默一些七七八八的输出。")]
    pub silent: bool,
    #[structopt(short, long, help = "输出到文件。")]
    pub output: Option<PathBuf>,
}

#[allow(clippy::items_after_statements, clippy::needless_raw_string_hashes)]
pub fn show_banner() {

    let s = r#"> Eonian Sharp
 _____ _____ _ _ _____         _                   
|   __|   __|_|_|  _  |___ ___| |_ ___ ___ ___ ___ 
|   __|__   |_ _|   __| . |  _|  _|_ -|  _| .'|   |
|_____|_____|_|_|__|  |___|_| |_| |___|___|__,|_|_|
                                                   
Super fast port scanning tool
Author: Enomothem"#;

    println!("{}", s.gradient(Color::Green).bold());
    let info = r#"________________________________________
http://eoniansharp.com         
https://github.com/Eonian-Sharp/ESPortscan
--------------------------------------"#;
    println!("{}", info.gradient(Color::Yellow).bold());

}