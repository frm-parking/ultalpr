pub struct YuvImage {
	pub width: usize,
	pub height: usize,
	pub y: Vec<u8>,
	pub u: Vec<u8>,
	pub v: Vec<u8>,
	pub y_strides: usize,
	pub u_strides: usize,
	pub v_strides: usize,
}

#[cfg(feature = "openh264")]
const _: () = {
	use openh264::decoder::DecodedYUV;

	impl<'a> From<&DecodedYUV<'a>> for YuvImage {
		fn from(value: &DecodedYUV<'a>) -> Self {
			let (width, height) = value.dimension_rgb();
			let (y_strides, u_strides, v_strides) = value.strides_yuv();

			Self {
				width,
				height,
				y: value.y_with_stride().to_owned(),
				u: value.u_with_stride().to_owned(),
				v: value.v_with_stride().to_owned(),
				y_strides,
				v_strides,
				u_strides,
			}
		}
	}
};
