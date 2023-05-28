use serde::Serialize;

use crate::messages::base::NetworkPlayerToken::NetworkPlayerToken;
use crate::messages::base::PlayerListElement::PlayerListElement;
use crate::messages::base::NetworkField::NetworkField;

#[derive(Serialize, Debug, Clone)]
pub struct
GameStateResponse
{
	Fields:                            Vec<NetworkField>,
	Players:                           Vec<PlayerListElement>,
	Message:                           String,
	Gain:                              Vec<NetworkPlayerToken>,
	Lost:                              Vec<NetworkPlayerToken>,
}


impl
GameStateResponse
{

	pub fn
	new
	(
		fields:                        Vec<NetworkField>,
		players:                       Vec<PlayerListElement>,
		message:                       String,
		points_gain:                   Vec<NetworkPlayerToken>,
		points_lost:                   Vec<NetworkPlayerToken>,
	)
	-> Self
	{
		GameStateResponse
		{
			Fields:                    fields,
			Players:                   players,
			Message:                   message,
			Gain:                      points_gain,
			Lost:                      points_lost,
		}
	}

	pub fn
	empty
	(
		message: String,
	)
	-> Self
	{
		Self::new(Vec::new(), Vec::new(), message, Vec::new(), Vec::new())
	}

	pub fn
	add_field
	(
		&mut self,
		field: NetworkField
	)
	{
		self.Fields.push(field);
	}

	pub fn
	add_player
	(
		&mut self,
		player: PlayerListElement
	)
	{
		self.Players.push(player);
	}

	pub fn
	add_gaining_player
	(
		&mut self,
		player: NetworkPlayerToken
	)
	{
		self.Gain.push(player);
	}

	pub fn
	add_losing_player
	(
		&mut self,
		player: NetworkPlayerToken
	)
	{
		self.Lost.push(player);
	}
}
