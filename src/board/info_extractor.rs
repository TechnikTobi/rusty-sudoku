use std::cmp::Ordering;

use super::position::Position;
use super::field::Field;
use super::board::Board;

impl
Board
{

	pub const SQUARE_SIZE: u8 = 3;

	pub fn
	is_full
	(
		&self
	)
	-> bool
	{
		!self.get_fields().iter().any(|field| field.get_value() == Field::EMPTY_FIELD_VALUE)
	}

	pub fn
	position_is_empty
	(
		&self,
		position: Position
	)
	-> bool
	{
		if !self.get_fields().iter().any(|field| field.get_position() == position)
		{
			return false;
		}

		return self.get_field(position).unwrap().get_value() == Field::EMPTY_FIELD_VALUE;
	}

	pub fn
	get_max_x_pos
	(
		&self
	)
	-> Position
	{
		self.get_fields()
			.iter()
			.map(|field| field.get_position())
			.max_by(|p1, p2|
				if p1.get_x() < p2.get_x() 
				{
					Ordering::Less
				}
				else
				{
					if p1.get_x() == p2.get_x()
					{
						p1.get_y().partial_cmp(&p2.get_y()).unwrap()
					}
					else
					{
						Ordering::Greater
					}
				}
			)
			.unwrap_or(Position::new(0, 0))
	}

	pub fn
	get_max_y_pos
	(
		&self
	)
	-> Position
	{
		self.get_fields()
			.iter()
			.map(|field| field.get_position())
			.max_by(|p1, p2|
				if p1.get_y() < p2.get_y() 
				{
					Ordering::Less
				}
				else
				{
					if p1.get_y() == p2.get_y()
					{
						p1.get_x().partial_cmp(&p2.get_x()).unwrap()
					}
					else
					{
						Ordering::Greater
					}
				}
			)
			.unwrap_or(Position::new(0, 0))
	}

	pub fn
	get_rows
	(
		&self
	)
	-> Vec<Vec<&Field>>
	{
		let mut rows = Vec::new();

		for y in 0..Self::MAX_Y
		{
			rows.push(
				self.get_fields()
					.iter()
					.filter(|field| field.get_position().get_y() == y)
					.collect::<Vec<&Field>>()
			)
		}

		return rows;
	}

	pub fn
	get_row
	(
		&self,
		pos: Position
	)
	-> Vec<&Field>
	{
		self.get_fields()
			.iter()
			.filter(|field| field.get_position().get_x() == pos.get_x())
			.collect::<Vec<&Field>>()
	}

	pub fn
	get_columns
	(
		&self
	)
	-> Vec<Vec<&Field>>
	{
		let mut columns = Vec::new();

		for x in 0..Self::MAX_X
		{
			columns.push(
				self.get_fields()
					.iter()
					.filter(|field| field.get_position().get_x() == x)
					.collect::<Vec<&Field>>()
			)
		}

		return columns;
	}

	pub fn
	get_column
	(
		&self,
		pos: Position
	)
	-> Vec<&Field>
	{
		self.get_fields()
			.iter()
			.filter(|field| field.get_position().get_y() == pos.get_y())
			.collect::<Vec<&Field>>()
	}

	pub fn
	get_squares
	(
		&self
	)
	-> Vec<Vec<&Field>>
	{

		let mut squares = Vec::new();

		for start_y in (0..Self::MAX_Y).step_by(Self::SQUARE_SIZE as usize)
		{
			for start_x in (0..Self::MAX_X).step_by(Self::SQUARE_SIZE as usize)
			{

				squares.push(
					self.get_fields()
						.iter()
						.filter(|field|
							field.get_position().get_x() >= start_x &&
							field.get_position().get_x() < start_x + Self::SQUARE_SIZE &&
							field.get_position().get_y() >= start_y &&
							field.get_position().get_y() < start_y + Self::SQUARE_SIZE
						)
						.collect::<Vec<&Field>>()
				);
			}
		}

		return squares;
	}

	pub fn
	get_square
	(
		&self,
		pos: Position
	)
	-> Vec<&Field>
	{
		self.get_squares()
			.iter()
			.find(|square| square.iter().any(|field| field.get_position() == pos))
			.unwrap()
			.clone()
	}


	#[allow(dead_code)]
	pub fn
	print
	(
		&self,
	)
	{
		for y in 0..Self::MAX_Y
		{
			if y == 0 || y % 3 == 0 {println!("╬═══════╬═══════╬═══════╬")}
			for x in 0..Self::MAX_X
			{
				if x == 0 || x % 3 == 0 {print!("║ ")}
				print!("{} ", self.get_field(Position::new(x, y)).unwrap().get_value());
				if x == Self::MAX_X-1 {print!("║")}
			}
			print!("\n");
			if y == Self::MAX_Y-1 {println!("╬═══════╬═══════╬═══════╬")}
		}
		print!("\n");
	}
}