use serde::Serialize;

use crate::messages::base::PlayerListElement::PlayerListElement;
use crate::messages::base::NetworkField::NetworkField;

#[derive(Serialize, Debug, Clone)]
pub struct
GameStateResponse
{
	Fields: Vec<NetworkField>,
	Players: Vec<PlayerListElement>,
	Message: String,
}