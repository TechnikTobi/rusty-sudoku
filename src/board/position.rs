use rand::Rng;

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
	random
	(
		x_limit: u8,
		y_limit: u8
	)
	-> Position
	{
		let mut random_generator = rand::thread_rng();
		Position 
		{ 
			x: random_generator.gen_range(0..x_limit), 
			y: random_generator.gen_range(0..y_limit)
		}
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