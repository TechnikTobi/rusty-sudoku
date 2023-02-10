use serde::Serialize;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;

use super::IResponse::IResponse;
#[derive(Serialize, Debug)]
pub struct
PlayerRegistrationResponse
{
	PlayerID: NetworkPlayerIdentifier,
	Message: String,
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
		message: String,
	)
	-> Self
	{
		PlayerRegistrationResponse
		{
			PlayerID: player_id,
			Message: message
		}
	}
}