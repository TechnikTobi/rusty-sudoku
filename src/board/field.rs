use crate::color::Color;
use super::position::Position;

#[derive(Copy, Clone)]
pub struct
Field
{
	position: Position,
	value: u32,
	color: Color
}

impl Field
{

	pub const EMPTY_FIELD_VALUE: u32 = 0;

	pub fn
	new
	(
		x: u8,
		y: u8,
		value: u32
	)
	-> Self
	{
		Field
		{
			position: Position::new(x, y),
			value: value,
			color: Color::get_default_color(),
		}
	}

	pub fn
	get_value
	(
		&self
	)
	-> u32
	{
		return self.value;
	}

	pub fn
	set_value
	(
		&mut self,
		new_value: u32
	)
	{
		self.value = new_value;
	}

	pub fn
	get_color
	(
		&self
	)
	-> Color
	{
		return self.color;
	}

	pub fn
	set_color
	(
		&mut self,
		new_color: Color
	)
	{
		self.color = new_color;
	}

	pub fn
	get_position
	(
		&self
	)
	-> Position
	{
		return self.position;
	}
}