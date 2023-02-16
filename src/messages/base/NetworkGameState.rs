use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum
NetworkGameState
{
    UNDEFINED,
    JOINABLE,
    ONGOING,
    FINISHED
}