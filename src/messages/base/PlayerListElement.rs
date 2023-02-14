use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct
PlayerListElement
{
	Name: String,
	Color: String,
	Points: u64,
}