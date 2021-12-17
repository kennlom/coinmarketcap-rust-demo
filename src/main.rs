// Globals are declared outside all other scopes.
const API_URL: &str = "https://api.coinmarketcap.com/data-api/v3/topsearch/rank?timeframe=24h&top=10";

/// print_table! macro
///
/// We created the print_table macro to keep the format consistent
/// when printing out the crypto data without having to copy and paste the
/// format in multiple places.
macro_rules! print_table {($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => (
	// {:<20}
	// Pads with spaces on right to fill up n characters
	println!("{:<20} {:<15} {:<20} {:<15} {}", $a, $b, $c, $d, $e)
)}

/// Top Trending Cryptos, data from CoinMarketCap
///
/// This simple application extracts data from the CoinMarketCap api endpoint
/// and output it in a nicely formatted table.
///
/// There is no error handling as this only serve to demonstrate Rust's
/// HTTP Request and json abilities.
fn main() {

	let client = reqwest::blocking::Client::builder()
		.cookie_store(true)
		.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
		.build()
		.unwrap();

	let response		= client.get(API_URL).send().unwrap();
	let body				= response.text().unwrap();
	let json: serde_json::Value	= serde_json::from_str(&body).expect("JSON was not well-formatted");

	let items = json.get("data").unwrap().get("cryptoTopSearchRanks").unwrap();
	let cryptos = items.as_array().unwrap();


	println!("\n\nTop Trending Cryptos, data from CoinMarketCap \n\n");

	print_table!("Name", "Symbol", "Price", "Change 24h", "Volume 24h");
	print_table!("----", "------", "-----", "----------", "----------");

	for crypto in cryptos {
		print_table!(
			crypto["name"].as_str().unwrap(),
			crypto["symbol"].as_str().unwrap(),
			format!("${:.13}", crypto["priceChange"]["price"].as_f64().unwrap()),
			format!("{:.2}%", crypto["priceChange"]["priceChange24h"].as_f64().unwrap()),
			format!("${:.2}", crypto["priceChange"]["volume24h"].as_f64().unwrap())
		);
	}

}
