use crate::game::GameID::GameID;
use crate::board::board::Board;

pub struct
Game
{
	id: GameID,
	name: String,
	difficulty: u8	
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
			difficulty: Board::bound_difficulty(difficulty)
		}
	}

	pub fn get_game_id (&self) -> &GameID { &self.id }
	pub fn get_name (&self) -> &String { &self.name }
	pub fn get_difficulty ( &self )	-> &u8 { &self.difficulty }

}