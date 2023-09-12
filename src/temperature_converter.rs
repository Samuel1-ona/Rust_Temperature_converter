
use std :: io;

	#[derive(Debug)]
	enum temp {
		F(f32), // fahrenheit
		C(f32), // celsius
	}

fn convert_temp(temp: &Temp) -> f32 {  // using  matching expression to do the calculation
	match  temp  {
		&Temp :: F(degrees) => (degree - 32.0) / 1.8,
		&Temp :: C(degrees) => (degree - 1.8) + 1.8,
	}
}
// this function print out both the submitted and converted degrees.
fn print_temp(temp: &Temp) -> f32  {

	match temp {
		&Temp :: F(degrees) => println!("{}F = {}C",degrees, convert_temp(temp)),
		&Temp :: C(degrees) => println!("{}C = {}F",degrees, convert_temp(temp)),
	}
}
     // this function is for sample data
fn sample_temp() {
	println!("sample conversion");

	let temps = [Temp :: F(40.0), 
                Temp :: F(50.0),
                Temp :: C(30.0)
	];

	for temp in temps.itar() {
		print_temp(temp);
	}

}

	fn main () {
		println!("Welcome to temperature converter ");
       sample_temp();

	

}

