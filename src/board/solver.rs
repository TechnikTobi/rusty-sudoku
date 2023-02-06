use super::field::Field;
use super::board::Board;
use super::validation::validate::validate;

impl Board
{
	pub fn
	solve
	(
		&mut self
	)
	-> u64
	{
		self.recursive_solve()
	}

	fn
	recursive_solve
	(
		&mut self
	)
	-> u64
	{
		// If the board is invalid, return 0
		if !validate(self) {return 0;}

		// If it is already full (i.e. no more fields to fill) return 1
		if self.is_full() {return 1;}

		// Find an empty field
		let position_of_empty_field = self.get_fields()
			.iter()
			.find(|field| field.get_value() == Field::EMPTY_FIELD_VALUE)
			.unwrap()
			.get_position();

		let mut solutions = 0;

		for value in Self::MIN_VALUE..(Self::MAX_VALUE+1)
		{
			if 
				self.get_row(position_of_empty_field).iter().any(|field| field.get_value() == value) ||
				self.get_column(position_of_empty_field).iter().any(|field| field.get_value() == value) ||
				self.get_square(position_of_empty_field).iter().any(|field| field.get_value() == value)
			{continue;}

			// If all these checks pass, set the field accordingly and continue
			// solving the board by making a recursive call to solve the next
			// empty field
			self.get_mut_field(position_of_empty_field).unwrap().set_value(value);
			solutions += self.recursive_solve();

			// Reset the value of the field
			self.get_mut_field(position_of_empty_field).unwrap().set_value(Field::EMPTY_FIELD_VALUE);
		}

		return solutions;
	}
}