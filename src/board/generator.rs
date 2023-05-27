use rand::thread_rng;
use rand::seq::SliceRandom;

use super::position::Position;
use super::field::Field;
use super::board::Board;
use super::difficulty::Difficulty;

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
			let mut candidates: Vec<u8> = (1..10).collect();
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
		given_difficulty: u8
	)
	-> Board
	{
		let mut working_board = self.clone();
		let mut checkpoint_board = self.clone();
		let difficulty = Difficulty::bound_difficulty(given_difficulty);

		// Keep track of cleared positions
		let mut cleared_positions:   Vec<Position> = Vec::new();

		let mut reset_position = Position::random(Self::MAX_X, Self::MAX_Y);

		for _ in 0..difficulty
		{
			// If the board has only one solution, set a new checkpoint
			// otherwise revert changes by cloning the checkpoint board
			if working_board.solve() == 1
			{
				checkpoint_board = working_board.clone();
			}
			else
			{
				working_board = checkpoint_board.clone();
			}

			// Early stopping: Don't remove more fields than requested
			if checkpoint_board.count_non_empty_fields() >= difficulty as usize
			{
				return checkpoint_board;
			}

			// Select a random position that hasn't been reset
			while cleared_positions.contains(&reset_position)
			{
				reset_position = Position::random(Self::MAX_X, Self::MAX_Y);
			}

			// Also select the mirror of the random position
			// This has something to do with Sudokus that are symmetric being
			// "better" in some way
			let mirrored_position = reset_position.get_mirror(Self::MAX_X, Self::MAX_Y);

			// Reset the fields
			working_board.get_mut_field(reset_position   ).unwrap().set_value(Field::EMPTY_FIELD_VALUE);
			working_board.get_mut_field(mirrored_position).unwrap().set_value(Field::EMPTY_FIELD_VALUE);

			// Keep track of fields that have been reset
			cleared_positions.push(reset_position   );
			cleared_positions.push(mirrored_position);
		}
		
		return checkpoint_board;
	}
}