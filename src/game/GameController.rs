use std::collections::HashMap;
use std::hash::Hash;

use crate::game::Game::Game;
use crate::game::player::PlayerID::PlayerID;
use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::outgoing::GameStateResponse::GameStateResponse;

use super::BoardManager::BoardManager;
use super::EGameState::*;
use super::player::PlayerManager::PlayerManager;

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
	toggle_player
	(
		&mut self,
		new_player: PlayerID
	)
	{
		if !self.is_joinable()
		{
			return;
		}

		if self.points.contains_key(&new_player)
		{
			self.points.remove(&new_player);
		}
		else
		{
			self.points.insert(new_player, Self::POINTS_UNREADY);
		}
	}

	pub fn
	to_network
	(
		&self,
		player_manager: &PlayerManager,
		message: String

	)
	-> GameStateResponse
	{
		let mut state = GameStateResponse::empty(message);

		for field in self.board_manager.get_play_board().get_fields()
		{
			state.add_field(field.to_network());
		}

		for (player_id, points) in &self.points
		{
			if let Some(player) = player_manager.get_player(&player_id)
			{
				state.add_player(player.to_network(points.to_owned()));
			}
			
		}

		return state;
	}

	pub fn
	get_player_id_list
	(
		&self
	)
	-> Vec<NetworkPlayerIdentifier>
	{
		self.points
			.iter()
			.map(|(id, _)| id.to_network())
			.collect::<Vec<NetworkPlayerIdentifier>>()
	}

}