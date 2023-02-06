use crate::board::board::Board;

use super::validation_rule::IBoardValidationRule;

pub struct
RowValueRule
{}

impl
IBoardValidationRule
for
RowValueRule
{
	fn 
	validate
	(
		&self,
		board: &Board
	)
	-> bool
	{

		for row in board.get_rows()
		{

			let mut seen_values = Vec::new();

			for field in row
			{
				if field.get_value() == 0 {continue;}
				if seen_values.contains(&field.get_value()) {return false;}

				seen_values.push(field.get_value());
			}
		}

		return true;
	}
}