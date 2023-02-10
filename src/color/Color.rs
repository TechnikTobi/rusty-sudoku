use rand::Rng;


#[derive(Copy, Clone, Debug)]
pub struct
Color
{
	red: u8,
	green: u8,
	blue: u8
}

impl Color
{
	/// Constructor for creating a new struct with given RGB values
	pub fn
	new
	(
		red: u8,
		green: u8,
		blue: u8
	)
	-> Self
	{
		Color { red, green, blue }
	}

	/// Creates a new struct with the default color: black
	pub fn
	get_default_color()
	-> Self
	{
		Color { red: 0, green: 0, blue: 0 }	
	}

	pub fn
	new_random_color()
	-> Self
	{
		let hue = rand::random::<f64>();
		let saturation = 0.6;
		let value = 0.8;

		Color { red: 0, green: 0, blue: 0 }	
	}

	/// Gets the color as a string containing the hex values of the red, green
	/// and blue channels
	pub fn
	get_hex_string
	(
		&self
	)
	-> String
	{
		format!("{:X}{:X}{:X}", self.red, self.green, self.blue)
	}

}