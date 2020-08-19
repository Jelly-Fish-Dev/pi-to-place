use std::str::FromStr;
use std::fs;


fn main(){

	//Grab the args
	let args: Vec<std::string::String> = std::env::args().collect();

	//pull number of points from args
	let points: usize;
	points = usize::from_str(&args[1])
		.expect("USEAGE: pi-to-point [Number] [pi.txt location]");

	//pull filename from args
	let filename = &args[2];

	//if no points just exit out 
	if points == 0{
		std::process::exit(1);
	}

	//check if pi.txt even works
	let pi = fs::read_to_string(filename).expect("cannot read pi.txt");
	//finally print the points
	println!("{}", &pi[0..points+2]);
}
