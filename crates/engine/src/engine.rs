use std::ffi::c_void;
use std::ffi::CString;
use std::ptr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use image::DynamicImage;
use serde::Deserialize;
use serde::Serialize;

use super::bindings::*;
use super::config::Config;
use super::error::ConfigCastError;
use super::error::ProcessError;

static INIT: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
pub enum ImageType {
	RGB24,
	RGBA32,
	BGRA32,
	NV12,
	NV21,
	YUV420P,
	YVU420P,
	YUV422P,
	YUV444P,
	BGR24,
	Y,
}

impl ImageType {
	pub fn to_internal(&self) -> u32 {
		match self {
			Self::RGB24 => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_RGB24,
			Self::RGBA32 => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_RGBA32,
			Self::BGRA32 => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_BGRA32,
			Self::NV12 => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_NV12,
			Self::NV21 => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_NV21,
			Self::YUV420P => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_YUV420P,
			Self::YVU420P => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_YVU420P,
			Self::YUV422P => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_YUV422P,
			Self::YUV444P => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_YUV444P,
			Self::BGR24 => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_BGR24,
			Self::Y => ultimateAlprSdk_ULTALPR_SDK_IMAGE_TYPE_ULTALPR_SDK_IMAGE_TYPE_Y,
		}
	}
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

		(
			x1 as u32,
			y1 as u32,
			x2 as u32 - x1 as u32,
			y2 as u32 - y1 as u32,
		)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
	pub duration: u32,
	pub frame_id: u32,
	#[serde(default = "Vec::new")]
	pub plates: Vec<Plate>,
}

#[derive(Debug)]
pub struct Engine {
	#[allow(unused)]
	cconfig: CString,
}

impl Engine {
	pub fn init(config: &Config) -> Result<Engine, ConfigCastError> {
		let status = INIT.load(Ordering::Relaxed);

		let cconfig = config.to_cstring()?;

		if status {
			panic!("Only one instance can be initialized");
		}

		unsafe {
			ultimateAlprSdk_UltAlprSdkEngine::init(cconfig.as_ptr(), ptr::null());
		}

		INIT.store(true, Ordering::Relaxed);

		Ok(Self { cconfig })
	}

	pub fn process(
		&self,
		image_type: ImageType,
		raw: &[u8],
		image: &DynamicImage,
	) -> Result<ProcessResult, ProcessError> {
		let result = unsafe {
			ultimateAlprSdk_UltAlprSdkEngine::process(
				image_type.to_internal(),
				image.as_bytes().as_ptr() as *const c_void,
				image.width() as usize,
				image.height() as usize,
				0,
				ultimateAlprSdk_UltAlprSdkEngine::exifOrientation(raw.as_ptr() as *const c_void, raw.len()),
			)
		};

		let raw_json = unsafe { CString::from_raw(result.json_) };
		let raw_json = raw_json.to_str()?;

		Ok(serde_json::from_str(raw_json)?)
	}
}

impl Drop for Engine {
	fn drop(&mut self) {
		INIT.store(false, Ordering::Relaxed);

		unsafe {
			ultimateAlprSdk_UltAlprSdkEngine::deInit();
		}
	}
}
