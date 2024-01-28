use anyhow::Context;
use anyhow::Result;
use engine::Config;
use engine::Engine;
use engine::ImageType;
use image::DynamicImage;
use image::RgbImage;
use openh264::decoder::Decoder;
use openh264::OpenH264API;
use restream_core::Frame;

#[derive(Debug)]
pub struct Snapshot {
	pub vrm: String,
	pub image: DynamicImage,
	pub bounding: DynamicImage,
}

pub struct Recognizer {
	engine: Engine,
	decoder: Decoder,
}

impl Recognizer {
	pub fn init(config: &Config) -> Result<Self> {
		let engine = Engine::init(config)?;
		Ok(Self {
			engine,
			decoder: Decoder::new(OpenH264API::from_source())?,
		})
	}

	pub async fn process(&mut self, frame: Frame) -> Result<Option<Snapshot>> {
		let yuv = self.decoder.decode(&frame.nals)?.context("Invalid frame")?;

		let (width, height) = yuv.dimension_rgb();
		let (y_strides, u_strides, v_strides) = yuv.strides_yuv();

		let result = self.engine.process_yuv(
			ImageType::YUV420P,
			width,
			height,
			y_strides,
			u_strides,
			v_strides,
			yuv.y_with_stride(),
			yuv.u_with_stride(),
			yuv.v_with_stride(),
		)?;

		let mut rgb = vec![0; 3 * width * height];
		yuv.write_rgb8(&mut rgb);
		let rgb_image =
			RgbImage::from_raw(width as u32, height as u32, rgb).context("Invalid rgb image")?;
		let mut image = DynamicImage::from(rgb_image);

		if let Some(plate) = result.plates.first() {
			let (x, y, w, h) = plate.bounding_box();
			let bounding = image.crop(x, y, w, h);
			let vrm = plate.text.to_owned();

			Ok(Some(Snapshot {
				image,
				bounding,
				vrm,
			}))
		} else {
			Ok(None)
		}
	}
}
