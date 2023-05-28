use uuid::Uuid;

use crate::messages::base::NetworkPlayerToken::NetworkPlayerToken;

/// The player token is, similar to the PlayerID, useful to identify a player
/// However, it is less strict regarding uniqueness
/// Furthermore, it can't be used for identify theft if somebody else obtains
/// it as the system doesn't use the token for identification
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct
PlayerToken
{
	value: Uuid
}

impl PlayerToken
{
	pub fn new() -> Self
	{ PlayerToken { value: Uuid::new_v4() } }

	pub fn
	from_network
	(
		network_id: &NetworkPlayerToken
	)
	-> Self
	{
		PlayerToken
		{
			value: Uuid::parse_str(network_id.get_value()).unwrap()
		}
	}

	pub fn
	to_network
	(
		&self
	)
	-> NetworkPlayerToken
	{
		NetworkPlayerToken::new(self.value.to_string())
	}

}