use serde::Deserialize;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;

use super::IRequest::IRequest;

#[derive(Deserialize, Debug)]
pub struct
GameStartRequest
{
	MasterID: NetworkPlayerIdentifier,
	GameID: NetworkGameIdentifier
}

impl
IRequest
for
GameStartRequest
{}

impl
GameStartRequest
{
	pub fn
	get_master_id
	(
		&self
	)
	-> &NetworkPlayerIdentifier
	{
		&self.MasterID
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