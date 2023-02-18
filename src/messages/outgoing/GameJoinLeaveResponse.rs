use serde::Serialize;

use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;

#[derive(Serialize, Debug, Clone)]
pub struct
GameJoinLeaveResponse
{
	JoinLeaveGameID: NetworkGameIdentifier,
}

impl
GameJoinLeaveResponse
{
	pub fn
	new
	(
		id: NetworkGameIdentifier,
	)
	-> Self
	{
		GameJoinLeaveResponse
		{
			JoinLeaveGameID: id,
		}
	}
}
