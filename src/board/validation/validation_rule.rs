use crate::board::board::Board;

pub trait 
IBoardValidationRule
{
	fn validate(&self, board: &Board) -> bool;
}