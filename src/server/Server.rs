use crate::game::player::PlayerManager::PlayerManager;
use crate::game::GameControllerManager::GameControllerManager;
use crate::messages::outgoing::GamesListResponse::*;

pub struct
SudokuServer
{
	player_manager: PlayerManager,
	game_controller_manager: GameControllerManager
}

impl
SudokuServer
{
	pub fn
	new
	()
	-> Self
	{
		SudokuServer 
		{ 
			player_manager: PlayerManager::new(),
			game_controller_manager: GameControllerManager::new(),
		}
	}


	pub fn
	get_player_manager
	(
		&self
	)
	-> &PlayerManager
	{
		&self.player_manager
	}

	pub fn
	get_mut_player_manager
	(
		&mut self
	)
	-> &mut PlayerManager
	{
		&mut self.player_manager
	}


	pub fn
	get_mut_game_controller_manager
	(
		&mut self
	)
	-> &mut GameControllerManager
	{
		&mut self.game_controller_manager
	}

	pub fn
	get_game_controller_manager
	(
		&self
	)
	-> &GameControllerManager
	{
		&self.game_controller_manager
	}

	pub fn
	generate_games_list_response
	(
		&self
	)
	-> GamesListResponse
	{
		let mut list = GamesListResponse::new(String::new());

		for (id, controller) in self.game_controller_manager.get_iter()
		{
			let element = GamesListElement::new(
				id.to_network(), 
				controller.get_game().get_name().clone(), 
				self.get_player_manager().get_player(controller.get_master_id()).unwrap().get_name().clone(), 
				controller.get_game().get_difficulty().clone(), 
				controller.count_total_players() as u8,
			);

			list.add_item(element);
		}

		return list
	}

}