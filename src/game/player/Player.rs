use crate::{game::player::PlayerID::PlayerID, color::Color};

pub struct
Player
{
	id: PlayerID,
	color: Color,
	name: String,
}

impl
Player
{
	pub fn
	new
	(
		name: String
	)
	-> Self
	{
		Player 
		{ 
			id: PlayerID::new(), 
			color: Color::new_random_color(), 
			name: name 
		}
	}

	pub fn get_player_id (&self) -> &PlayerID { &self.id }
	pub fn get_color     (&self) -> &Color    { &self.color }
	pub fn get_name      (&self) -> &String   { &self.name }
}