#[derive(PartialEq, Eq, Hash)]
pub struct
Position
{
	x: u32,
	y: u32
}

pub struct
Field
{
	position: Position,
	value: u32
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
			value: value
		}
	}
}