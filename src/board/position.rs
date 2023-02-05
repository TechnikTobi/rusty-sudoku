#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct
Position
{
	x: u32,
	y: u32
}

impl
Position
{

	pub fn
	new
	(
		x: u32,
		y: u32
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
	-> u32
	{
		return self.x;
	}

	pub fn
	get_y
	(
		&self
	)
	-> u32
	{
		return self.y;
	}
}