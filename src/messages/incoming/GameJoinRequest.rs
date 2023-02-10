use serde::Deserialize;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;

use super::IRequest::IRequest;

#[derive(Deserialize, Debug)]
pub struct
GameJoinRequest
{
	PlayerID: NetworkPlayerIdentifier,
	GameID: NetworkGameIdentifier
}

impl
IRequest
for
GameJoinRequest
{}

impl
GameJoinRequest
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