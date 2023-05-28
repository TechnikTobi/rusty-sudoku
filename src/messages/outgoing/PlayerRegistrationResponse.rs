use serde::Serialize;

use crate::messages::base::{NetworkPlayerIdentifier::NetworkPlayerIdentifier, NetworkPlayerToken::NetworkPlayerToken};

use super::IResponse::IResponse;
#[derive(Serialize, Debug)]
pub struct
PlayerRegistrationResponse
{
	PlayerID:                          NetworkPlayerIdentifier,
	PlayerToken:                       NetworkPlayerToken,
	Message:                           String,
}

impl
IResponse
for
PlayerRegistrationResponse
{
	fn get_message(&self) -> &String { &self.Message }
}

impl
PlayerRegistrationResponse
{

	pub fn
	new
	(
		player_id: NetworkPlayerIdentifier,
		player_token: NetworkPlayerToken,
		message: String,
	)
	-> Self
	{
		PlayerRegistrationResponse
		{
			PlayerID: player_id,
			PlayerToken: player_token,
			Message: message
		}
	}
}