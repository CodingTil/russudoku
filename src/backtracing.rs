use crate::rules::apply_all_rules;
use crate::sudoku::Sudoku;

fn branch(sudoku: &Sudoku, n: usize) -> Result<Sudoku, ()> {
	// Create a copy of the game, select the first field that is not fixed and fix it to the first possible value (if n == 0, if n>1 then go one value (not field) further)
	let mut new_sudoku = *sudoku;
	let mut row: usize;
	let mut col: usize;
	let mut found = false;
	for r in 0..9 {
		if found {
			break;
		}
		for c in 0..9 {
			if !new_sudoku.fields[r][c].is_fixed() {
				row = r;
				col = c;

				// Fix the field to the nth possible value
				let mut count = 0;
				for i in 1..10 {
					if new_sudoku.fields[row][col].get(i) {
						if count == n {
							new_sudoku.fields[row][col].fix(i);
							found = true;
							break;
						}
						count += 1;
					}
				}

				if !found {
					return Err(());
				}
				break;
			}
		}
	}
	if !found {
		return Err(());
	}

	Ok(new_sudoku)
}

pub fn backtrack(sudoku: &Sudoku) -> Option<Sudoku> {
	// Check if the sudoku is solved
	if sudoku.is_solved() {
		return Some(*sudoku);
	}

	let mut sudoku = *sudoku;

	// Not solved, try rule application
	let changed = apply_all_rules(&mut sudoku);
	if changed {
		// Rules were applied, check if the sudoku is solved
		if sudoku.is_solved() {
			return Some(sudoku);
		}
	}

	// Sudoku still not solved, branch
	for n in 0..9 {
		let new_sudoku = branch(&sudoku, n);
		if new_sudoku.is_err() {
			continue;
		}
		let new_sudoku = new_sudoku.unwrap();
		let result = backtrack(&new_sudoku);
		if result.is_some() {
			return result;
		}
	}

	// No solution found
	None
}
