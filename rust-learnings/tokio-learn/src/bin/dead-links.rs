use reqwest;
use tokio::{self, fs};
use tokio::time::{Duration, sleep};
use std::collections::HashMap;

const THREAD_NUM: usize = 8;

async fn request_and_print_error(links: Vec<String>) {
    for link in links.iter() {
        sleep(Duration::from_millis(100)).await;
        let req = reqwest::get(link.clone()).await;
        match req {
            Ok(response) => {
                let status = response.status();
                if status.is_server_error() || status.is_client_error() {
                    println!("{}", link);
                }
            }, 
            Err(err) => eprintln!("{:?}", err),
        };
    }
}

async fn check_dead_links(path: &str) {
    let data: String = fs::read_to_string(path).await.unwrap();
    let lines: Vec<String> = data.lines().map(|s| s.trim().to_string()).collect();
    let mut join_handles = Vec::new();
    let div = lines.len() / THREAD_NUM;
    for i in 0..THREAD_NUM {
        let chunk = Vec::from(&lines[i * div..(i+1) * div]);
        let handle = tokio::spawn(async move {
            request_and_print_error(chunk).await;
        });
        join_handles.push(handle);
    };

    let final_chunk = Vec::from(&lines[7 * div..]);
    let handle = tokio::spawn(async move {
        request_and_print_error(final_chunk).await;
    });
    join_handles.push(handle);

    for handle in join_handles {
        handle.await.unwrap();
    }
}

async fn check_duplicate_links(path: &str) {
    let data: String = fs::read_to_string(path).await.unwrap();
    let lines: Vec<String> = data.lines().map(|s| s.trim().to_string()).collect();
    let mut map: HashMap<String, u32> = HashMap::new();
    for url in lines.iter() {
        let l = url.len() - 1;
        let mut url_normalized = url.clone();
        if url.as_bytes()[l] != b'/' {
            url_normalized = format!("{}/", url);
        }

        match map.get(&url_normalized) {
            Some(count) => map.insert(url_normalized, count + 1),
            None => map.insert(url_normalized, 1),
        };
    };
    
    for (k, v) in map.iter() {
        if *v > 1 {
            println!("{} : {}", k, v);
        }
    }
}


#[tokio::main]
async fn main() {
    const PATH: &str = "links-rust.txt";
    // check_dead_links(PATH).await;
    check_duplicate_links(PATH).await;
}
