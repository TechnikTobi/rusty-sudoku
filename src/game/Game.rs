use crate::board::difficulty::Difficulty;
use crate::game::GameID::GameID;
use crate::board::board::Board;

use super::EGameState::EGameState;

pub struct
Game
{
	id: GameID,
	name: String,
	difficulty: u8,
	state: EGameState,
}

impl
Game
{
	pub fn
	new
	(
		name: String,
		difficulty: u8
	)
	-> Self
	{
		Game
		{
			id: GameID::new(),
			name: name,
			difficulty: Difficulty::bound_difficulty(difficulty),
			state: EGameState::READY
		}
	}

	pub fn get_game_id    (&self) -> &GameID     { &self.id }
	pub fn get_name       (&self) -> &String     { &self.name }
	pub fn get_difficulty (&self) -> &u8         { &self.difficulty }
	pub fn get_state      (&self) -> &EGameState { &self.state }

	pub fn 
	set_state
	(
		&mut self, 
		new_state: EGameState
	)
	{
		self.state = new_state;
	}

}