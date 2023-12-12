use crate::backtracing::backtrack;

pub mod backtracing;
pub mod rules;
pub mod sudoku;

fn main() {
	// get parameter --game <game> from command line
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 3 {
		println!("Usage: {} --game <game>", args[0]);
		return;
	}

	let game = &args[2];
	let sudoku = sudoku::Sudoku::from_string(game);

	println!("Sudoku to solve:");
	sudoku.print();

	let result = backtrack(&sudoku);
	if result.is_some() {
		println!("Sudoku solved:");
		let sudoku = result.unwrap();
		sudoku.print();
		assert!(sudoku.is_solved());
	} else {
		println!("Sudoku not solved");
	}
}
