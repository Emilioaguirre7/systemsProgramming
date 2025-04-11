# Financial Data Fetcher

This Rust application fetches and logs the live prices of:

- Bitcoin (BTC)
- Ethereum (ETH)
- S&P 500 Index (^GSPC)

The app runs continuously and updates every 10 seconds, saving the results to text files.

---

## Features

-  Written entirely in synchronous Rust (no async)
-  Uses traits and structs to organize logic
-  Fetches live prices from public APIs
-  Writes each asset's price to its own file:
  - `bitcoin.txt`
  - `ethereum.txt`
  - `sp500.txt`

---

## How It Works

- Structs: `Bitcoin`, `Ethereum`, and `SP500`
- Trait: `Pricing` provides `fetch_price()` and `save_to_file()`
- Uses `ureq` for HTTP requests and `serde_json` to parse JSON
- Fetches prices every 10 seconds using a loop and `thread::sleep()`

---

## How to Run

```bash
cargo run
