use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct Node {
	pub rules: String,
	pub facts: String,
}

pub struct Parser {
	pub node: Vec<Node>,
	pub val_init: Vec<String>,
	pub val_search: Vec<String>,
}

impl Parser
{
	fn operation_is_valid(line: &String) -> bool
	{
		for val in line.chars() {
			if !val.is_alphabetic() && !val.is_whitespace() &&
				val != '+' && val != '(' && val != ')' && val != '!' && val != '|' && val != '^'
			{
				println!("Error bad value {} in line {}", val, line);
				return false;
			}
		}
		return true;
	}

	fn declaration_is_valid(&mut self, line: &String, t: char) -> bool
	{
		for (i, elem) in line.chars().enumerate() {
			if i == 0 && elem != t {
				println!("Error bad bormat {}", line);
				return false;
			} else if elem.is_alphabetic() {
				self.val_init.push(elem.to_string().clone());
			} else {
				println!("Error bad bormat {}", line);
				return false;
			}
		}
		return true;
	}

	fn validator(&mut self, raw_file: &mut Vec<String>) -> bool
	{
		for line in raw_file {
			let mut node: Node = Node{ rules: String::from(""), facts: String::from("") };
			let mut node2: Node = Node{ rules: String::from(""), facts: String::from("") };

			if let Some(val) = line.find("<=>") {
				line.split("<>=>").enumerate().for_each(|x|
					if x.0 == 2 {
						println!("Error more than 2 |<=>| in line >  {}", line);
						node.rules = String::from("");
					}
					else if x.0 == 0 && Parser::operation_is_valid(& x.1.to_string())
						{ node.rules = x.1.to_string(); }
					else if x.0 == 1 && Parser::operation_is_valid(& x.1.to_string())
						{ node.facts = x.1.to_string(); }
				);
				if node.rules.len() == 0 || node.facts.len() == 0 {
						return false;
				}
				node2.rules = node.facts.clone();
				node2.facts = node.rules.clone();
				self.node.push(node);
				self.node.push(node2);
			} else if let Some(val) = line.find("=>") {
				line.split("=>").enumerate().for_each(|x|
					if x.0 == 2 {
						println!("Error more than 2 |=>| in line >  {}", line);
						node.rules = String::from("");
					}
					else if x.0 == 0 && Parser::operation_is_valid(& x.1.to_string())
						{ node.rules = x.1.to_string(); }
					else if x.0 == 1 && Parser::operation_is_valid(& x.1.to_string())
						{ node.facts = x.1.to_string(); }
				);
				if node.rules.len() == 0 || node.facts.len() == 0 {
						return false;
				}
				self.node.push(node);
			} else if let Some(val) = line.find("=") {
				if !self.declaration_is_valid(& line, '='){
					println!("Error bad format for line >  {:?}", line);
					return false;
				}
			} else if let Some(val) = line.find("?") {
				if self.declaration_is_valid(& line, '?') {
					println!("Error bad format for line >  {:?}", line);
					return false;
				}
			} else if line.len() != 0 {
				println!("Error bad format for line >  {:?}", line);
				return false;
			}
		}
		return true;
	}

	pub fn parse(&mut self) -> Result<Vec<String>, io::Error>
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
					});
			}
			if self.validator(&mut end) {
				return Ok(end);
			}
			return Err(io::Error::new(io::ErrorKind::Other, "File bad format"));
		}
		Err(io::Error::new(io::ErrorKind::Other, "Need file name"))
	}
}
