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