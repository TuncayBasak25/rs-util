use std::{f32::consts::PI, ops::{Add, Mul, Sub}};

#[derive(Debug, Clone, Copy)]
pub struct Angle(f32);

impl Angle {
	pub fn from_rad(rad: f32) -> Self {
		Self(rad)
	}

	pub fn from_deg(deg: f32) -> Self {
		Self(deg.to_radians())
	}

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

impl Add<Angle> for Angle {
	type Output = Self;

	fn add(self, rhs: Angle) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl Sub<Angle> for Angle {
	type Output = Self;

	fn sub(self, rhs: Angle) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}

impl Mul<f32> for Angle {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self(self.0 / rhs)
	}
}

impl Div<f32>