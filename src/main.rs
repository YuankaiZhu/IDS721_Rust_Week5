use reqwest::header;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

const API_KEY: &str = "6823396d-c534-4793-a115-48832d58c2c5";

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cryptocurrency {
    name: String,
    symbol: String,
    quote: Quote,
}

fn main() {
    println!("Enter the symbol of a cryptocurrency to get its current price:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let symbol = input.trim().to_uppercase();

    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}",
        symbol
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .header("X-CMC_PRO_API_KEY", API_KEY)
        .send()
        .expect("Failed to send request");

    let data: serde_json::Value = response.json().expect("Failed to parse response");

    let cryptocurrency = match data.get("data") {
        Some(cryptocurrencies) => {
            let quotes = cryptocurrencies.get(&symbol).unwrap().get("quote").unwrap();
            let price = quotes.get("USD").unwrap().get("price").unwrap().as_f64().unwrap();
            let name = cryptocurrencies.get(&symbol).unwrap().get("name").unwrap().as_str().unwrap();
            Cryptocurrency {
                name: name.to_string(),
                symbol: symbol.to_string(),
                quote: Quote { price },
            }
        }
        None => {
            println!("Could not find cryptocurrency with symbol '{}'", symbol);
            return;
        }
    };

    println!(
        "{} ({}) is currently trading at ${:.2}",
        cryptocurrency.name, cryptocurrency.symbol, cryptocurrency.quote.price
    );
}