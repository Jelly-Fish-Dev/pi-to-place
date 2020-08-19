use std::str::FromStr;
use std::fs;


fn main(){
	let points: usize;
	let args: Vec<std::string::String> = std::env::args().collect();

	points = usize::from_str(&args[1])
		.expect("USEAGE: pi-to-point [Number] [pi.txt location]");

	let filename = &args[2];

	if points == 0{
		std::process::exit(1);
	}

	let pi = fs::read_to_string(filename).expect("cannot read pi.txt");
	println!("{}", &pi[0..points+2]);
}
