use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Read;

use alpr::engine::Engine;

use crate::alpr::config::Config;
use crate::alpr::config::DebugLevel;
use crate::alpr::engine::ImageType;

mod alpr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut config = Config::default();

	let assets = current_dir()?.join("assets");
	config.debug_level = DebugLevel::Warn;
	config.assets_folder = assets.clone();
    config.license_token_data = std::env::var("ULTALPR_TOKEN").ok();

	let engine = Engine::init(&config)?;

	let image_path = assets.join("images/multi.jpg");
	let image_size = {
		let metadata = fs::metadata(&image_path).unwrap();
		metadata.len() as usize
	};
	let image_data = {
		let mut file = File::open(&image_path).unwrap();
		let mut buf = vec![0; image_size];
		file.read_exact(&mut buf)?;
		buf
	};

	let mut image = image::load_from_memory(&image_data)?;

	let result = engine.process(ImageType::RGB24, &image_data, &image)?;

	for plate in result.plates {
		println!("Plate: {}", plate.text);
		let (x, y, w, h) = plate.bounding_box();
		let bbox = image.crop(x, y, w, h);
		bbox.save(format!("./plates/{}.jpeg", plate.text))?;
	}

	Ok(())
}
