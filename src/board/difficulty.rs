pub struct
Difficulty
{}

impl
Difficulty
{
	pub const MIN_DIFFICULTY: u8 = 1;
	pub const MAX_DIFFICULTY: u8 = 60;


	pub fn
	bound_difficulty
	(
		given_difficulty: u8
	)
	-> u8
	{
		given_difficulty.min(Self::MAX_DIFFICULTY).max(Self::MIN_DIFFICULTY)
	}
}