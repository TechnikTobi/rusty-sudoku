use crate::board::board::Board;

use super::validation_rule::IBoardValidationRule;
use super::rule_column_values::ColumnValueRule;
use super::rule_row_values::RowValueRule;
use super::rule_size::SizeRule;
use super::rule_square_values::SquareValueRule;

pub fn
validate
(
	board: &Board
)
-> bool
{
	let rules: Vec<Box<dyn IBoardValidationRule>> = vec!
	[
		Box::new(ColumnValueRule {}),
		Box::new(RowValueRule {}),
		Box::new(SizeRule {}),
		Box::new(SquareValueRule {})
	];

	for rule in rules
	{
		if !rule.validate(board) {return false;}
	}

	return true;
}