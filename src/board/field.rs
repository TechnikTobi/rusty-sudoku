use crate::color::Color;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct
Position
{
	x: u32,
	y: u32
}

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
	pub fn
	new
	(
		x: u32,
		y: u32,
		value: u32
	)
	-> Self
	{
		Field
		{
			position: Position { x: x, y: y },
			value: value,
			color: Color::get_default_color(),
		}
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
	set_color
	(
		&mut self,
		new_color: Color
	)
	{
		self.color = new_color;
	}
}