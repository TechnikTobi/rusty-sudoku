/// Describes the different states a game goes through in its lifetime
#[derive(PartialEq, Eq)]
pub enum
EGameState
{
	READY,
	ONGOING,
	FINISHED
}