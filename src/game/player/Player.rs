use crate::{game::player::PlayerID::PlayerID, color::Color::Color, messages::base::PlayerListElement::PlayerListElement};

#[derive(Debug)]
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

	pub fn
	to_network
	(
		&self,
		points: u64
	)
	-> PlayerListElement
	{
		PlayerListElement::new(
			self.name.to_owned(), 
			self.color.get_hex_string(),
			points
		)
	}
}