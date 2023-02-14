use serde::Serialize;

use crate::messages::base::PlayerListElement::PlayerListElement;

#[derive(Serialize, Debug, Clone)]
pub struct
GameStateResponse
{
	Fields: Vec<NetworkField>,
	Players: Vec<PlayerListElement>,
	Message: String,
}