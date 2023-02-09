use serde::Serialize;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;

use super::IResponse::IResponse;

#[derive(Serialize, Debug)]
pub struct
PlayerRegistrationResponse
{
	PlayerID: NetworkPlayerIdentifier
}

impl
IResponse
for
PlayerRegistrationResponse
{}

impl
PlayerRegistrationResponse
{

	pub fn
	new
	(
		PlayerID: NetworkPlayerIdentifier
	)
	-> Self
	{
		PlayerRegistrationResponse
		{
			PlayerID: PlayerID
		}
	}
}