use std::time::Instant;

use crate::{game::player::PlayerID::PlayerID, color::Color::Color, messages::base::PlayerListElement::PlayerListElement};

use super::PlayerToken::PlayerToken;

#[derive(Debug, Clone)]
pub struct
Player
{
	id:                                PlayerID,
	token:                             PlayerToken,
	color:                             Color,
	name:                              String,
	creation_time:                     Instant,
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
			id:                        PlayerID::new(), 
			token:                     PlayerToken::new(),
			color:                     Color::new_random_color(), 
			name:                      name,
			creation_time:             Instant::now()
		}
	}

	pub fn get_player_id (&self) -> &PlayerID    { &self.id }
	pub fn get_token     (&self) -> &PlayerToken { &self.token }
	pub fn get_color     (&self) -> &Color       { &self.color }
	pub fn get_name      (&self) -> &String      { &self.name }
	pub fn get_age       (&self) -> u64          { self.creation_time.elapsed().as_secs() }

	pub fn
	to_network
	(
		&self,
		points: i64
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