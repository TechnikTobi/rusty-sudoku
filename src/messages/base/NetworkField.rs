use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

	pub fn get_x     (&self) -> u8 { self.X }
	pub fn get_y     (&self) -> u8 { self.Y }
	pub fn get_value (&self) -> u8 { self.Value }
}