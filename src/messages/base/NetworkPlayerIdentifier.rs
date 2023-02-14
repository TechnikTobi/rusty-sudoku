use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct
NetworkPlayerIdentifier
{
	value: usize
}

impl 
NetworkPlayerIdentifier
{
	pub fn 
	new
	(
		value: usize
	) 
	-> Self
	{ 
		NetworkPlayerIdentifier 
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