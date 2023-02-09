use std::{collections::HashMap, hash::Hash};

use super::{GameID::GameID, GameController::GameController, player::PlayerID::PlayerID};

pub struct
GameControllerManager
{
	games: HashMap<GameID, GameController>
}

impl
GameControllerManager
{

	pub fn
	new() -> Self
	{
		GameControllerManager 
		{ 
			games: HashMap::new()
		}
	}

	pub fn
	create_game
	(
		&mut self,
		master: PlayerID,
		name: String,
		difficulty: u8
	)
	-> GameID
	{
		let new_game_controller = GameController::new(master, name, difficulty);
		let return_id = new_game_controller.get_game().get_game_id().clone();
		self.games.insert(return_id, new_game_controller);
		return return_id;
	}

	pub fn
	get_game
	(
		&self,
		id: &GameID
	)
	-> Option<&GameController>
	{
		self.games.get(id)
	}

	pub fn
	get_mut_game
	(
		&mut self,
		id: &GameID
	)
	-> Option<&mut GameController>
	{
		self.games.get_mut(id)
	}

	pub fn
	get_iter
	(
		&self
	)
	-> std::collections::hash_map::Iter<'_, GameID, GameController>
	{
		self.games.iter()
	}

}