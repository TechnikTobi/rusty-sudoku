#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct
Position
{
	x: u8,
	y: u8
}

impl
Position
{

	pub fn
	new
	(
		x: u8,
		y: u8
	)
	-> Position
	{
		Position { x: x, y: y }
	}

	pub fn
	get_x
	(
		&self
	)
	-> u8
	{
		return self.x;
	}

	pub fn
	get_y
	(
		&self
	)
	-> u8
	{
		return self.y;
	}
}