# CryptoNews Aggregator

CryptoNews Aggregator is a simple web application built with **Rust (Axum)** and **HTML/CSS/JavaScript** that allows users to search for a cryptocurrency and instantly receive the latest news about it. It fetches data from **CoinMarketCap** for crypto identification and **NewsData.io** for news headlines.

## Features

- **Search any cryptocurrency** (e.g., BTC, ETH, SOL)
- **Fetch latest news** related to the cryptocurrency
- Uses **CoinMarketCap** for token info and **NewsData.io** for news
- Simple frontend interface

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)
- **C/C++ compiler** (required by some Rust dependencies):
  - **Windows**: Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  - **Linux**:  
    ```bash
    sudo apt install build-essential
    ```
  - **macOS**:  
    ```bash
    xcode-select --install
    ```
- API keys:
  - [CoinMarketCap](https://coinmarketcap.com/api/)
  - [NewsData.io](https://newsdata.io/)

## Getting Started

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/cryptonews-aggregator.git
   cd cryptonews-aggregator
   ```

2. **Create `.env` file**:
   ```
   COINMARKETCAP_API_KEY=your_coinmarketcap_api_key
   NEWSAPI_KEY=your_newsdata_api_key
   ```

3. **Run the app**:
   ```bash
   cargo run
   ```

4. Open the browser and go to:
   ```
   http://localhost:3000
   ```

## Project Structure

```
├── src/
│   └── main.rs
├── static/
│   ├── index.html
│   ├── style.css
│   └── script.js
├── .env
└── Cargo.toml
```

## How It Works

- When the user enters a crypto symbol (e.g., ETH), the backend:
  1. Resolves the full coin name using **CoinMarketCap**
  2. Queries **NewsData.io** using the coin name
  3. Returns the top news headlines
- The frontend displays the results instantly.

## Built With

- **Rust + Axum** – Backend
- **reqwest** – HTTP requests
- **serde** – JSON parsing
- **dotenv** – Env variable handling
- **HTML/CSS/JS** – Frontend

## Authors

- **Dias Zakir / SE-2320**
- **Anvar Tamabayev / SE-2320**
