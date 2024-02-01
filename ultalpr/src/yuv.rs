pub struct YuvImage {
	pub width: usize,
	pub height: usize,
	pub y: Vec<u8>,
	pub u: Vec<u8>,
	pub v: Vec<u8>,
}

impl YuvImage {
	pub fn crop(&self, x: usize, y: usize, w: usize, h: usize) -> Option<YuvImage> {
		if x + w > self.width || y + h > self.height {
			return None;
		}

		let new_y_size = w * h;
		let new_uv_size = w / 2 * h / 2;

		let mut new_y = vec![0u8; new_y_size];
		let mut new_u = vec![0u8; new_uv_size];
		let mut new_v = vec![0u8; new_uv_size];

		for j in 0..h {
			let src_offset = (y + j) * self.width + x;
			let dst_offset = j * w;
			new_y[dst_offset..dst_offset + w].copy_from_slice(&self.y[src_offset..src_offset + w]);
		}

		for j in 0..h / 2 {
			let src_offset = (y / 2 + j) * self.width / 2 + x / 2;
			let dst_offset = j * w / 2;
			new_u[dst_offset..dst_offset + w / 2]
				.copy_from_slice(&self.u[src_offset..src_offset + w / 2]);
			new_v[dst_offset..dst_offset + w / 2]
				.copy_from_slice(&self.v[src_offset..src_offset + w / 2]);
		}

		Some(YuvImage {
			width: w,
			height: h,
			y: new_y,
			u: new_u,
			v: new_v,
		})
	}
}

#[cfg(feature = "openh264")]
const _: () = {
	use openh264::decoder::DecodedYUV;
	use openh264::formats::YUVSource;

	impl<'a> From<&DecodedYUV<'a>> for YuvImage {
		fn from(value: &DecodedYUV<'a>) -> Self {
			Self {
				width: value.width() as usize,
				height: value.height() as usize,
				y: value.y_with_stride().to_owned(),
				u: value.u_with_stride().to_owned(),
				v: value.v_with_stride().to_owned(),
			}
		}
	}
};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_crop_inside_bounds() {
		let width = 4;
		let height = 4;
		let y = vec![1u8; width * height];
		let u = vec![2u8; width / 2 * height / 2];
		let v = vec![3u8; width / 2 * height / 2];
		let image = YuvImage {
			width,
			height,
			y,
			u,
			v,
		};

		let cropped_image = image.crop(1, 1, 2, 2).unwrap();

		assert_eq!(cropped_image.width, 2);
		assert_eq!(cropped_image.height, 2);
		assert_eq!(cropped_image.y.len(), 4);
		assert_eq!(cropped_image.u.len(), 1);
		assert_eq!(cropped_image.v.len(), 1);
	}

	#[test]
	fn test_crop_outside_bounds() {
		let width = 4;
		let height = 4;
		let y = vec![1u8; width * height];
		let u = vec![2u8; width / 2 * height / 2];
		let v = vec![3u8; width / 2 * height / 2];
		let image = YuvImage {
			width,
			height,
			y,
			u,
			v,
		};

		let cropped_image = image.crop(3, 3, 2, 2);

		assert!(cropped_image.is_none());
	}

	#[test]
	fn test_crop_fake_image() {
		let width = 4;
		let height = 4;
		let y = vec![1u8; width * height];
		let u = vec![2u8; width / 2 * height / 2];
		let v = vec![3u8; width / 2 * height / 2];
		let image = YuvImage {
			width,
			height,
			y,
			u,
			v,
		};

		let cropped_image = image.crop(1, 1, 0, 0).unwrap();

		assert_eq!(cropped_image.width, 0);
		assert_eq!(cropped_image.height, 0);
		assert_eq!(cropped_image.y.len(), 0);
		assert_eq!(cropped_image.u.len(), 0);
		assert_eq!(cropped_image.v.len(), 0);
	}
}
