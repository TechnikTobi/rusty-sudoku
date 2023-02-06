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
		let new_player = Player::new(name);
		let new_player_id = new_player.get_player_id().clone();
		self.players.insert(new_player_id.clone(), new_player);
		return new_player_id;
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