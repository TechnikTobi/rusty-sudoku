use rand::thread_rng;
use rand::seq::SliceRandom;

use super::position::Position;
use super::field::Field;
use super::board::Board;

impl Board
{

	pub const MIN_VALUE: u8 = 1;
	pub const MAX_VALUE: u8 = 9;

	/// Create a new valid Sudoku board that is fully filled with numbers.
	/// This board can then be used to remove numbers to end up with the final
	/// board for playing a game.
	pub fn
	generate_full_board()
	-> Board
	{
		return Self::add_random_number(&mut Board::new()).unwrap();
	}

	fn
	add_random_number
	(
		board: &mut Board
	)
	-> Option<Board>
	{

		if board.is_full() {return Some(board.clone());}

		for i in 0..Self::MAX_X*Self::MAX_Y
		{
			let pos = Position::new(
				i % Self::MAX_VALUE, 
				i / Self::MAX_VALUE
			);

			// Is the field at the current position already filled with a value?
			// If so, continue. If the position is not in the board, return None
			if let Some(field) = board.get_mut_field(pos)
			{
				if field.get_value() != Field::EMPTY_FIELD_VALUE
				{
					continue;
				}
			}
			else
			{
				return None;
			}

			// The field at the current position is not initialised
			// Therefore, try different values to put there at random
			let mut candidates: Vec<u32> = (1..10).collect();
			candidates.shuffle(&mut thread_rng());

			for value in candidates
			{
				// Check if value would be a valid value for this field
				if 
					board.get_row(pos).iter().any(|field| field.get_value() == value) ||
					board.get_column(pos).iter().any(|field| field.get_value() == value) ||
					board.get_square(pos).iter().any(|field| field.get_value() == value)
				{continue}

				// If value is valid for this field, set it and make recursive
				// call to fill the next field
				// If this is successful, return the new board
				board.get_mut_field(pos).unwrap().set_value(value);
				if let Some(new_board) = Self::add_random_number(board)
				{
					return Some(new_board);
				}

				board.get_mut_field(pos).unwrap().set_value(Field::EMPTY_FIELD_VALUE);
			}

			return None;
		}

		return None;
	}


	/// Take a fully filled valid Sudoku board and remove fields until
	pub fn
	generate_final_board
	(
		&self,
		difficulty: u8
	)
	-> Board
	{
		let mut final_board = self.clone();

		return final_board;
	}
}