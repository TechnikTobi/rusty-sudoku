use crate::board::board::Board;

use super::validation_rule::IBoardValidationRule;

pub struct
ColumnValueRule
{}

impl
IBoardValidationRule
for
ColumnValueRule
{
	fn 
	validate
	(
		&self,
		board: &Board
	)
	-> bool
	{

		for column in board.get_columns()
		{

			let mut seen_values = Vec::new();

			for field in column
			{
				if field.get_value() == 0 {continue;}
				if seen_values.contains(&field.get_value()) {return false;}

				seen_values.push(field.get_value());
			}
		}

		return true;
	}
}