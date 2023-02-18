use serde::Deserialize;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;
use crate::messages::base::NetworkField::NetworkField;

use super::IRequest::IRequest;

#[derive(Deserialize, Debug)]
pub struct
GameSetFieldRequest
{
	PlayerID: NetworkPlayerIdentifier,
	GameID: NetworkGameIdentifier,
	Field: NetworkField
}

impl
IRequest
for
GameSetFieldRequest
{}

impl
GameSetFieldRequest
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

	pub fn
	get_field
	(
		&self
	)
	-> &NetworkField
	{
		&self.Field
	}
}