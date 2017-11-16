use std::rc::Rc;
use std::cell::RefCell;
use parser_module::parser::Parser;
use parser_module::parser;
use colored::*;

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
	// pub op: Operator,
	pub operator: String,
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
	pub find: bool,
	pub classe: Types,
	pub edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
	pub fn new( name: String, classe: Types) -> Rc<RefCell<Node>> {
		Rc::new(RefCell::new(Node {
			name: name,
			find: false,
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
					panic!("ERROR: the same elem is as Fact and Rule");
				}
			}
		}
		// println!("self {:?}", self);
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
				let tmp = Node::new(String::from("Operator") , Types::Rul( Rule{ operator: rule.to_string().clone() } ));
				self.constructor(&mut operator_stack, tmp.clone());
				operator_stack.push(tmp.clone());
			}
		}
		return operator_stack[0].clone();
	}

	pub fn init_rules(&mut self, elem: &parser::Node) {
		let new_rule = self.gen_rule(&elem.rules);

		for fact in elem.facts.chars() {
			if fact.is_alphabetic() {
				let current_fact = self.get_node_by_name(fact.to_string());
				current_fact.borrow_mut().edges.push(new_rule.clone());
			}
		}
	}

	pub fn generate(&mut self, data : &mut Parser) {
		for elem in data.node.iter() {
			self.init_facts(elem); // can be change
			self.init_rules(elem);
		}

		self.start_node(data);
	}

	fn test_fact(&self, fact: &mut Fact, v: i32, inv: i32) -> State
	{
		// println!("fact {:?} v {}", fact, v);
		if (v > 0 && inv == 0) || (fact.valid && !fact.invalid) {
			fact.valid = true;
			return State::Valid;
		} else if (inv > 0 && v == 0) || (!fact.valid && fact.invalid) {
			fact.invalid = true;
			return State::Invalid;
		} else if (inv > 0 && v > 0) || (fact.valid && fact.invalid){
			fact.valid = true;
			fact.invalid = true;
			return State::Undefined;
		}
		return State::None;
	}

	fn test_rul(&self, rule: &String, v: i32) -> State {

		if "+" == rule {
			if v == 2 {	return State::Valid; } else { return State::Invalid; }
		} else if "|" == rule {
			if v > 0 { return State::Valid; } else { return State::Invalid; }
		} else if "^" == rule {
			if v == 1 { return State::Valid; } else { return State::Invalid; }
		} else if "!" == rule {
			if v == 1 { return State::Invalid; } else { return State::Valid; }
		}

		return State::None;
	}

	fn test(&self, node: Rc<RefCell<Node>>, v: i32, inv: i32) -> State {

		match node.borrow_mut().classe {
			Types::Fac(ref mut fac) => return self.test_fact(fac, v, inv),
			Types::Rul(ref rul) => return self.test_rul(&rul.operator, v),
			Types::None => panic!("Error empty Node"),
		}
	}

	fn find_the_truth(&self, head: Rc<RefCell<Node>>, index: i32, old_stack: Vec<String>) -> State {
		let mut valid: i32 = 0;
		let mut invalid: i32 = 0;
		let mut stack: Vec<String> = old_stack.clone();

		if head.borrow().name.len() == 1 {
			stack.push(head.borrow().name.clone());
		}

		for node in head.borrow().edges.iter() {
			if !stack.contains(&node.borrow().name) {
				let state = self.find_the_truth(node.clone(), index + 1, stack.clone());
				if head.borrow().name.len() == 1 {
					stack.push(node.borrow().name.clone());
				}

				if let State::Valid = state {
					valid += 1;
				} else if let State::Invalid = state {
					invalid += 1;
				} else if let State::Undefined = state {
					println!("Help");
					invalid += 1;
					valid += 1;
				}
			}
			// else {
			// 	return self.test(head.clone(), 0, 0);
			// }
		}
		return self.test(head.clone(), valid, invalid);
	}

	pub fn search_in_graph(&mut self, elem: &String)
	{
		for node in self.edges.iter() {
			if node.borrow().name == *elem {
				// println!("name {}", elem);
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

		for elem in self.edges.iter() {
			if data.val_search.contains(&elem.borrow().name) {
				self.print_node(elem, 3);
			}
		}
	}

	pub fn show_fact(&self, elem: &Fact)
	{
		if elem.valid == true {
			print!("F: {} ", elem.name.green());
		} else {
			print!("F: {} ", elem.name.red());
		}
	}

	pub fn print_node(&self, node:&Rc<RefCell<Node>>, depth:usize)
	{
		let elem = node.borrow();
		if elem.edges.len() > 0 {
			if depth == 3 {
				// println!("");
				if let Types::Fac(ref Fact) = elem.classe {
					self.show_fact(Fact);
					println!("<=");
				}
			}

			for child in elem.edges.iter() {
				match child.borrow().classe {
					Types::Fac(ref Fact) => {
						print!("{:>width$}", "",width=(3*depth));
						self.show_fact(Fact);
						println!("");
					},
					Types::Rul(ref Rule) => {
						// println!("");
						print!{"{n:>width$}",n="", width=(3* depth)};
						println!("Op [{}]", Rule.operator);
						self.print_node(&child, depth + 1);
					},
					Types::None => {
					}
				}
			}
		}
	}
}
