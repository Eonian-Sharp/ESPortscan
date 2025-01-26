use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::collections::HashSet;
use tokio::net::lookup_host;
use std::net::{IpAddr, Ipv4Addr};
use ipnetwork::IpNetwork;

async fn read_ips_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);

    let mut ip_set = HashSet::new();



    // 调用 lines() 并存储结果
    let mut lines_iter = reader.lines();

    // 使用迭代器逐行读取
    while let Some(line) = lines_iter.next_line().await? {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() {
            ip_set.insert(trimmed_line.to_string());
        }
    }

    let mut ips: Vec<String> = ip_set.into_iter().collect();
    ips.sort_unstable();


    Ok(ips)
}

pub fn input_ip(input: Option<String>) -> Vec<String> {
    input.map_or(Vec::new(), |value| {
        value
            .trim()             
            .split(',')         
            .map(|s| s.trim())   
            .map(|s| s.to_string())
            .collect()           
    })
}

pub async fn input_file(file: Option<String>) -> Vec<String> {
    match file {
        Some(file_path) => {
            match read_ips_from_file(&file_path).await {
                Ok(ips) => ips,
                Err(_) => Vec::new(),
            }
        }
        None => Vec::new(), 
    }
}


pub async fn resolve_ips(input: Vec<String>) -> Vec<String> {
    let mut resolved_ips = Vec::new();

    for entry in input {
        match lookup_host((entry.as_str(), 0)).await {
            Ok(addrs) => {
                for addr in addrs {
                    // 将 IpAddr 转换为字符串并添加到结果向量中
                    resolved_ips.push(addr.ip().to_string());
                }
            }
            Err(_) => {
                // 尝试直接解析为 IP 地址
                if let Ok(ip) = entry.parse::<IpAddr>() {
                    // 将解析成功的 IP 地址转换为字符串
                    resolved_ips.push(ip.to_string());
                } else if let Ok(network) = entry.parse::<IpNetwork>() {
                    // 处理 CIDR 格式的 IP 地址范围
                    match network {
                        IpNetwork::V4(v4_network) => {
                            let start = v4_network.network();
                            let prefix = v4_network.prefix();
                            let num_addresses = 2u32.pow((32 - prefix).into());

                            for i in 0..num_addresses {
                                let mut octets = start.octets();
                                let mut index = 3;
                                let mut remaining = i;

                                while remaining > 0 {
                                    octets[index] += remaining as u8;
                                    remaining >>= 8;
                                    index -= 1;
                                }

                                let ip = Ipv4Addr::from(octets);
                                resolved_ips.push(ip.to_string());
                            }
                        }
                        IpNetwork::V6(_) => {
                            eprintln!("IPv6 range is not supported yet: {}", entry);
                        }
                    }
                } else {
                    // eprintln!("Failed to resolve: {}", entry);
                }
            }
        }
    }

    resolved_ips
}



