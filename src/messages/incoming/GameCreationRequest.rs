use serde::Deserialize;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;

use super::IRequest::IRequest;

#[derive(Deserialize, Debug)]
pub struct
GameCreationRequest
{
	PlayerID: NetworkPlayerIdentifier,
	GameName: String,
	Difficulty: u8
}

impl
IRequest
for
GameCreationRequest
{}

impl
GameCreationRequest
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
	get_game_name
	(
		&self
	)
	-> &String
	{
		&self.GameName
	}

	pub fn
	get_difficulty
	(
		&self
	)
	-> &u8
	{
		&self.Difficulty
	}
}