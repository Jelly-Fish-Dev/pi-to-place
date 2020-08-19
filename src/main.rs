
use std::str::FromStr;
use std::fs;

fn main() {
    let mut points: usize;
	points = 0;
    //for arg in std::env::args().skip(1){
	//	points = usize::from_str(&arg).expect("USAGE: pi-to-point #");
	//}

	let args: Vec<std::string::String> = std::env::args().collect();
	
	points = usize::from_str(&args[1]).expect("a");
	let filename = &args[2];
							   
	if points == 0{
		std::process::exit(1);
	}

	let pi = fs::read_to_string(filename).expect("cannot read files");
	println!("{}", &pi[0..points+2]);
}
