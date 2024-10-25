// command line format
// port-sniffer -h --> help screen
// port-sniffer -j 100 192.168.1.1 --> j will set number of threads

// reference - https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb (Rustlang Project: Port Sniffer CLI)

use std::net::TcpStream;
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::{env, net::IpAddr};

// store command line arguments in structured format inside a struct
#[allow(dead_code)]
#[derive(Debug)]
struct Args {
    flag: String,
    ipaddr: IpAddr,
    threads: u32,
}

const MAX_PORT: u32 = 65536;

impl Args {
    // annotate str with static so that error can be sent back to main function, might not have needed it if we instead used String (?)
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Args {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            if (f.contains("-h") || f.contains("-help")) && args.len() == 2 {
                println!(
                    "Usage: -j to select number of threads you want
                \n        -h or -help to show this message"
                );
                return Err("help");
            } else if f.contains("-j") && args.len() == 4 {
                let threads = match args[2].parse::<u32>() {
                    Ok(t) => t,
                    Err(_) => return Err("Error parsing thread count"),
                };
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(ip) => ip,
                    Err(_) => return Err("Error parsing IP address"),
                };
                return Ok(Args {
                    flag: String::from("j"),
                    ipaddr,
                    threads,
                });
            }
            return Err("Parsing error");
        };
    }
}

fn scan(tx: Sender<u32>, start_port: u32, addr: IpAddr, num_threads: u32) {
    let mut p = start_port;
    while (MAX_PORT - p) > num_threads {
        let addr_port = format!("{}:{}", addr, p);
        if let Ok(_) = TcpStream::connect(addr_port) {
            println!("Connected to port: {}", p);
            tx.send(p).unwrap();
        }
        p += num_threads;
    }
}

// fn scan_single_threaded(start_port: u32, addr: IpAddr) {
//     for p in start_port..=MAX_PORT {
//         let addr_port = format!("{}:{}", addr, start_port);
//         println!("{}", addr_port);
//         if let Ok(_) = TcpStream::connect(addr_port) {
//             println!("Connected to port: {}", p);
//         }
//     }
// }

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let program = env_args[0].clone();
    let args = Args::new(&env_args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            println!("{} failed to execute: {}", program, err);
            process::exit(1);
        }
    });

    // scan_single_threaded(1, args.ipaddr);

    let threads = args.threads;

    let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
    let (tx, rx) = channel::<u32>();
    for i in 1..=threads {
        let tx = tx.clone();
        join_handles.push(thread::spawn(move || scan(tx, i, args.ipaddr, threads)));
    }

    drop(tx);
    let mut open_ports: Vec<u32> = Vec::new();
    for port in rx {
        open_ports.push(port);
    }

    println!("Open ports: {:?}", open_ports);
}
