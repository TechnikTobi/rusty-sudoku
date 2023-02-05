use crate::board::field::*;

#[derive(Clone)]
pub struct
Board
{
	fields: Vec<Field>
}

static MAX_X: u32 = 9;
static MAX_Y: u32 = 9;

impl Board
{

	pub fn
	new()
	-> Self
	{


		let mut fields = Vec::new();
		(0..MAX_Y).for_each(|y| 
			(0..MAX_X).for_each(|x| 
				{ fields.push(Field::new(x, y, 0)); } 
			) 
		);

		Board { fields }
	}

}