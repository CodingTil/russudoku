use crate::sudoku::{NumberField, Sudoku};

pub fn apply_all_rules(sudoku: &mut Sudoku) -> bool {
	let mut changed = false;
	loop {
		let mut changed_this_round = false;
		changed_this_round |= remove_options_fixed_number_row(sudoku);
		changed_this_round |= remove_options_fixed_number_col(sudoku);
		changed_this_round |= remove_options_fixed_number_block(sudoku);
		changed_this_round |= fixate_only_option_in_row(sudoku);
		changed_this_round |= fixate_only_option_in_col(sudoku);
		changed_this_round |= fixate_only_option_in_block(sudoku);
		changed |= changed_this_round;
		if !changed_this_round {
			break;
		}
	}
	changed
}

fn remove_options_fixed_number_row(sudoku: &mut Sudoku) -> bool {
	let mut removed = false;
	for row in 0..9 {
		let mut fixed = NumberField::zeros();
		for col in 0..9 {
			if sudoku.fields[row][col].is_fixed() {
				fixed = fixed.or(&sudoku.fields[row][col]);
			}
		}
		let filter = fixed.not();
		for col in 0..9 {
			if !sudoku.fields[row][col].is_fixed() && sudoku.fields[row][col].and(&fixed).any() {
				sudoku.fields[row][col] = sudoku.fields[row][col].and(&filter);
				removed = true;
			}
		}
	}
	removed
}

fn remove_options_fixed_number_col(sudoku: &mut Sudoku) -> bool {
	let mut removed = false;
	for col in 0..9 {
		let mut fixed = NumberField::zeros();
		for row in 0..9 {
			if sudoku.fields[row][col].is_fixed() {
				fixed = fixed.or(&sudoku.fields[row][col]);
			}
		}
		let filter = fixed.not();
		for row in 0..9 {
			if !sudoku.fields[row][col].is_fixed() && sudoku.fields[row][col].and(&fixed).any() {
				sudoku.fields[row][col] = sudoku.fields[row][col].and(&filter);
				removed = true;
			}
		}
	}
	removed
}

fn remove_options_fixed_number_block(sudoku: &mut Sudoku) -> bool {
	let mut removed = false;
	for block_row in 0..3 {
		for block_col in 0..3 {
			let mut fixed = NumberField::zeros();
			for row in 0..3 {
				for col in 0..3 {
					let row = block_row * 3 + row;
					let col = block_col * 3 + col;
					if sudoku.fields[row][col].is_fixed() {
						fixed = fixed.or(&sudoku.fields[row][col]);
					}
				}
			}
			let filter = fixed.not();
			for row in 0..3 {
				for col in 0..3 {
					let row = block_row * 3 + row;
					let col = block_col * 3 + col;
					if !sudoku.fields[row][col].is_fixed()
						&& sudoku.fields[row][col].and(&fixed).any()
					{
						sudoku.fields[row][col] = sudoku.fields[row][col].and(&filter);
						removed = true;
					}
				}
			}
		}
	}
	removed
}

fn fixate_only_option_in_row(sudoku: &mut Sudoku) -> bool {
	let mut changed = false;
	for row in 0..9 {
		for number in 1..10 {
			let mut count = 0;
			let mut col = 0;
			for i in 0..9 {
				if !sudoku.fields[row][i].is_fixed() && sudoku.fields[row][i].get(number) {
					count += 1;
					col = i;
				}
			}
			if count == 1 {
				sudoku.fields[row][col].fix(number);
				changed = true;
			}
		}
	}
	changed
}

fn fixate_only_option_in_col(sudoku: &mut Sudoku) -> bool {
	let mut changed = false;
	for col in 0..9 {
		for number in 1..10 {
			let mut count = 0;
			let mut row = 0;
			for i in 0..9 {
				if !sudoku.fields[i][col].is_fixed() && sudoku.fields[i][col].get(number) {
					count += 1;
					row = i;
				}
			}
			if count == 1 {
				sudoku.fields[row][col].fix(number);
				changed = true;
			}
		}
	}
	changed
}

fn fixate_only_option_in_block(sudoku: &mut Sudoku) -> bool {
	let mut changed = false;
	for block_row in 0..3 {
		for block_col in 0..3 {
			for number in 1..10 {
				let mut count = 0;
				let mut row = 0;
				let mut col = 0;
				for i in 0..3 {
					for j in 0..3 {
						let r = block_row * 3 + i;
						let c = block_col * 3 + j;
						if !sudoku.fields[r][c].is_fixed() && sudoku.fields[r][c].get(number) {
							count += 1;
							row = r;
							col = c;
						}
					}
				}
				if count == 1 {
					sudoku.fields[row][col].fix(number);
					changed = true;
				}
			}
		}
	}
	changed
}
