use std::collections::HashSet;
/// 此处存储了一些端口的分类，便于管理
/// global_distribute 端口全球发布, 来源于Quake空间测绘统计
/// http_protocol http协议端口号
/// industry_net_protocol 工业互联网协议端口
/// society_engineering 社会工程学端口
/// vulnerability_ports 漏洞端口
/// unauthentication_ports 未授权端口号（一般可分为HTTP协议和其它提供服务的协议，基本上未授权的协议都可以进行弱口令尝试以及爆破）
/// top1000_list
#[derive(Debug)]
pub struct ExcellentPort {
    global_distribute: Vec<u16>,
    http_protocol: Vec<u16>,
    industry_net_protocol: Vec<u16>,
    society_engineering: Vec<u16>,
    vulnerability_ports: Vec<u16>,
    unauthentication_ports: Vec<u16>,
    top1000_list: Vec<u16>,
}

impl ExcellentPort {
    pub fn new() -> Self {
        ExcellentPort {
            global_distribute: vec![ 80, 443, 8443, 8080, 2083, 2095, 22, 7547, 2077, 2092, 2096, 2087, 888, 5060, 2086, 8000, 21, 8888, 53],  
            http_protocol: vec![ 80,81,82,83,84,85,86,87,88,89,90,91,92,98,99,443,800,801,808,880,888,889,1000,1010,1080,1081,1082,1099,1118,1888,2008,2020,2100,2375,2379,3000,3008,3128,3505,5555,6080,6648,6868,7000,7001,7002,7003,7004,7005,7007,7008,7070,7071,7074,7078,7080,7088,7200,7680,7687,7688,7777,7890,8000,8001,8002,8003,8004,8006,8008,8009,8010,8011,8012,8016,8018,8020,8028,8030,8038,8042,8044,8046,8048,8053,8060,8069,8070,8080,8081,8082,8083,8084,8085,8086,8087,8088,8089,8090,8091,8092,8093,8094,8095,8096,8097,8098,8099,8100,8101,8108,8118,8161,8172,8180,8181,8200,8222,8244,8258,8280,8288,8300,8360,8443,8448,8484,8800,8834,8838,8848,8858,8868,8879,8880,8881,8888,8899,8983,8989,9000,9001,9002,9008,9010,9043,9060,9080,9081,9082,9083,9084,9085,9086,9087,9088,9089,9090,9091,9092,9093,9094,9095,9096,9097,9098,9099,9100,9200,9443,9448,9800,9981,9986,9988,9998,9999,10000,10001,10002,10004,10008,10010,10250,12018,12443,14000,16080,18000,18001,18002,18004,18008,18080,18082,18088,18090,18098,19001,20000,20720,21000,21501,21502,28018,20880 ],
            industry_net_protocol: vec![ 502, 44818, 102, 161, 162, 500, 53, 22, 21, 20, 80, 443, 111, 137, 138, 139, 23 ],
            society_engineering: vec![ 6, 66, 666, 6666, 8, 88, 888, 8888, 123, 1234, 12345, 54321, 56789, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 30000, 40000, 50000, 60000, 10086, 11111, 22222, 33333, 44444, 55555, 9999, 7777, 5555, 4444, 3333, 2222, 1111 ],
            vulnerability_ports: vec![ 389, 445, 3307, 6379, 21, 9200, 7001, 3389, 5432, 1521, 8081, 11211, 9092, 27017, 27018, 8019, 8042, 9000, 8088, 3888, 2181, 2375, 7848, 2049, 5000, 2379, 4848, 5984, 8083, 8086, 4440, 22, 5631, 5632, 9001, 7077 ],
            unauthentication_ports: vec![ 389, 873, 2049, 2181, 5601, 5900, 5984, 6379, 7001, 8080, 8081, 8088, 8161, 8888, 9200, 10051, 11211, 27017, 28096 ],
            top1000_list: vec![1,3,6,9,13,17,19,20,21,22,23,24,25,30,32,37,42,49,53,70,79,80,81,82,83,84,88,89,99,106,109,110,113,119,125,135,139,143,146,161,163,179,199,211,222,254,255,259,264,280,301,306,311,340,366,389,406,416,425,427,443,444,458,464,481,497,500,512,513,514,524,541,543,544,548,554,563,587,593,616,625,631,636,646,648,666,667,683,687,691,700,705,711,714,720,722,726,749,765,777,783,787,800,808,843,873,880,888,898,900,901,902,911,981,987,990,992,995,999,1000,1001,1007,1009,1010,1021,1022,1023,1024,1025,1026,1027,1028,1029,1030,1031,1032,1033,1034,1035,1036,1037,1038,1039,1040,1041,1042,1043,1044,1045,1046,1047,1048,1049,1050,1051,1052,1053,1054,1055,1056,1057,1058,1059,1060,1061,1062,1063,1064,1065,1066,1067,1068,1069,1070,1071,1072,1073,1074,1075,1076,1077,1078,1079,1080,1081,1082,1083,1084,1085,1086,1087,1088,1089,1090,1091,1092,1093,1094,1095,1096,1097,1098,1099,1102,1104,1105,1106,1107,1110,1111,1112,1113,1117,1119,1121,1122,1123,1126,1130,1131,1137,1141,1145,1147,1148,1151,1154,1163,1164,1165,1169,1174,1183,1185,1186,1192,1198,1201,1213,1216,1217,1233,1236,1244,1247,1259,1271,1277,1287,1296,1300,1309,1310,1322,1328,1334,1352,1417,1433,1443,1455,1461,1494,1500,1503,1521,1524,1533,1556,1580,1583,1594,1600,1641,1658,1666,1687,1700,1717,1718,1719,1720,1723,1755,1761,1782,1801,1805,1812,1839,1862,1863,1875,1900,1914,1935,1947,1971,1974,1984,1998,1999,2000,2001,2002,2003,2004,2005,2006,2007,2008,2009,2013,2020,2021,2030,2033,2034,2038,2040,2041,2042,2045,2046,2047,2048,2065,2068,2099,2103,2105,2106,2111,2119,2121,2126,2135,2144,2160,2170,2179,2190,2196,2200,2222,2251,2260,2288,2301,2323,2366,2381,2382,2393,2399,2401,2492,2500,2522,2525,2557,2601,2604,2607,2638,2701,2710,2717,2725,2800,2809,2811,2869,2875,2909,2920,2967,2998,3000,3003,3005,3006,3011,3013,3017,3030,3052,3071,3077,3128,3168,3211,3221,3260,3268,3283,3300,3306,3322,3323,3324,3333,3351,3367,3369,3370,3371,3389,3404,3476,3493,3517,3527,3546,3551,3580,3659,3689,3703,3737,3766,3784,3800,3809,3814,3826,3827,3851,3869,3871,3878,3880,3889,3905,3914,3918,3920,3945,3971,3986,3995,3998,4000,4001,4002,4003,4004,4005,4045,4111,4125,4129,4224,4242,4279,4321,4343,4443,4444,4445,4449,4550,4567,4662,4848,4899,4998,5000,5001,5002,5003,5009,5030,5033,5050,5054,5060,5080,5087,5100,5101,5120,5190,5200,5214,5221,5225,5269,5280,5298,5357,5405,5414,5431,5440,5500,5510,5544,5550,5555,5560,5566,5631,5633,5666,5678,5718,5730,5800,5801,5810,5815,5822,5825,5850,5859,5862,5877,5900,5901,5902,5903,5906,5910,5915,5922,5925,5950,5952,5959,5960,5961,5962,5987,5988,5998,5999,6000,6001,6002,6003,6004,6005,6006,6009,6025,6059,6100,6106,6112,6123,6129,6156,6346,6389,6502,6510,6543,6547,6565,6566,6580,6646,6666,6667,6668,6689,6692,6699,6779,6788,6792,6839,6881,6901,6969,7000,7001,7004,7007,7019,7025,7070,7100,7103,7106,7200,7402,7435,7443,7496,7512,7625,7627,7676,7741,7777,7800,7911,7920,7937,7999,8000,8001,8007,8008,8009,8010,8021,8031,8042,8045,8080,8081,8082,8083,8084,8085,8086,8087,8088,8089,8093,8099,8180,8192,8193,8200,8222,8254,8290,8291,8300,8333,8383,8400,8402,8443,8500,8600,8649,8651,8654,8701,8800,8873,8888,8899,8994,9000,9001,9002,9009,9010,9040,9050,9071,9080,9090,9099,9100,9101,9102,9110,9200,9207,9220,9290,9415,9418,9485,9500,9502,9535,9575,9593,9594,9618,9666,9876,9877,9898,9900,9917,9929,9943,9968,9998,9999,10000,10001,10002,10003,10009,10012,10024,10082,10180,10215,10243,10566,10616,10621,10626,10628,10778,11110,11967,12000,12174,12265,12345,13456,13722,13782,14000,14238,14441,15000,15002,15003,15660,15742,16000,16012,16016,16018,16080,16113,16992,17877,17988,18040,18101,18988,19101,19283,19315,19350,19780,19801,19842,20000,20005,20031,20221,20828,21571,22939,23502,24444,24800,25734,26214,27000,27352,27355,27715,28201,30000,30718,30951,31038,31337,32768,32769,32770,32771,32772,32773,32774,32775,32776,32777,32778,32779,32780,32781,32782,32783,32784,33354,33899,34571,34572,35500,38292,40193,40911,41511,42510,44176,44442,44501,45100,48080,49152,49153,49154,49155,49156,49157,49158,49159,49160,49163,49165,49167,49175,49400,49999,50000,50001,50002,50006,50300,50389,50500,50636,50800,51103,51493,52673,52822,52848,52869,54045,54328,55055,55555,55600,56737,57294,57797,58080,60020,60443,61532,61900,62078,63331,64623,64680,65000,65129,65389],

        }
    }
    pub fn merge(&self) -> Vec<u16> {
        let mut set = HashSet::new();

        // 将所有向量中的元素插入到 HashSet 中，自动去重
        set.extend(&self.global_distribute);
        set.extend(&self.http_protocol);
        set.extend(&self.industry_net_protocol);
        set.extend(&self.society_engineering);
        set.extend(&self.vulnerability_ports);
        set.extend(&self.unauthentication_ports);
        set.extend(&self.top1000_list);

        // 将 HashSet 转换回 Vec
        set.into_iter().collect()
    }
}
#[allow(dead_code)]
pub fn parse_ports(port_input: &str) -> Vec<u16> {
    let mut ports = Vec::new();
    for part in port_input.split(',') {
        if part.contains('-') {
            // 处理端口范围，如 "1000-2000"
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                if let (Ok(start), Ok(end)) = (range[0].parse::<u16>(), range[1].parse::<u16>()) {
                    ports.extend(start..=end);
                }
            }
        } else {
            // 处理单个端口，如 "80"
            if let Ok(port) = part.parse::<u16>() {
                ports.push(port);
            }
        }
    }
    ports
}
#[allow(dead_code)]
pub fn parse_ports_v2(port_spec: Option<String>) -> Vec<u16> {
    match port_spec {
        Some(spec) => {
            let mut ports = HashSet::new(); // 使用 HashSet 去除重复端口

            for part in spec.split(',') {
                if part.contains('-') {
                    // 处理端口范围，如 "1000-2000"
                    let range: Vec<&str> = part.split('-').collect();
                    if range.len() == 2 {
                        if let (Ok(start), Ok(end)) = (range[0].parse::<u16>(), range[1].parse::<u16>()) {
                            for port in start..=end {
                                ports.insert(port); // 插入端口范围
                            }
                        }
                    }
                } else {
                    // 处理单个端口，如 "80"
                    if let Ok(port) = part.parse::<u16>() {
                        ports.insert(port); // 插入单个端口
                    }
                }
            }

            let mut ports_vec: Vec<u16> = ports.into_iter().collect(); // 转换为 Vec
            ports_vec.sort_unstable(); // 对端口进行排序
            ports_vec
        }
        None => {
            // 如果 port_spec 是 None，返回一个空的 Vec<u16>
            Vec::new()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum PortParseError {
    InvalidPort(String),
    InvalidRange(String),
    InvalidFormat(String),
}

impl std::fmt::Display for PortParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortParseError::InvalidPort(msg) => write!(f, "Invalid port: {}", msg),
            PortParseError::InvalidRange(msg) => write!(f, "Invalid port range: {}", msg),
            PortParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}
#[allow(dead_code)]
fn parse_ports_v3(port_spec: &str) -> Result<Vec<u16>, PortParseError> {
    let mut ports = HashSet::new(); // 使用 HashSet 去除重复端口

    for part in port_spec.split(',') {
        let part = part.trim(); // 去除前后空格

        if part.contains('-') {
            // 处理端口范围，如 "1000-2000"
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                let start = range[0].trim().parse::<u16>().map_err(|_| {
                    PortParseError::InvalidPort(format!("'{}' is not a valid port", range[0]))
                })?;
                let end = range[1].trim().parse::<u16>().map_err(|_| {
                    PortParseError::InvalidPort(format!("'{}' is not a valid port", range[1]))
                })?;

                if start > end {
                    return Err(PortParseError::InvalidRange(format!("'{}-{}'", start, end)));
                }

                for port in start..=end {
                    ports.insert(port);
                }
            } else {
                return Err(PortParseError::InvalidFormat(format!("'{}'", part)));
            }
        } else {
            // 处理单个端口，如 "80"
            let port = part.parse::<u16>().map_err(|_| {
                PortParseError::InvalidPort(format!("'{}' is not a valid port", part))
            })?;
            ports.insert(port);
        }
    }

    let mut ports_vec: Vec<u16> = ports.into_iter().collect();
    ports_vec.sort_unstable();
    Ok(ports_vec)
}