use std::{env::args, error::Error, io::Cursor};

use image::ImageReader;
mod get;

// Naïve image search
async fn download_image(img_id: u64, path: &str, com_url: &str) -> Result<(), Box<dyn Error>> {
	let destfile = format!("{path}/{:0>3}.jpg", img_id);

	let uri = format!("{com_url}/{:0>3}.jpg", img_id);
	print!("Downloading {uri}... ");
	let img = reqwest::get(uri).await?;
	let img = img.bytes().await?;

	let reader = ImageReader::new(Cursor::new(img))
		.with_guessed_format()?;
	let img = reader.decode()?;

	img.save_with_format(destfile, image::ImageFormat::Jpeg)?;

	println!("Done!");

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = args().collect();
	if args.len() < 2 || args.len() > 3 {
		eprintln!("Usage: {} <comic name> [destination folder]", &args[0]);
		return Ok(()) // Not really an error
	}
	let comic = &args[1];
	let path = args.get(2).unwrap_or(&String::from(".")).to_owned();

	let com_url_name = urlencoding::encode(&comic).to_string();
	let com_url = "https://static.yiffer.xyz/comics/".to_owned() + &com_url_name;
	let com_disk = path + "/" + &comic.replace(' ', "_");

	let _ =	std::fs::create_dir(&com_disk);

	let com_api_url = "https://yiffer.xyz/api/comics/".to_owned() + &com_url_name;
	println!("{com_api_url}");
	let data = get::get_characteristics(&com_api_url).await?;

	for i in 0..data.number_of_pages {
		download_image(i + 1, &com_disk, &com_url).await?;
	}

	Ok(())
}
