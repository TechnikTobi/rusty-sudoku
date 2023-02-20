pub fn
hsv_to_rgb
(
	h: f64,
	s: f64,
	v: f64
)
-> (u8, u8, u8)
{

	let M = 255.0 * v;
	let m = M * (1.0 - s);

	let z = (M - m) * (1.0 - (float_mod_2(h / 60.0) - 1.0).abs());

	if 0.0 <= h && h < 60.0
	{
		return (
			M as u8, 
			(z+m) as u8, 
			m as u8
		);
	}
	else if 60.0 <= h && h < 120.0
	{
		return (
			(z+m) as u8, 
			M as u8, 
			m as u8
		);
	}
	else if 120.0 <= h && h < 180.0
	{
		return (
			m as u8, 
			M as u8, 
			(z+m) as u8
		);
	}
	else if 180.0 <= h && h < 240.0
	{
		return (
			m as u8, 
			(z+m) as u8, 
			M as u8
		);
	}
	else if 240.0 <= h && h < 300.0
	{
		return (
			(z+m) as u8, 
			m as u8, 
			M as u8
		);
	}
	else if 300.0 <= h && h < 360.0
	{
		return (
			M as u8, 
			m as u8, 
			(z+m) as u8
		);
	}
	else
	{
		println!("Could not convert! Ahhh!");
		println!("{} {} {}", h, s, v);
		return (0, 0, 0);
	}
}

fn
float_mod_2
(
	mut value: f64
)
-> f64
{
	while value < 0.0
	{
		value += 2.0;
	}

	while value >= 2.0
	{
		value -= 2.0;
	}

	assert!(value >= 0.0);
	assert!(value < 2.0);

	return value;
}