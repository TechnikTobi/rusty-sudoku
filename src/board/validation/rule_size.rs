use crate::board::{board::Board};

use super::validation_rule::IBoardValidationRule;

pub struct
SizeRule
{}

impl
IBoardValidationRule
for
SizeRule
{
	fn 
	validate
	(
		&self,
		board: &Board
	)
	-> bool
	{
		
		board.get_max_x_pos() == board.get_max_y_pos() &&
		board.get_max_x_pos().get_x() == Board::MAX_X-1 &&
		board.get_max_y_pos().get_y() == Board::MAX_Y-1

	}
}