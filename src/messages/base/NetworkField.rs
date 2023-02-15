use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct
NetworkField
{
	X: u8,
	Y: u8,
	Value: u8,
	Color: String,
}

impl
NetworkField
{
	pub fn
	new
	(
		x: u8,
		y: u8, 
		value: u8,
		color: String
	)
	-> Self
	{
		NetworkField 
		{ 
			X: x, 
			Y: y, 
			Value: value, 
			Color: color 
		}
	}
}