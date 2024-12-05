use std::{f32::consts::PI};

#[derive(Debug, Clone, Copy)]
pub struct Angle(f32);

impl Angle {
	pub fn as_rad(&self) -> f32 {
		self.0
	}

	pub fn as_deg(&self) -> f32 {
		self.0.to_degrees()
	}

	pub fn cos(&self) -> f32 {
		self.as_rad().cos()
	}

	pub fn sin(&self) -> f32 {
		self.as_rad().sin()
	}

	pub fn tan(&self) -> f32 {
		self.as_rad().tan()
	}
}