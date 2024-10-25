use std::net::IpAddr;
use std::str::FromStr;

// store command line arguments in structured format inside a struct
#[allow(dead_code)]
#[derive(Debug)]
pub struct Args {
    pub ipaddr: IpAddr,
    pub threads: u32,
}

impl Args {
    // annotate str with static so that error can be sent back to main function, might not have needed it if we instead used String (?)
    pub fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let flag = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Args { ipaddr, threads: 4 });
        } else {
            if (flag.contains("-h") || flag.contains("-help")) && args.len() == 2 {
                println!(
                    "Usage: -j to select number of threads you want
                \n        -h or -help to show this message"
                );
                return Err("help");
            } else if flag.contains("-j") && args.len() == 4 {
                let threads = match args[2].parse::<u32>() {
                    Ok(t) => t,
                    Err(_) => return Err("Error parsing thread count"),
                };
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(ip) => ip,
                    Err(_) => return Err("Error parsing IP address"),
                };
                return Ok(Args { ipaddr, threads });
            }
            return Err("Error parsing IP address");
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ip_addr() {
        let env_args: Vec<String> = vec![String::from("port-sniffer"), String::from("127.0.0.1")];
        let args = Args::new(&env_args);
        assert!(args.is_ok());

        let args = args.unwrap();
        assert_eq!(args.ipaddr, IpAddr::from_str("127.0.0.1").unwrap());
        assert_eq!(args.threads, 4);
    }

    #[test]
    fn test_invalid_ip_addr() {
        let env_args: Vec<String> = vec![
            String::from("port-sniffer"),
            String::from("xxx.xxx.xxx.xxx"),
        ];
        let args = Args::new(&env_args);
        assert!(args.is_err());
        assert_eq!(args.err(), Some("Error parsing IP address"));
    }

    #[test]
    fn test_help_screen() {
        let env_args: Vec<String> = vec![String::from("port-sniffer"), String::from("-h")];
        let args = Args::new(&env_args);
        assert!(args.is_err());
        assert_eq!(args.err(), Some("help"));
    }

    #[test]
    fn test_insufficient_arguments() {
        let env_args: Vec<String> = vec![String::from("port-sniffer")];
        let args = Args::new(&env_args);
        assert!(args.is_err());
        assert_eq!(args.err(), Some("Not enough arguments"));
    }

    #[test]
    fn test_thread_number_parsing() {
        let threads_num = 10;
        let env_args: Vec<String> = vec![
            String::from("port-sniffer"),
            String::from("-j"),
            String::from(format!("{}", threads_num)),
            String::from("127.0.0.1"),
        ];
        let args = Args::new(&env_args);
        assert!(args.is_ok());

        let args = args.unwrap();
        assert_eq!(args.ipaddr, IpAddr::from_str("127.0.0.1").unwrap());
        assert_eq!(args.threads, threads_num);
    }
}
