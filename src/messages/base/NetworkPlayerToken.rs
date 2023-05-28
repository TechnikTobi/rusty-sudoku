use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct
NetworkPlayerToken
{
	value: String
}

impl 
NetworkPlayerToken
{
	pub fn 
	new
	(
		value: String
	) 
	-> Self
	{ 
		NetworkPlayerToken 
		{ 
			value: value
		} 
	}

	pub fn
	get_value
	(
		&self
	)
	-> &String
	{
		&self.value
	}
}