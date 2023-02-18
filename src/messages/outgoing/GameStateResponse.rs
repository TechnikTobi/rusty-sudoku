use serde::Serialize;

use crate::messages::base::NetworkGameState::NetworkGameState;
use crate::messages::base::PlayerListElement::PlayerListElement;
use crate::messages::base::NetworkField::NetworkField;

#[derive(Serialize, Debug, Clone)]
pub struct
GameStateResponse
{
	State: NetworkGameState,
	Fields: Vec<NetworkField>,
	Players: Vec<PlayerListElement>,
	Message: String,
}


impl
GameStateResponse
{

	pub fn
	new
	(
		state: NetworkGameState,
		fields: Vec<NetworkField>,
		players: Vec<PlayerListElement>,
		message: String,
	)
	-> Self
	{
		GameStateResponse
		{
			State: state,
			Fields: fields,
			Players: players,
			Message: message,
		}
	}

	pub fn
	empty
	(
		message: String,
	)
	-> Self
	{
		Self::new(NetworkGameState::UNDEFINED, Vec::new(), Vec::new(), message)
	}

	pub fn
	set_state
	(
		&mut self,
		state: NetworkGameState
	)
	{
		self.State = state;
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
}
