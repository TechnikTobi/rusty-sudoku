use crate::board::board::Board;
use crate::board::position::Position;
use crate::color::Color::Color;

use super::EPlacementState::*;

pub struct
BoardManager
{
	full_board: Board,
	play_board: Board,
}

impl
BoardManager
{
	pub fn
	new
	(
		difficulty: u8
	)
	-> Self
	{
		let full_board = Board::generate_full_board();
		let play_board = full_board.generate_final_board(difficulty);

		BoardManager { full_board, play_board }
	}

	pub fn
	set_field
	(
		&mut self,
		position: Position,
		value: u8,
		color: Color
	)
	-> EPlacementState
	{

		// Check if the position is valid & empty
		if !self.play_board.position_is_empty(position)
		{
			return EPlacementState::INVALID;
		}

		// Check if the given value is correct
		if value == self.full_board.get_field(position).unwrap().get_value()
		{
			self.play_board.get_mut_field(position).unwrap().set_value(value);
			self.play_board.get_mut_field(position).unwrap().set_color(color);

			return EPlacementState::CORRECT;
		}
		else 
		{
			return EPlacementState::INCORRECT;
		}
	}

	pub fn
	get_play_board
	(
		&self
	)
	-> &Board
	{
		&self.play_board
	}
}