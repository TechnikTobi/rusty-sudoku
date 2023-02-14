use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct
NetworkGameIdentifier
{
	value: usize
}

impl 
NetworkGameIdentifier
{
	pub fn 
	new
	(
		value: usize
	) 
	-> Self
	{ 
		NetworkGameIdentifier 
		{ 
			value: value
		} 
	}

	pub fn
	get_value
	(
		&self
	)
	-> &usize
	{
		&self.value
	}
}