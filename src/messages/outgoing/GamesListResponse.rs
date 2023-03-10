use serde::Serialize;

use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;

use super::IResponse::IResponse;

#[derive(Serialize, Debug, Clone)]
pub struct
GamesListElement
{
	GameID: NetworkGameIdentifier,
	GameName: String,
	CreatorName: String,
	Difficulty: u8,
	TotalPlayers: u8,
}

#[derive(Serialize, Debug, Clone)]
pub struct
GamesListResponse
{
	Games: Vec<GamesListElement>,
	Message: String,
}

impl
GamesListElement
{
	pub fn
	new
	(
		game_id: NetworkGameIdentifier,
		game_name: String,
		creator_name: String,
		difficulty: u8,
		total_players: u8,
	)
	-> Self
	{
		GamesListElement 
		{ 
			GameID: game_id, 
			GameName: game_name, 
			CreatorName: creator_name, 
			Difficulty: difficulty, 
			TotalPlayers: total_players,
		}
	}
}

impl
IResponse
for
GamesListResponse
{
	fn get_message(&self) -> &String { &self.Message }
}

impl
GamesListResponse
{
	pub fn
	new
	(
		message: String
	)
	-> Self
	{
		GamesListResponse 
		{ 
			Games: Vec::new(),
			Message: message,
		}
	}

	pub fn
	add_item
	(
		&mut self,
		item: GamesListElement
	)
	{
		self.Games.push(item);
	}
}