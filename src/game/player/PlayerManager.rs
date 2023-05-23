use std::collections::HashMap;

use super::PlayerID::PlayerID;
use super::Player::Player;

pub struct
PlayerManager
{
	players: HashMap<PlayerID, Player>
}

impl
PlayerManager
{
	pub fn new() -> Self { PlayerManager { players: HashMap::new() }}

	pub fn
	add_player
	(
		&mut self,
		name: String
	)
	-> PlayerID
	{
		let new_player = Player::new(name.trim().to_string());
		let new_player_id = new_player.get_player_id().clone();
		self.players.insert(new_player_id.clone(), new_player);
		self.cleanup_players();
		return new_player_id;
	}

	fn
	cleanup_players
	(
		&mut self
	)
	{
		let PLAYER_AGE_LIMIT = 86400;
		self.players.retain(|_, player| player.get_age() < PLAYER_AGE_LIMIT);
	}


	pub fn
	get_player
	(
		&self,
		id: &PlayerID
	)
	-> Option<&Player>
	{
		return self.players.get(id)
	}
}