use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::{env, process};

use reqwest::blocking::Client;

#[derive(Debug, serde::Serialize)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time_ms: u128,
    timestamp: SystemTime,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage_and_exit();
    }

    let mut urls = Vec::new();
    let mut file_path = None;
    let mut workers = num_cpus::get();
    let mut timeout_secs = 5;
    let mut retries = 0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i < args.len() {
                    file_path = Some(args[i].clone());
                }
            }
            "--workers" => {
                i += 1;
                if i < args.len() {
                    workers = args[i].parse().unwrap_or(workers);
                }
            }
            "--timeout" => {
                i += 1;
                if i < args.len() {
                    timeout_secs = args[i].parse().unwrap_or(timeout_secs);
                }
            }
            "--retries" => {
                i += 1;
                if i < args.len() {
                    retries = args[i].parse().unwrap_or(retries);
                }
            }
            _ => {
                urls.push(args[i].clone());
            }
        }
        i += 1;
    }

    if let Some(path) = file_path {
        if let Ok(lines) = read_lines(&path) {
            for line in lines.flatten() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    urls.push(trimmed.to_string());
                }
            }
        }
    }

    if urls.is_empty() {
        eprintln!("No URLs provided.");
        print_usage_and_exit();
    }

    let client = Arc::new(Client::builder().timeout(Duration::from_secs(timeout_secs)).build().unwrap());
    let urls = Arc::new(Mutex::new(urls));
    let (tx, rx) = mpsc::channel();

    for _ in 0..workers {
        let urls = Arc::clone(&urls);
        let tx = tx.clone();
        let client = Arc::clone(&client);

        thread::spawn(move || loop {
            let url = {
                let mut locked = urls.lock().unwrap();
                if locked.is_empty() {
                    break;
                }
                locked.pop().unwrap()
            };

            let mut attempt = 0;
            let result;
            let start = Instant::now();
            loop {
                match client.get(&url).send() {
                    Ok(resp) => {
                        result = Ok(resp.status().as_u16());
                        break;
                    }
                    Err(e) => {
                        attempt += 1;
                        if attempt > retries {
                            result = Err(e.to_string());
                            break;
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
            let elapsed = start.elapsed().as_millis();
            let status = WebsiteStatus {
                url: url.clone(),
                status: result.clone(),
                response_time_ms: elapsed,
                timestamp: SystemTime::now(),
            };

            tx.send(status).unwrap();
        });
    }

    drop(tx);

    let mut results = Vec::new();
    for received in rx {
        println!(
            "{} => {} ({}ms)",
            received.url,
            match &received.status {
                Ok(code) => format!("HTTP {}", code),
                Err(e) => format!("Error: {}", e),
            },
            received.response_time_ms
        );
        results.push(received);
    }

    let file = File::create("status.json").expect("Unable to create file");
    serde_json::to_writer_pretty(file, &results).expect("Failed to write JSON");

    println!("Results written to status.json");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_usage_and_exit() {
    eprintln!("Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]");
    process::exit(2);
}
