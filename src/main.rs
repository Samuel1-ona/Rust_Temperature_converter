
use std::io;
    
    enum Temp {
        F(f32), // fahrenheit
        C(f32), // celsius
    }

fn convert_temp(temp: &Temp) -> f32 {  // using  matching expression to do the calculation
    match  temp  {
        &Temp :: F(degrees) => (degrees - 32.0) / 1.8, // expression to convert to F
        &Temp :: C(degrees) => (degrees - 1.8) + 1.8,  // expression to convert to C
    }
}
// this function print out both the submitted and converted degrees.
fn print_temp(temp: &Temp)   {

    match temp {
        &Temp :: F(degrees) => println!("{}F = {}C",degrees, convert_temp(temp)), // converting from F to C
        &Temp :: C(degrees) => println!("{}C = {}F",degrees, convert_temp(temp)), // converting from C to F
    }
}
     // this function is for sample data
fn sample_temp() {
    println!("sample conversion");

    let temps = [Temp :: F(40.0), 
                Temp :: F(50.0),
                Temp :: C(30.0)
    ];

    for temp in temps.iter() {
        print_temp(temp);
    }

}


// user input 

fn get_user_temp() {
    println!(" type quit to end the program"); // Telling the user on how quit the program 

    loop {
        let mut temp_input = String :: new(); // to store the user input 
        println!("please input tempereture you want to convert (format 100F or -40C");

        io::stdin() 
        .read_line(&mut temp_input)
        .expect("failed to read line");

    let trimmed_white_space = temp_input.trim(); // to trim the white space 

    if trimmed_white_space == "quit" {   // check if they want to quit and break the loop
        break; 
        
    }
    let (temp, scale) = trimmed_white_space.split_at(trimmed_white_space.len()-1); // spliting the string in the last character to know whether it is F or C 

    let temp: f32 = match temp.parse() { // to change string to float or intrger
        Ok(num) => num,
        Err(_) => continue,

    };
    let temp: Temp = match scale { // match the last character of the user input
        "C" =>  Temp:: C(temp),
        "F"=>   Temp:: F(temp),
        _ => continue,
    };
    print_temp(&temp);

    }
}

    fn main () {
        println!("Welcome to temperature converter ");
       sample_temp();  // calling the sample data  function
       get_user_temp(); // calling the user function

    

}

