use ultalpr::engine;
use ultalpr::Config;

#[tokio::main]
async fn main() {
	let config = Config::default();
	let _defer = engine::init(config).await;

	let image = image::open("./images/240201_12h45m10s.jpeg").unwrap();
	let result = engine::process_image(image).await.unwrap();

	println!("{result:?}");
}
