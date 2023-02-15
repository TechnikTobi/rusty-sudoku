use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct
PlayerListElement
{
	Name: String,
	Color: String,
	Points: u64,
}

impl
PlayerListElement
{
	pub fn
	new
	(
		name: String,
		color: String,
		points: u64,
	)
	-> Self
	{
		PlayerListElement
		{
			Name: name,
			Color: color,
			Points: points,
		}
	}
}