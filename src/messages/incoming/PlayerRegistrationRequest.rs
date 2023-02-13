use serde::Deserialize;

use super::IRequest::IRequest;

#[derive(Deserialize, Debug, Clone)]
pub struct
PlayerRegistrationRequest
{
	PlayerName: String
}

impl
IRequest
for
PlayerRegistrationRequest
{}

impl
PlayerRegistrationRequest
{
	pub fn
	get_player_name
	(
		&self
	)
	-> &String
	{
		&self.PlayerName
	}
}