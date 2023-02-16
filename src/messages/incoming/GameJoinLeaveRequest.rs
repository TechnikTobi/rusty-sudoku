use serde::Deserialize;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;

use super::IRequest::IRequest;

#[derive(Deserialize, Debug)]
pub struct
GameJoinLeaveRequest
{
	PlayerID: NetworkPlayerIdentifier,
	GameID: NetworkGameIdentifier
}

impl
IRequest
for
GameJoinLeaveRequest
{}

impl
GameJoinLeaveRequest
{
	pub fn
	get_player_id
	(
		&self
	)
	-> &NetworkPlayerIdentifier
	{
		&self.PlayerID
	}

	pub fn
	get_game_id
	(
		&self
	)
	-> &NetworkGameIdentifier
	{
		&self.GameID
	}
}