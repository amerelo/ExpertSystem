use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use parser_module::parser::Parser;


pub enum Types {
	Fac(Fact),
	Rul(Rule),
}

pub enum Operator {
	Not,
	And,
	Or,
	Xor,
}

pub struct Rule {
	pub op: Operator,
	pub edges: Vec<Rc<RefCell<Node>>>,
}

pub struct Fact {
	pub name: String,
	pub valid: bool,
	pub invalid: bool,
}

pub struct Node {
	pub datum: &'static str,
	pub dattype: &'static str,

	edges: Vec<Rc<RefCell<Node>>>,
	classe: Types,
}

impl Node {
	pub fn new(datum: &'static str, dattype: &'static str, elem: Types) -> Rc<RefCell<Node>> {
		Rc::new(RefCell::new(Node {
			datum: datum,
			dattype: dattype,
			edges: Vec::new(),
			classe: elem,
		}))
	}

	pub fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
	where F: Fn(&'static str)
	{
		if seen.contains(&self.datum) {
			return;
		}
		f(self.datum);
		seen.insert(self.datum);
		for n in &self.edges {
			n.borrow().traverse(f, seen);
		}
	}

	pub fn first(&self) -> Rc<RefCell<Node>> {
		self.edges[0].clone()
	}
	pub fn second(&self) -> Rc<RefCell<Node>> {
		self.edges[1].clone()
	}
}

pub fn init() -> Rc<RefCell<Node>> {

	let root = Node::new("A", "ALPHA", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );

	let b = Node::new("B", "Beta", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
	let c = Node::new("C", "Charly", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
	let d = Node::new("D", "Delta", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
	let e = Node::new("E", "Epsilon", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
	let f = Node::new("F", "FALSE", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );

	{
		let mut mut_root = root.borrow_mut();
		mut_root.edges.push(b.clone());
		mut_root.edges.push(c.clone());
		mut_root.edges.push(d.clone());

		let mut mut_c = c.borrow_mut();
		mut_c.edges.push(e.clone());
		mut_c.edges.push(f.clone());
		// mut_c.edges.push(root.clone());
	}

	root
}
