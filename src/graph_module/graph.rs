use std::rc::Rc;
use std::cell::RefCell;
use parser_module::parser::Parser;
use parser_module::parser;
use colored::*;
use std::process;

#[derive(Debug)]
pub enum Types {
	Fac(Fact),
	Rul(Rule),
	None,
}

#[derive(Debug)]
pub enum State {
	Valid,
	Invalid,
	Undefined,
	None,
}

#[derive(Debug)]
pub struct Rule {
	pub operator: String,
	pub state: State,
}

#[derive(Debug)]
pub struct Fact {
	pub name: String,
	pub valid: bool,
	pub invalid: bool,
}

#[derive(Debug)]
pub struct Node {
	pub name: String,
	pub classe: Types,
	pub edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
	pub fn new( name: String, classe: Types) -> Rc<RefCell<Node>> {
		Rc::new(RefCell::new(Node {
			name: name,
			classe: classe,
			edges: Vec::new(),
		}))
	}

	pub fn is_not_in_edges(&self, nodes: &Vec<Rc<RefCell<Node>>>, name: &char) -> bool {
		// println!("search for {:?}", name);
		for node in nodes.iter() {
			// println!("name > {:?}", node.borrow().name);
			if node.borrow().name == name.to_string() {
				return false;
			}
		}
		return true;
	}

	pub fn get_node_by_name(&self, name: String) -> Rc<RefCell<Node>> {

		for node in self.edges.iter() {
			if node.borrow().name == name {
				return node.clone();
			}
		}
		return self.edges[0].clone();
	}

	pub fn init_facts(&mut self, elem: &parser::Node) {
		for item in elem.rules.chars() {
			if item.is_alphabetic() && self.is_not_in_edges(&self.edges, &item) {

				let tmp = Node::new(item.to_string().clone(), Types::Fac(Fact{name: item.to_string().clone(), valid: false, invalid: false,}) );
				self.edges.push(tmp.clone());
			}
		}

		for item in elem.facts.chars() {
			if item.is_alphabetic() && self.is_not_in_edges(&self.edges, &item) {
				let tmp = Node::new(item.to_string().clone(), Types::Fac(Fact{name: item.to_string().clone(), valid: false, invalid: false,}) );
				self.edges.push(tmp.clone());
			}
		}

		for fac in elem.facts.chars() {
			for rul in elem.rules.chars() {
				if fac.is_alphabetic() && rul.is_alphabetic() && rul == fac{
					println!("ERROR: the same elem is as Fact and Rule");
					process::exit(1);
				}
			}
		}
	}

	pub fn constructor(&mut self, mut stack: &mut Vec<Rc<RefCell<Node>>> , new_node: Rc<RefCell<Node>>) {
		let mut operator: String = "".to_string();

		if let Some(node) = stack.pop() {
			if let Types::Rul(ref rul) = node.borrow().classe {
				operator = rul.operator.clone();
			}

			if (operator == "!" && node.borrow().edges.len() < 1) || (operator != "!" && node.borrow().edges.len() < 2) {
				node.borrow_mut().edges.push(new_node.clone());
			} else {
				self.constructor(&mut stack, new_node.clone());
			}
			stack.push(node);
		} else {
			stack.push(new_node.clone());
		}
	}

	pub fn gen_rule(&mut self, elem: &String) -> Rc<RefCell<Node>> {
		let mut operator_stack: Vec<Rc<RefCell<Node>>> = vec![];

		for rule in elem.chars().rev() {
			if rule.is_alphabetic() {
				let node = self.get_node_by_name(rule.to_string());
				self.constructor(&mut operator_stack, node);
			} else {
				let tmp = Node::new(String::from("Operator") , Types::Rul( Rule{ operator: rule.to_string().clone(), state: State::None } ));
				self.constructor(&mut operator_stack, tmp.clone());
				operator_stack.push(tmp.clone());
			}
		}
		return operator_stack[0].clone();
	}

	pub fn init_rules(&mut self, elem: &parser::Node) {
		let new_rule = self.gen_rule(&elem.rules);
		let mut negatif: bool = false;

		for fact in elem.facts.chars().rev() {
			if fact == '!' { negatif = true; }
			if fact.is_alphabetic() {
				let current_fact = self.get_node_by_name(fact.to_string());
				if negatif == true {
					let negative_node = Node::new(String::from("Operator") , Types::Rul( Rule{ operator: "-".to_string() , state: State::Valid } ));
					negative_node.borrow_mut().edges.push(new_rule.clone());
					current_fact.borrow_mut().edges.push(negative_node);
					negatif = false;
				}
				else {
					current_fact.borrow_mut().edges.push(new_rule.clone());
				}
			}
		}
	}

	pub fn generate(&mut self, data : &mut Parser) {
		for elem in data.node.iter() {
			self.init_facts(elem); // can be change
			self.init_rules(elem);
		}

		for val in data.val_init.iter() {
			if self.is_not_in_edges(&self.edges, &val.chars().nth(0).unwrap() ) {
				let tmp = Node::new(val.clone() , Types::Fac(Fact{name: val.clone(), valid: true, invalid: false} ) );
				self.edges.push(tmp);
			}
		}

		for val in data.val_search.iter() {
			if self.is_not_in_edges(&self.edges, &val.chars().nth(0).unwrap() ) {
				let tmp = Node::new(val.clone() , Types::Fac(Fact{name: val.clone(), valid: false, invalid: true} ) );
				self.edges.push(tmp);
			}
		}

		// let tmp = Node::new(String::from("Operator") , Types::Rul( Rule{ operator: rule.to_string().clone(), state: State::None } ));


		self.start_node(data);
	}

	fn test_fact(&self, fact: &mut Fact, v: i32, inv: i32) -> State
	{
		if (v > 0 && inv == 0) || (fact.valid && !fact.invalid) {
			fact.valid = true;
			return State::Valid;
		} else if (inv > 0 && v == 0) || (!fact.valid && fact.invalid) {
			fact.invalid = true;
			return State::Invalid;
		} else if (inv > 0 && v > 0) || (fact.valid && fact.invalid) {
			fact.valid = true;
			fact.invalid = true;
			return State::Undefined;
		}
		return State::None;
	}

	fn test_rul(&self, rule: &String, v: i32, inv: i32) -> State {

		if "+" == rule {
			if v == 2 && inv == 0 { return State::Valid; }
			else if v + inv > 2{ return State::Undefined; }
			// else { return State::Invalid; }
		} else if "|" == rule {
			if v > 0 && inv < 2 { return State::Valid; }
			if v == 2 && inv == 2 { return State::Undefined; }
			// else { return State::Invalid; }
		} else if "^" == rule {
			if v == 1 && inv == 0 { return State::Valid; }
			else if v > 0 && inv > 0 { return State::Undefined; }
			// else { return State::Invalid; }
		} else if "!" == rule {
			if v == 1 && inv == 0{ return State::Invalid; }
			else if v > 0 && inv > 0{ return State::Undefined; }
			else { return State::Valid; }
		} else if "-" == rule {
			return State::Invalid;
		}

		return State::None;
	}

	fn test(&self, node: Rc<RefCell<Node>>, v: i32, inv: i32) -> State {

		match node.borrow_mut().classe {
			Types::Fac(ref mut fac) => return self.test_fact(fac, v, inv),
			Types::Rul(ref mut rul) => {
				rul.state = self.test_rul(&rul.operator, v, inv);
				return self.test_rul(&rul.operator, v, inv);
			},
			Types::None => {
				println!("Error empty Node");
				process::exit(1);
			},
		}
	}

	fn get_node_state(&self, node: Rc<RefCell<Node>>) -> State
	{
		match node.borrow().classe {
			Types::Fac(ref fac) => {
				if fac.valid == true {
					return State::Valid;
				} else {
					return State::Invalid;
				}
				// else if fac.invalid == true {
				// 	return State::Invalid;
				// }
				// else (undefined)
			},
			Types::Rul(ref rul) => {
				if let State::Valid = rul.state {
					return State::Valid;
				} else if let State::Invalid = rul.state {
					return State::Invalid;
				} else {
					return State::Undefined;
				}
			},
			Types::None => {
				println!("Error empty Node");
				process::exit(1);
			},
		}
	}

	fn find_the_truth(&self, head: Rc<RefCell<Node>>, index: i32, old_stack: Vec<String>) -> State {
		let mut valid: i32 = 0;
		let mut invalid: i32 = 0;
		let mut stack: Vec<String> = old_stack.clone();
		let mut state: State;

		if head.borrow().name.len() == 1 {
			stack.push(head.borrow().name.clone());
		}

		for node in head.borrow().edges.iter() {

			if !stack.contains(&node.borrow().name) {

				state = self.find_the_truth(node.clone(), index + 1, stack.clone());

				if node.borrow().name.len() == 1  {
					// println!("is alpha {}", head.borrow().name);
					stack.push(node.borrow().name.clone());
				}
				// else {
				// 	println!("not alpha {}", node.borrow().name);
				// }

			}
			else {
				// println!("name {:?}", node.borrow().name);
				state = self.get_node_state(node.clone());
				// println!("state {:?}", state);
			}

			// println!("----------->  {:?}", stack);
			if let State::Valid = state {
				valid += 1;
			} else if let State::Invalid = state {
				invalid += 1;
			} else if let State::Undefined = state {
				// println!("Help");
				invalid += 1;
				valid += 1;
			}
		}
		return self.test(head.clone(), valid, invalid);
	}

	pub fn search_in_graph(&mut self, elem: &String)
	{
		for node in self.edges.iter() {
			if node.borrow().name == *elem {
				let stack: Vec<String> = vec![];
				self.find_the_truth(node.clone(), 0, stack.clone());
			}
		}
	}

	pub fn start_node(&mut self, data : &mut Parser)
	{
		//println!("len {:?}", self.edges.len());
		//println!("data |-:|.|:-| {:?} --- {:?} ", data.val_init , data.val_search);
		// init as true
		for elem in self.edges.iter() {
			if data.val_init.contains(&elem.borrow().name) {
				if let Types::Fac(ref mut fac) = elem.borrow_mut().classe {
					fac.valid = true;
				}
				// println!("value {:?} --- ", elem.borrow().name );
			}
			// println!("elem {:?}", elem );
		}

		for value in data.val_search.iter() {
			self.search_in_graph(value);
		}

		// for elem in self.edges.iter() {
		// 	if data.val_search.contains(&elem.borrow().name) {
		// 		self.print_node(elem, 0);
		// 	}
		// }
	}

	pub fn show_fact(&self, elem: &Fact)
	{
		// println!("v {:?} , f {:?}", elem.valid,  elem.invalid);
		if elem.valid == true  && elem.invalid == false {
			print!("{}", elem.name.green());
		} else if elem.valid == true && elem.invalid == true {
			print!("{}", elem.name.blue());
		} else {
			print!("{}", elem.name.red());
		}
	}

	pub fn print_node(&self, node:&Rc<RefCell<Node>>, depth:usize)
	{
		let elem = node.borrow();
		if depth == 0 {
			if let Types::Fac(ref fact) = elem.classe {
				self.show_fact(fact);
				println!("╗ {:?}", "root");
			}
		}
		let mut ni = 0;
		for child in elem.edges.iter() {
			print!(" ");
			match child.borrow().classe {
				Types::Fac(ref fact) => {
					if ni == 0
					{
						print!("{n:<width$}",n="".blue(),width=(1*depth));
						print!("╚");
						print!("╦═");
					}
					else
					{
						print!(" {n:<width$}",n="".blue(),width=(1*depth));
						print!("╚═");
					}
					self.show_fact(fact);
					println!("");
				},
				Types::Rul(ref rule) => {
					print!("{n:<width$}",n="".blue(),width=(1*depth));
					if ni == 0 {
						print!("╚╦═");
					} else {
						if child.borrow().edges.len() > 0 {
							print!(" ╠═");
						} else {
							print!(" ╚═");
						}
					}
					let op:String = String::from("Op");

					match rule.state {
						State::Valid => {
							println!("{}[{}]", op.green(), rule.operator.green());
						},
						State::Invalid => {
							println!("{}[{}]", op.red(), rule.operator.red());
						},
						State::Undefined => {
							println!("{}[{}]", op.cyan(), rule.operator.cyan());
						},
						State::None => {
							println!("{}[{}]", op.red(), rule.operator.red());
						}
					}
					self.print_node(&child, depth + 1);
				},
				Types::None => {
				}
			}
			ni+=1;
		}
	}
}
