use super::HSV::hsv_to_rgb;

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
		let hue: f64 = rand::random::<f64>() * 360.0;
		let saturation: f64 = 0.6;
		let value: f64 = 0.8;

		let (red, green, blue) = hsv_to_rgb(hue, saturation, value);

		Color { red: red, green: green, blue: blue }	
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
		format!("{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
	}

}