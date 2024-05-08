use std::{ffi::CString, ptr};

use serde::{Deserialize, Serialize};

use crate::{bindings::ultimateAlprSdk_UltAlprSdkEngine as sdk, yuv::YuvImage, Config, UltalprError};

pub struct Deinit;

impl Drop for Deinit {
	fn drop(&mut self) {
		unsafe {
			sdk::deInit();
		}
	}
}

pub fn init(config: Config) -> Result<Deinit, UltalprError> {
	let cconfig = config.to_cstring()?;
	let config_ptr = cconfig.as_ptr();

	unsafe {
		sdk::init(config_ptr, ptr::null());
	}

	Ok(Deinit)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plate {
	pub text: String,
	pub warped_box: [f32; 8],
}

impl Plate {
	pub fn bounding_box(&self) -> (u32, u32, u32, u32) {
		let [x1, y1, _, _, x2, y2, _, _] = self.warped_box;

		(x1 as u32, y1 as u32, x2 as u32 - x1 as u32, y2 as u32 - y1 as u32)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
	pub duration: u32,
	pub frame_id: u32,
	#[serde(default)]
	pub plates: Vec<Plate>,
}

#[cfg(feature = "image")]
pub fn process_image(image: image::DynamicImage) -> Result<ProcessResult, UltalprError> {
	let result = unsafe {
		sdk::process(0, image.as_bytes().as_ptr() as *const _, image.width() as usize, image.height() as usize, 0, 1)
	};

	let raw_json = unsafe { CString::from_raw(result.json_) };
	let raw_json = raw_json.to_str()?;

	Ok(serde_json::from_str(raw_json)?)
}

pub fn process_yuv(image: YuvImage) -> Result<ProcessResult, UltalprError> {
	let result = unsafe {
		sdk::process1(
			5,
			&image.y as *const _ as *const _,
			&image.u as *const _ as *const _,
			&image.v as *const _ as *const _,
			image.width,
			image.height,
			image.y_strides,
			image.u_strides,
			image.v_strides,
			0,
			1,
		)
	};

	let raw_json = unsafe { CString::from_raw(result.json_) };
	let raw_json = raw_json.to_str()?;

	Ok(serde_json::from_str(raw_json)?)
}
