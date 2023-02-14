use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct
GameStateResponse
{
	Fields: Vec<NetworkField>,
	Players: Vec<PlayerListElement>,
	Message: String,
}