use axum::{extract::Query, response::Json, routing::get, Router};
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsArticle {
    pub title: String,
    pub url: String,
    pub source: String,
    pub date: String,
}

#[derive(Deserialize)]
struct NewsQuery {
    query: String,
}

fn resolve_coin_name(symbol: &str) -> &str {
    match symbol.to_uppercase().as_str() {
        "BTC" => "Bitcoin",
        "ETH" => "Ethereum",
        "BNB" => "Binance Coin",
        "ADA" => "Cardano",
        "DOGE" => "Dogecoin",
        "XRP" => "Ripple",
        "SOL" => "Solana",
        "DOT" => "Polkadot",
        "AVAX" => "Avalanche",
        _ => symbol, // fallback to original query if unknown
    }
}


pub async fn fetch_newsapi(query: &str) -> Vec<NewsArticle> {
    let api_key = std::env::var("NEWSAPI_KEY").expect("NEWSAPI_KEY not set");

    let search_term = resolve_coin_name(query); // Resolving "ETH" → "Ethereum"

    let url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&language=en",
        api_key,
        search_term.to_lowercase()
    );

    let res = reqwest::get(&url).await.ok();
    if let Some(response) = res {
        let json: serde_json::Value = response.json().await.unwrap_or_default();


        let mut news = Vec::new();

        if let Some(results) = json["results"].as_array() {
            for a in results.iter().take(5) {
                news.push(NewsArticle {
                    title: a["title"].as_str().unwrap_or("").to_string(),
                    url: a["link"].as_str().unwrap_or("").to_string(),
                    source: a["source_id"].as_str().unwrap_or("NewsData.io").to_string(),
                    date: a["pubDate"].as_str().unwrap_or("").to_string(),
                });
            }
        }
        news
    } else {
        vec![]
    }
}

async fn fetch_coinmarketcap(query: &str) -> Vec<NewsArticle> {
    let api_key = env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY not set");
    let client = reqwest::Client::new();

    let res = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/info")
        .query(&[("symbol", query)])
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await
        .ok();

    if let Some(response) = res {
        let json: serde_json::Value = response.json().await.unwrap_or_default();
        let mut news = Vec::new();
        if let Some(data) = json["data"][query].as_object() {
            let desc = data["description"].as_str().unwrap_or("");
            if let Some(site_url) = data["urls"]["website"][0].as_str() {
                news.push(NewsArticle {
                    title: format!("Overview of {}", query),
                    url: site_url.to_string(),
                    source: "CoinMarketCap".to_string(),
                    date: chrono::Utc::now().to_rfc3339(),
                });
            }
        }
        news
    } else {
        vec![]
    }
}

async fn fetch_news(Query(params): Query<NewsQuery>) -> Json<Vec<NewsArticle>> {
    let query = params.query.clone();

    let (newsapi, cmc) = tokio::join!(
        fetch_newsapi(&query),
        fetch_coinmarketcap(&query)
    );

    let mut combined = newsapi;
    combined.extend(cmc);

    Json(combined)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
        .route("/news", get(fetch_news))
        .route("/", get(|| async {
            axum::response::Html(include_str!("../static/index.html"))
        }))
        .route("/style.css", get(|| async {
            (
                [("Content-Type", "text/css")],
                include_str!("../static/style.css")
            )
        }))
        
        .route("/script.js", get(|| async {
            axum::response::Html(include_str!("../static/script.js"))
        }));

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("🚀 Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
