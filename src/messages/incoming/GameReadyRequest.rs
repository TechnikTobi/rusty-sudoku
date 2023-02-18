use serde::Deserialize;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;

use super::IRequest::IRequest;

#[derive(Deserialize, Debug)]
pub struct
GameReadyRequest
{
	ReadyPlayerID: NetworkPlayerIdentifier,
	GameID: NetworkGameIdentifier,
}

impl
IRequest
for
GameReadyRequest
{}

impl
GameReadyRequest
{
	pub fn
	get_player_id
	(
		&self
	)
	-> &NetworkPlayerIdentifier
	{
		&self.ReadyPlayerID
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