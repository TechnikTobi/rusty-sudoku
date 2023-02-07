use std::collections::HashMap;

use crate::game::Game::Game;
use crate::game::player::PlayerID::PlayerID;

use super::BoardManager::BoardManager;
use super::EGameState::*;

pub struct
GameController
{
	game: Game,
	board_manager: BoardManager,
	points: HashMap<PlayerID, i64>,
	master_id: PlayerID
}

impl
GameController
{
	const POINTS_UNREADY: i64 = -1;
	const POINTS_READY: i64 = 0;

	pub fn
	new
	(
		master_id: PlayerID,
		name: String,
		difficulty: u8
	)
	-> Self
	{
		GameController 
		{ 
			game: Game::new(name, difficulty), 
			board_manager: BoardManager::new(difficulty), 
			points: HashMap::new(),
			master_id: master_id,
		}
	}

	pub fn get_master_id (&self) -> &PlayerID               { &self.master_id }
	pub fn get_game      (&self) -> &Game                   { &self.game }
	pub fn get_points    (&self) -> &HashMap<PlayerID, i64> { &self.points }
	
	pub fn is_joinable   (&self) -> bool { self.game.get_state() == &EGameState::READY }
	pub fn is_finished   (&self) -> bool { self.game.get_state() == &EGameState::FINISHED }

	pub fn get_mut_game  (&mut self) -> &mut Game { &mut self.game }

	pub fn
	add_player
	(
		&mut self,
		new_player: PlayerID
	)
	{
		if self.is_joinable()
		{
			self.points.insert(new_player, Self::POINTS_UNREADY);
		}
	}

}