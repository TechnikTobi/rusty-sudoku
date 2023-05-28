use std::collections::HashMap;
use std::time::Instant;

use crate::board::field::Field;
use crate::game::Game::Game;
use crate::game::GameID::GameID;
use crate::game::player::PlayerID::PlayerID;
use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::outgoing::GameStateResponse::GameStateResponse;

use super::BoardManager::BoardManager;
use super::EGameState::*;
use super::player::Player::Player;
use super::player::PlayerManager::PlayerManager;
use super::player::PlayerToken::PlayerToken;

pub struct
GameController
{
	game:                              Game,
	board_manager:                     BoardManager,
	points:                            HashMap<PlayerID, i64>,
	master_id:                         PlayerID,
	creation_time:                     Instant,
	players_who_gained:                Vec<PlayerToken>,
	players_who_lost:                  Vec<PlayerToken>
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
			game:                      Game::new(name, difficulty), 
			board_manager:             BoardManager::new(difficulty), 
			points:                    HashMap::new(),
			master_id:                 master_id,
			creation_time:             Instant::now(),
			players_who_gained:        Vec::new(),
			players_who_lost:          Vec::new(),
		}
	}

	pub fn get_master_id       (&self) -> &PlayerID               { &self.master_id }
	pub fn get_game            (&self) -> &Game                   { &self.game }
	pub fn get_age             (&self) -> u64                     { self.creation_time.elapsed().as_secs() }
	
	pub fn is_joinable         (&self) -> bool  { self.game.get_state() == &EGameState::READY }

	pub fn count_total_players (&self) -> usize { self.points.len() }
	pub fn count_ready_players (&self) -> usize { self.points.iter().filter(|(_, points)| *points == &Self::POINTS_READY).count() }

	pub fn
	toggle_player
	(
		&mut self,
		new_player: PlayerID
	)
	-> GameID
	{
		if !self.is_joinable()
		{
			return GameID::empty();
		}

		if self.points.contains_key(&new_player)
		{
			self.points.remove(&new_player);
		}
		else
		{
			self.points.insert(new_player, Self::POINTS_UNREADY);
		}

		return self.get_game().get_game_id().to_owned();
	}

	pub fn
	ready_player
	(
		&mut self,
		player_id: PlayerID
	)
	{
		if !self.points.contains_key(&player_id)
		{
			return;
		}

		self.points.insert(player_id, Self::POINTS_READY);

		if self.count_ready_players() == self.count_total_players()
		{
			self.game.set_state(EGameState::ONGOING);
		}
	}

	pub fn
	to_network
	(
		&self,
		player_manager: &PlayerManager,
		message: String

	)
	-> (GameStateResponse, Vec<NetworkPlayerIdentifier>)
	{
		let mut state = GameStateResponse::empty(message);

		if !self.is_joinable()
		{
			for field in self.board_manager.get_play_board().get_fields()
			{
				state.add_field(field.to_network());
			}
		}

		for (player_id, points) in &self.points
		{
			if let Some(player) = player_manager.get_player(&player_id)
			{
				state.add_player(player.to_network(points.to_owned()));
			}
			
		}

		self.players_who_gained.iter().for_each(|token| state.add_gaining_player(token.to_network()));
		self.players_who_lost.iter().for_each(|token| state.add_losing_player(token.to_network()));
		

		return (state, self.get_player_id_list());
	}

	pub fn
	reset_for_next_to_network
	(
		&mut self
	)
	{
		self.players_who_gained.clear();
		self.players_who_lost.clear();
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

	pub fn
	set_field
	(
		&mut self,
		field: Field,
		player: &Player
	)
	{
		if 
			self.game.get_state() != &EGameState::ONGOING ||
			!self.points.contains_key(player.get_player_id())
		{
			return;
		}

		let new_points = self.board_manager.set_field(
			field.get_position(), 
			field.get_value(),
			player.get_color().to_owned()
		).points() + self.points.get(player.get_player_id()).unwrap();
		
		match self.points.get(player.get_player_id()).unwrap().cmp(&new_points)
		{
			std::cmp::Ordering::Greater => self.players_who_lost.push(player.get_token().clone()),
			std::cmp::Ordering::Equal   => (),
			std::cmp::Ordering::Less    => self.players_who_gained.push(player.get_token().clone()),
		}

		self.points.insert(player.get_player_id().to_owned(), new_points);

		if self.board_manager.get_play_board().is_full()
		{
			self.game.set_state(EGameState::FINISHED);
		}
	}

}