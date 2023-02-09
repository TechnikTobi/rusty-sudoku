use serde::Serialize;

use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;

use super::IResponse::IResponse;

#[derive(Serialize)]
pub struct
GamesListElement
{
	GameID: NetworkGameIdentifier,
	GameName: String,
	CreatorName: String,
	Difficulty: u8,
	ReadyPlayers: u8,
	TotalPlayers: u8,
}

#[derive(Serialize)]
pub struct
GamesListResponse
{
	Games: Vec<GamesListElement>
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
		ready_players: u8,
		total_players: u8
	)
	-> Self
	{
		GamesListElement 
		{ 
			GameID: game_id, 
			GameName: game_name, 
			CreatorName: creator_name, 
			Difficulty: difficulty, 
			ReadyPlayers: ready_players, 
			TotalPlayers: total_players
		}
	}
}

impl
IResponse
for
GamesListResponse
{}

impl
GamesListResponse
{
	pub fn
	new
	()
	-> Self
	{
		GamesListResponse 
		{ 
			Games: Vec::new() 
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