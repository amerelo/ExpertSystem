use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;


pub struct Node {
	Rules: String,
	Facts: String,
}

pub struct Parser {
	node: Vec<Node>,
	val_init: Vec<String>,
	val_search: Vec<String>,
}

impl Parser
{
	fn operation_is_valid(val: &String) {

		// if res == Operations {
		// 	if val.len() == 1 {
		// 		print!(">Ok>|{}|  ", val);
		// 	}
		// 	else {
		// 		print!(">Not Ok>|{}|  ", val);
		// 	}
		// }
	}

	fn validator(raw_file: &mut Vec<String>) {
		for line in raw_file {
			if let Some(val) = line.find("<=>") {

				println!("Super Operations {:?}", line);

			} else if let Some(val) = line.find("=>") {

				println!("Operations {:?}", line);

			} else if let Some(val) = line.find("=") {
				println!("Init Val {:?}", line);
			} else if let Some(val) = line.find("?") {
				println!("Search {:?}", line);
			}
		}
	}

	pub fn parse() -> Result<Vec<String>, io::Error>
	{
		let args: Vec<String> = env::args().collect();
		if args.len() > 1 {
			let mut contents = String::new();
			let mut end: Vec<String> = vec![];
			let mut f = File::open(&args[1])?;

			f.read_to_string(&mut contents)?;
			let v: Vec<&str> = contents.split('\n').collect();
			for lm in v {
				lm.split('#').enumerate().for_each(|x|
					if x.0 == 0 && x.1.len() > 0
					{
						end.push(x.1.trim().clone().to_string())
					})
			}
			// Parser::npi(&mut end);
			Parser::validator(&mut end);
			return Ok(end);
		}
		Err(io::Error::new(io::ErrorKind::Other, "Need file name"))
	}
}
