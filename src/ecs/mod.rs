
pub struct Flag(usize);

impl Flag {
	fn get(&self) -> usize {
		self.0
	}
	pub fn set_flag(&mut self, flag: Flag) {
		self.0 = flag.0;
	}

	pub fn add_flag(&mut self, flag: Flag) {
		self.0 = self.get() | flag.get();
	}

	pub fn remove_flag(&mut self, flag: Flag) {
		self.0 = self.get() & !flag.get();
	}

	pub fn has_flag(&mut self, flag: Flag) -> bool {
		self.get() & flag.get() != 0
	}
}

