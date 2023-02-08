use std::{sync::atomic::{AtomicUsize, Ordering}};

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct
PlayerID
{
	value: usize
}

static PlayerIDcounter: AtomicUsize = AtomicUsize::new(1);

impl PlayerID
{
	pub fn new() -> Self
	{ PlayerID { value: PlayerIDcounter.fetch_add(1, Ordering::Relaxed) } }

	pub fn
	from_network
	(
		network_id: NetworkPlayerIdentifier
	)
	-> Self
	{
		PlayerID
		{
			value: network_id.get_value().clone()
		}
	}

	pub fn as_str (&self) -> String
	{ self.value.to_string() }
}