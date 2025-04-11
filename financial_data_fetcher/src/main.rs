use std::{fs::OpenOptions, io::Write, thread, time::Duration};

//trait that all assets will implement
trait Pricing {
    fn fetch_price(&mut self);     // fetch latest price from API
    fn save_to_file(&self);        // write price to a file
}

//structs for each asset
struct Bitcoin {
    price: f64,
}

struct Ethereum {
    price: f64,
}

struct SP500 {
    price: f64,
}

//constructors
impl Bitcoin {
    fn new() -> Self {
        Self { price: 0.0 }
    }
}

impl Ethereum {
    fn new() -> Self {
        Self { price: 0.0 }
    }
}

impl SP500 {
    fn new() -> Self {
        Self { price: 0.0 }
    }
}

//  implementation for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) {
        let url = "https://api.coinbase.com/v2/prices/BTC-USD/spot";
        if let Ok(response) = ureq::get(url).call() {
            if let Ok(resp) = response.into_json::<serde_json::Value>() {
                if let Some(price_str) = resp["data"]["amount"].as_str() {
                    if let Ok(price) = price_str.parse::<f64>() {
                        self.price = price;
                    }
                }
            }
        }
    }

    fn save_to_file(&self) {
        let mut file = OpenOptions::new().append(true).create(true).open("bitcoin.txt").unwrap();
        writeln!(file, "Bitcoin: ${:.2}", self.price).unwrap();
    }
}

//implementation for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&mut self) {
        let url = "https://api.coinbase.com/v2/prices/ETH-USD/spot";
        if let Ok(response) = ureq::get(url).call() {
            if let Ok(resp) = response.into_json::<serde_json::Value>() {
                if let Some(price_str) = resp["data"]["amount"].as_str() {
                    if let Ok(price) = price_str.parse::<f64>() {
                        self.price = price;
                    }
                }
            }
        }
    }

    fn save_to_file(&self) {
        let mut file = OpenOptions::new().append(true).create(true).open("ethereum.txt").unwrap();
        writeln!(file, "Ethereum: ${:.2}", self.price).unwrap();
    }
}

// implementation for SP 500 using Yahoo Finance
impl Pricing for SP500 {
    fn fetch_price(&mut self) {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC";
        if let Ok(response) = ureq::get(&url).call() {
            if let Ok(resp) = response.into_json::<serde_json::Value>() {
                if let Some(price) = resp["chart"]["result"]
                    .get(0)
                    .and_then(|r| r["meta"]["regularMarketPrice"].as_f64())
                {
                    self.price = price;
                }
            }
        }
    }

    fn save_to_file(&self) {
        let mut file = OpenOptions::new().append(true).create(true).open("sp500.txt").unwrap();
        writeln!(file, "S&P 500: ${:.2}", self.price).unwrap();
    }
}

fn main() {
    //list of all assets implementing the Pricing trait
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new()),
        Box::new(Ethereum::new()),
        Box::new(SP500::new()),
    ];

    //loop that runs forever updating every 10 seconds
    loop {
        for asset in assets.iter_mut() {
            asset.fetch_price();
            asset.save_to_file();
        }

        //log to show the loop is running
        println!("Prices updated. Waiting 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}
