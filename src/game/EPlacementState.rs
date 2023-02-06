/// Describes the result of trying to set the value of a field by a player 
/// and the amount of poiints they get for this action
pub enum
EPlacementState
{
	CORRECT,
	INCORRECT,
	INVALID
}

impl
EPlacementState
{
	/// Gets the amount of points the player receives for their action
	pub fn
	points
	(
		&self
	)
	-> i64
	{
		match self
		{
			EPlacementState::CORRECT => 100,
			EPlacementState::INCORRECT => -100,
			EPlacementState::INVALID => 0
		}
	}
}