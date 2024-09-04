use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ComicDataResponse {
	pub name: String,
	#[serde(alias = "numberOfPages")]
	pub number_of_pages: u64,
	pub artist: String,
	pub id: u64,
	pub cat: String,
	pub tag: String,
	pub created: String,
	pub updated: String,
	#[serde(alias = "yourRating")]
	pub your_rating: Option<f64>,
	#[serde(alias = "userRating")]
	pub user_rating: Option<f64>,
	pub keywords: Vec<String>,
	#[serde(alias = "previousComic")]
	pub previous_comic: Option<String>,
	#[serde(alias = "nextComic")]
	pub next_comic: Option<String>
}

pub async fn get_characteristics(uri: &str) -> Result<ComicDataResponse, Box<dyn Error>> {
	let resp = reqwest::get(uri).await?;
	let resp = resp.text().await?;
	let cdr = serde_json::from_str(&resp)?;
	Ok(cdr)
}