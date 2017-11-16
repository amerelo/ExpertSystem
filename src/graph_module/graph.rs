use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use parser_module::parser::Parser;
use parser_module::parser;

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

	// pub fn traverse<F>(&self, f: &F, seen: &mut HashSet<String>)
	// where F: Fn(String)
	// {
	// 	if seen.contains(&self.name) {
	// 		return;
	// 	}
	// 	f(self.name.clone());
	// 	seen.insert(self.name.clone());
	//
	// 	// println!("edges for self >>>  {:?}", self.edges);
	// 	for n in &self.edges {
	// 		// println!("n ? {:?}", n);
	// 		n.borrow().traverse(f, seen);
	// 	}
	// }

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

	// pub fn insert_node(&self, node2: String, node1: String) {
	//
	// 	let mut node_c = self.get_node_by_name(node2);
	// 	let mut node_p = self.get_node_by_name(node1);
	//
	// 	// set struct Fact valid to true
	// 	match node_c.borrow_mut().classe {
	// 		Types::Fac(ref mut fact) => fact.valid = true,
	// 		Types::Rul(ref mut rul) => println!("rul -> {:?}", rul),
	// 		Types::None => println!("None"),
	// 	}
	// 	node_p.borrow_mut().edges.push(node_c.clone());
	// }

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
	}

	// can be changed
	pub fn stack_operator(&mut self, mut stack: &mut Vec<Rc<RefCell<Node>>> , new_node: Rc<RefCell<Node>>) {
		if let Some(node) = stack.pop() {
			if node.borrow().edges.len() < 2 {
				node.borrow_mut().edges.push(new_node.clone());
			} else {
				self.stack_operator(&mut stack, new_node.clone());
			}
			stack.push(node);
		}
		stack.push(new_node);
	}

	// can be changed
	pub fn constructor(&mut self, mut stack: &mut Vec<Rc<RefCell<Node>>> , new_node: Rc<RefCell<Node>>) {

		if let Some(node) = stack.pop() {
			if node.borrow().edges.len() < 2 {
				node.borrow_mut().edges.push(new_node.clone());
			} else {
				self.constructor(&mut stack, new_node.clone());
			}
			stack.push(node);
		}
	}

	pub fn gen_rule(&mut self, elem: &String) -> Rc<RefCell<Node>> {
		println!("rule - - - {:?}", elem);
		let mut operator_stack: Vec<Rc<RefCell<Node>>> = vec![];

		for rule in elem.chars().rev() {
			if rule.is_alphabetic() {
				let node = self.get_node_by_name(rule.to_string());
				self.constructor(&mut operator_stack, node);
			} else {
				let tmp = Node::new(String::from("Operator") , Types::Rul( Rule{ operator: rule.to_string().clone() } ));
				self.stack_operator(&mut operator_stack, tmp.clone());
			}
		}
		// println!("operator_stack - - - > {:?}", operator_stack[0]);
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

	// fn is_fact_true(&self, node: &Node) -> bool {
	//
	// 	if let Types::Fac(ref fact) = node.classe {
	// 		println!("is {:?} valid {:?}", fact.name , fact.valid);
	// 	}
	// 	return true;
	// }

	fn test_fact(&self, fact: &mut Fact, v: i32, inv: i32) -> State
	{

		println!("fact {:?} v {}", fact, v);
		if (v > 0 && inv == 0) || (fact.valid && !fact.invalid) {
			// println!("----------- valid");
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

	fn test_rul(&self, rule: &String, v: i32, inv: i32) -> State {

		println!("rule {} v {}", rule, v);
		if "+" == rule {
			if v == 2 {	return State::Valid; } else { return State::Invalid; }
		} else if "|" == rule {
			if v > 0 { return State::Valid; } else { return State::Invalid; }
		} else if "^" == rule {
			if v == 1 { return State::Valid; } else { return State::Invalid; }
		}
		return State::None;
	}

	fn test(&self, node: &Rc<RefCell<Node>>, v: i32, inv: i32) -> State {

		match node.borrow_mut().classe {
			Types::Fac(ref mut fac) => return self.test_fact(fac, v, inv),
			Types::Rul(ref rul) => return self.test_rul(&rul.operator, v, inv),
			Types::None => panic!("Error empty Node"),
		}
		return State::None;
	}

	fn find_the_truth(&self, head :Rc<RefCell<Node>>, index: i32) -> State {
		let mut valid: i32 = 0;
		let mut invalid: i32 = 0;

		// println!("search of {} ", head.borrow().name);
		for node in head.borrow().edges.iter() {

			println!("item name {:?} --- index {}", node.borrow().name, index);
			let state = self.find_the_truth(node.clone(), index + 1);
			if let State::Valid = state {
				valid += 1;
			} else if let State::Invalid = state {
				invalid += 1;
			} else if let State::Undefined = state {
				println!("Help");
				invalid += 1;
				valid += 1;
			}
			// match self.find_the_truth(node.clone(), index + 1)
			// if let State::Valid
		}
		return self.test(&head, valid, invalid);
	}

	pub fn search_in_graph(&mut self, elem: &String)
	{
		for node in self.edges.iter() {
			if node.borrow().name == *elem {
				println!("search of {} ", elem);
				// self.is_fact_true(&*node.borrow()
				self.find_the_truth(node.clone(), 0);
			}
		}
	}

	pub fn start_node(&mut self, data : &mut Parser)
	{
		println!("len {:?}", self.edges.len());
		println!("data |-:|.|:-| {:?} --- {:?} ", data.val_init , data.val_search);

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
			println!("elem {:?}", elem);
		}

		// self.traverse(&|d| println!("{}", d), &mut HashSet::new());
		// let f = self.first();
		// self.foo(&*f.borrow());
		// let h = self.second();
		// self.foo(&*h.borrow());
	}



	// pub fn first(&self) -> Rc<RefCell<Node>> {
	// 	self.edges[0].clone()
	// }
	// pub fn second(&self) -> Rc<RefCell<Node>> {
	// 	self.edges[1].clone()
	// }

	// pub fn foo(&self, node: &Node) {
	// 	println!("name: {}", node.name);
	// }
}

// pub fn init() -> Rc<RefCell<Node>> {
//
// 	let root = Node::new("A", "ALPHA", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
//
// 	let b = Node::new("B", "Beta", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
// 	let c = Node::new("C", "Charly", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
// 	let d = Node::new("D", "Delta", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
// 	let e = Node::new("E", "Epsilon", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
// 	let f = Node::new("F", "FALSE", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
//
// 	{
// 		let mut mut_root = root.borrow_mut();
// 		mut_root.edges.push(b.clone());
// 		mut_root.edges.push(c.clone());
// 		mut_root.edges.push(d.clone());
//
// 		let mut mut_c = c.borrow_mut();
// 		mut_c.edges.push(e.clone());
// 		mut_c.edges.push(f.clone());
// 		// mut_c.edges.push(root.clone());
// 	}
// 	root
// }
