#[derive(Copy, Clone, Debug)]
pub struct NumberField {
	pub one: bool,
	pub two: bool,
	pub three: bool,
	pub four: bool,
	pub five: bool,
	pub six: bool,
	pub seven: bool,
	pub eight: bool,
	pub nine: bool,
}

impl NumberField {
	pub fn new() -> NumberField {
		NumberField {
			one: true,
			two: true,
			three: true,
			four: true,
			five: true,
			six: true,
			seven: true,
			eight: true,
			nine: true,
		}
	}

	pub fn get(&self, n: u8) -> bool {
		match n {
			1 => self.one,
			2 => self.two,
			3 => self.three,
			4 => self.four,
			5 => self.five,
			6 => self.six,
			7 => self.seven,
			8 => self.eight,
			9 => self.nine,
			_ => panic!("Invalid number"),
		}
	}

	pub fn zeros() -> NumberField {
		NumberField {
			one: false,
			two: false,
			three: false,
			four: false,
			five: false,
			six: false,
			seven: false,
			eight: false,
			nine: false,
		}
	}

	pub fn ones() -> NumberField {
		NumberField {
			one: true,
			two: true,
			three: true,
			four: true,
			five: true,
			six: true,
			seven: true,
			eight: true,
			nine: true,
		}
	}

	pub fn reset(&mut self) {
		self.one = false;
		self.two = false;
		self.three = false;
		self.four = false;
		self.five = false;
		self.six = false;
		self.seven = false;
		self.eight = false;
		self.nine = false;
	}

	pub fn fix(&mut self, n: u8) {
		self.reset();
		match n {
			1 => self.one = true,
			2 => self.two = true,
			3 => self.three = true,
			4 => self.four = true,
			5 => self.five = true,
			6 => self.six = true,
			7 => self.seven = true,
			8 => self.eight = true,
			9 => self.nine = true,
			_ => panic!("Invalid number"),
		}
	}

	pub fn is_fixed(&self) -> bool {
		let mut count = 0;
		if self.one {
			count += 1;
		}
		if self.two {
			count += 1;
		}
		if self.three {
			count += 1;
		}
		if self.four {
			count += 1;
		}
		if self.five {
			count += 1;
		}
		if self.six {
			count += 1;
		}
		if self.seven {
			count += 1;
		}
		if self.eight {
			count += 1;
		}
		if self.nine {
			count += 1;
		}
		count == 1
	}

	pub fn get_fixed(&self) -> Result<u8, String> {
		if !self.is_fixed() {
			return Err("Not fixed".to_string());
		}
		if self.one {
			Ok(1)
		} else if self.two {
			Ok(2)
		} else if self.three {
			Ok(3)
		} else if self.four {
			Ok(4)
		} else if self.five {
			Ok(5)
		} else if self.six {
			Ok(6)
		} else if self.seven {
			Ok(7)
		} else if self.eight {
			Ok(8)
		} else if self.nine {
			Ok(9)
		} else {
			Err("Not fixed".to_string())
		}
	}

	pub fn or(&self, other: &NumberField) -> NumberField {
		NumberField {
			one: self.one || other.one,
			two: self.two || other.two,
			three: self.three || other.three,
			four: self.four || other.four,
			five: self.five || other.five,
			six: self.six || other.six,
			seven: self.seven || other.seven,
			eight: self.eight || other.eight,
			nine: self.nine || other.nine,
		}
	}

	pub fn and(&self, other: &NumberField) -> NumberField {
		NumberField {
			one: self.one && other.one,
			two: self.two && other.two,
			three: self.three && other.three,
			four: self.four && other.four,
			five: self.five && other.five,
			six: self.six && other.six,
			seven: self.seven && other.seven,
			eight: self.eight && other.eight,
			nine: self.nine && other.nine,
		}
	}

	pub fn not(&self) -> NumberField {
		NumberField {
			one: !self.one,
			two: !self.two,
			three: !self.three,
			four: !self.four,
			five: !self.five,
			six: !self.six,
			seven: !self.seven,
			eight: !self.eight,
			nine: !self.nine,
		}
	}

	pub fn any(&self) -> bool {
		self.one
			|| self.two || self.three
			|| self.four || self.five
			|| self.six || self.seven
			|| self.eight
			|| self.nine
	}
}

impl Default for NumberField {
	fn default() -> NumberField {
		NumberField::new()
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Sudoku {
	pub fields: [[NumberField; 9]; 9],
}

impl Sudoku {
	pub fn from_string(game: &str) -> Sudoku {
		let mut sudoku = Sudoku {
			fields: [[NumberField::new(); 9]; 9],
		};
		// string is numbers (0..10) in row major order (0 is empty)
		assert!(game.len() == 9 * 9);
		let mut row = 0;
		let mut col = 0;
		for c in game.chars() {
			let n = c.to_digit(10).unwrap() as u8;
			if n != 0 {
				sudoku.fields[row][col].fix(n);
			}
			if col == 8 {
				row += 1;
				col = 0;
			} else {
				col += 1;
			}
		}
		sudoku
	}

	#[allow(clippy::println_empty_string)]
	pub fn print(&self) {
		for row in 0..9 {
			for col in 0..9 {
				if self.fields[row][col].is_fixed() {
					print!("{}", self.fields[row][col].get_fixed().unwrap());
				} else {
					print!(" ");
				}
				if col == 2 || col == 5 {
					print!("|");
				}
			}
			println!("");
			if row == 2 || row == 5 {
				println!("---+---+---");
			}
		}
	}

	pub fn is_solved(&self) -> bool {
		if !self.is_legal() {
			return false;
		}
		for row in 0..9 {
			for col in 0..9 {
				if !self.fields[row][col].is_fixed() {
					return false;
				}
			}
		}
		true
	}

	pub fn is_legal(&self) -> bool {
		// Check that each row contains a maximum of one of each number
		for row in 0..9 {
			let mut counts = [false; 9];
			for col in 0..9 {
				let fixed = self.fields[row][col].get_fixed();
				if fixed.is_err() {
					continue;
				}
				let fixed = fixed.unwrap();
				let index = fixed as usize - 1;
				if counts[index] {
					return false;
				} else {
					counts[index] = true;
				}
			}
		}

		// Check that each column contains a maximum of one of each number
		for col in 0..9 {
			let mut counts = [false; 9];
			for row in 0..9 {
				let fixed = self.fields[row][col].get_fixed();
				if fixed.is_err() {
					continue;
				}
				let fixed = fixed.unwrap();
				let index = fixed as usize - 1;
				if counts[index] {
					return false;
				} else {
					counts[index] = true;
				}
			}
		}

		// Check that each 3x3 square contains a maximum of one of each number
		for row in 0..3 {
			for col in 0..3 {
				let mut counts = [false; 9];
				for i in 0..3 {
					for j in 0..3 {
						let fixed = self.fields[row * 3 + i][col * 3 + j].get_fixed();
						if fixed.is_err() {
							continue;
						}
						let fixed = fixed.unwrap();
						let index = fixed as usize - 1;
						if counts[index] {
							return false;
						} else {
							counts[index] = true;
						}
					}
				}
			}
		}

		// All checks passed, the Sudoku is legal
		true
	}
}
