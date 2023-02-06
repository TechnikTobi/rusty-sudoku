use std::{sync::atomic::{AtomicUsize, Ordering}};

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

	pub fn as_str (&self) -> String
	{ self.value.to_string() }
}