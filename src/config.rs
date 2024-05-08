use std::{ffi::CString, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::UltalprError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DebugLevel {
	Verbose,
	Info,
	Warn,
	Error,
	Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Charset {
	Latin,
	Korean,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OpenvinoDevice {
	Gna,
	Hetero,
	Cpu,
	Multi,
	Gpu,
	Myriad,
	Hddl,
	Fpga,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuBackend {
	OpenGL,
	OpenCL,
	Nnapi,
	Metal,
	None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecognScoreType {
	Min,
	Mean,
	Median,
	Max,
	MinMax,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RectifyPolarity {
	Both,
	DarkOnBright,
	BrightOnDark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub debug_level: DebugLevel,
	pub debug_write_input_image_enabled: bool,
	pub debug_internal_data_path: PathBuf,
	pub license_token_file: Option<PathBuf>,
	pub license_token_data: Option<String>,
	pub num_threads: i32,
	pub gpgpu_enabled: bool,
	pub npu_enabled: bool,
	pub max_latency: i32,
	pub ienv_enabled: bool,
	pub assets_folder: PathBuf,
	pub charset: Charset,
	pub openvino_enabled: bool,
	pub openvino_device: OpenvinoDevice,
	pub detect_roi: [f32; 4],
	pub detect_minscore: f32,
	pub detect_gpu_backend: GpuBackend,
	pub detect_quantization_enabled: bool,
	pub car_noplate_detect_enabled: bool,
	pub car_noplate_detect_min_score: f32,
	pub pyramidal_search_enabled: bool,
	pub pyramidal_search_sensitivity: f32,
	pub pyramidal_search_minscore: f32,
	pub pyramidal_search_min_image_size_inpixels: u32,
	pub pyramidal_search_quantization_enabled: bool,
	pub klass_lpci_enabled: bool,
	pub klass_vcr_enabled: bool,
	pub klass_vmmr_enabled: bool,
	pub klass_vbsr_enabled: bool,
	pub klass_vcr_gamma: f32,
	pub recogn_score_type: RecognScoreType,
	pub recogn_minscore: f32,
	pub recogn_rectify_enabled: bool,
	pub recogn_rectify_polarity: RectifyPolarity,
	pub recogn_rectify_polarity_preferred: RectifyPolarity,
	pub recogn_gpu_backend: GpuBackend,
	pub recogn_quantization_enabled: bool,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			debug_level: DebugLevel::Info,
			debug_write_input_image_enabled: false,
			debug_internal_data_path: "".into(),
			license_token_file: None,
			license_token_data: None,
			num_threads: -1,
			gpgpu_enabled: true,
			npu_enabled: true,
			max_latency: -1,
			ienv_enabled: false,
			assets_folder: "assets".into(),
			charset: Charset::Latin,
			openvino_enabled: true,
			openvino_device: OpenvinoDevice::Cpu,
			detect_roi: [0., 0., 0., 0.],
			detect_minscore: 0.1,
			detect_gpu_backend: GpuBackend::None,
			detect_quantization_enabled: true,
			car_noplate_detect_enabled: false,
			car_noplate_detect_min_score: 0.8,
			pyramidal_search_enabled: true,
			pyramidal_search_sensitivity: 0.28,
			pyramidal_search_minscore: 0.8,
			pyramidal_search_min_image_size_inpixels: 800,
			pyramidal_search_quantization_enabled: true,
			klass_lpci_enabled: false,
			klass_vcr_enabled: false,
			klass_vmmr_enabled: false,
			klass_vbsr_enabled: false,
			klass_vcr_gamma: 1.5,
			recogn_score_type: RecognScoreType::Min,
			recogn_minscore: 0.3,
			recogn_rectify_enabled: false,
			recogn_rectify_polarity: RectifyPolarity::Both,
			recogn_rectify_polarity_preferred: RectifyPolarity::DarkOnBright,
			recogn_gpu_backend: GpuBackend::None,
			recogn_quantization_enabled: true,
		}
	}
}

impl Config {
	pub fn default_with_token(token: String) -> Self {
		Self { license_token_data: Some(token), ..Default::default() }
	}

	pub fn to_json(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}

	pub fn to_cstring(&self) -> Result<CString, UltalprError> {
		Ok(CString::new(self.to_json()?.as_str())?)
	}
}
