use std::path::Path;

use tokio::fs;
use tokio::io::AsyncReadExt;
use ultalpr::engine;
use ultalpr::Config;

#[tokio::main]
async fn main() {
	let config = Config::default();
	let _deinit = engine::init(config).await;

	let img_path = Path::new("./images/240201_12h45m10s.jpeg");
	let len = fs::metadata(img_path).await.unwrap().len() as usize;
	let mut file = fs::File::open(img_path).await.unwrap();
	let mut buf = vec![0; len];
	file.read_exact(&mut buf).await.unwrap();

	let image = image::load_from_memory(&buf).unwrap();
	let result = engine::process_image(buf, image).await.unwrap();

	println!("{result:?}");
}
