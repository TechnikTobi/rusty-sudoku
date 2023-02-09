use std::{sync::atomic::{AtomicUsize, Ordering}};

use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct
GameID
{
	value: usize
}

static GameIDcounter: AtomicUsize = AtomicUsize::new(1);

impl GameID
{
	pub fn new() -> Self
	{ GameID { value: GameIDcounter.fetch_add(1, Ordering::Relaxed) } }

	pub fn
	from_network
	(
		network_id: &NetworkGameIdentifier
	)
	-> Self
	{
		GameID
		{
			value: network_id.get_value().clone()
		}
	}

	pub fn
	to_network
	(
		&self
	)
	-> NetworkGameIdentifier
	{
		NetworkGameIdentifier::new(self.value)
	}

	pub fn as_str (&self) -> String
	{ self.value.to_string() }
}