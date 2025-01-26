use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio::time::{timeout, Duration};
use futures::future::join_all;
use std::sync::Arc;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

pub struct Scanner {
    batch_size: usize,
    initial_timeout: Duration,
    detailed_timeout: Duration,
    output: Option<Mutex<File>>,
}

impl Scanner {
    pub fn new(output: Option<File>) -> Self {
        Scanner {
            batch_size: 100,
            initial_timeout: Duration::from_millis(100),
            detailed_timeout: Duration::from_secs(1),
            output: output.map(Mutex::new),
            
        }
    }
    // 对socket进行扫描
    async fn scan_socket(self: Arc<Self>, socket: SocketAddr, timeout_time: Duration) -> (SocketAddr, bool) {
        match timeout(timeout_time, TcpStream::connect(socket)).await {
            Ok(Ok(_)) => (socket, true),  // 端口开放
            _ => (socket, false),        // 端口关闭或超时
        }
    }

    // 启动一个扫描任务，传入一个Socket迭代器
    pub async fn run(self: Arc<Self>, socket_iterator: impl Iterator<Item = SocketAddr>) {
        let mut ftrs = Vec::new();

        // 第一阶段：快速扫描
        for socket in socket_iterator {
            // 判断是否达到批次数量
            if ftrs.len() >= self.batch_size {
                self.clone().process_batch(&mut ftrs).await;    // 处理当前批次任务
            }
            ftrs.push(tokio::spawn(self.clone().scan_socket(socket, self.initial_timeout)));
        }
        self.clone().process_batch(&mut ftrs).await;

        // 第二阶段：详细扫描，对开放的端口进行二次校验，保证正确性。
        let mut detailed_ftrs = Vec::new();
        for result in join_all(ftrs).await {
            match result {
                Ok((socket, is_open)) if is_open => {
                    detailed_ftrs.push(tokio::spawn(self.clone().scan_socket(socket, self.detailed_timeout)));
                }
                Ok((socket, is_open)) => {
                    // println!("{}: {}", socket, if is_open { "Open" } else {"Closed"});
                    if is_open {
                        println!("{}", socket);
                    }
                }
                Err(e) => { eprintln!("任务执行失败: {:?}", e)}
            }
        }
        self.process_batch(&mut detailed_ftrs).await;
    }

    async fn process_batch(self: Arc<Self>, ftrs: &mut Vec<tokio::task::JoinHandle<(SocketAddr, bool)>>) {
        // let opt = parse_opt::Select::from_args();
        if !ftrs.is_empty() {
            let results = join_all(ftrs.drain(..)).await;
            for result in results {
                match result {
                    Ok((socket, is_open)) => {
                        // println!("{}: {}", socket, if is_open {"Open"} else {"Closed"});
                        if is_open {
                            let output = format!{"{}\n", socket};
                            println!("{}", socket);
                            if let Some(file) = &self.output {
                                let mut file = file.lock().unwrap();
                                if let Err(e) = file.write_all(output.as_bytes()) {
                                    eprintln!("写入失败: {:?}", e);
                                }
                            }
                            
                        }
                    }
                    Err(e) => {
                        eprintln!("任务执行失败: {:?}", e);
                    }
                }
            }
        }
    }
}