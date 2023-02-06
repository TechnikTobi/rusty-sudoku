use crate::board::{board::Board};

use super::validation_rule::IBoardValidationRule;

pub struct
SquareValueRule
{}

impl
IBoardValidationRule
for
SquareValueRule
{
	fn 
	validate
	(
		&self,
		board: &Board
	)
	-> bool
	{
		
		for square in board.get_squares()
		{

			let mut seen_values = Vec::new();

			for field in square
			{
				if field.get_value() == 0 {continue;}
				if seen_values.contains(&field.get_value()) {return false;}

				seen_values.push(field.get_value());
			}
		}

		return true;
	}
}