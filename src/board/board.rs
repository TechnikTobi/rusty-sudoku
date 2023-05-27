use crate::board::field::*;
use super::position::Position;

#[derive(Clone)]
pub struct
Board
{
	fields: Vec<Field>
}

impl Board
{

	pub const MAX_X: u8 = 9;
	pub const MAX_Y: u8 = 9;

	pub fn
	new()
	-> Self
	{


		let mut fields = Vec::new();
		(0..Self::MAX_Y).for_each(|y| 
			(0..Self::MAX_X).for_each(|x| 
				{ fields.push(Field::new(x, y, 0)); } 
			) 
		);

		Board { fields }
	}

	pub fn
	get_fields
	(
		&self
	)
	-> &Vec<Field>
	{
		return &self.fields;
	}

	pub fn
	get_mut_field
	(
		&mut self,
		pos: Position
	)
	-> Option<&mut Field>
	{
		self.fields.iter_mut().find(|field| field.get_position() == pos)
	}

	pub fn
	get_field
	(
		&self,
		pos: Position
	)
	-> Option<&Field>
	{
		self.fields.iter().find(|field| field.get_position() == pos)
	}

	pub fn
	count_non_empty_fields
	(
		&self
	)
	-> usize
	{
		self.fields.iter()
			.filter(|field| field.get_value() == Field::EMPTY_FIELD_VALUE)
			.count()
	}
}