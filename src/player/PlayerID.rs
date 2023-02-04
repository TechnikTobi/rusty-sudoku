use std::{sync::atomic::{AtomicUsize, Ordering}};

#[derive(PartialEq, Eq, Hash)]
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

	pub fn as_str (&self) -> String
	{ self.value.to_string() }
}