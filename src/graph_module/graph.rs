use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use parser_module::parser::Parser;

pub enum Types {
	Fac(Fact),
	Rul(Rule),
	None,
}

pub enum Operator {
	Not,
	And,
	Or,
	Xor,
}

pub struct Rule {
	pub op: Operator,

	pub content:String,
}

pub struct Fact {
	pub name: String,
	pub valid: bool,
	pub invalid: bool,
}

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

	pub fn traverse<F>(&self, f: &F, seen: &mut HashSet<String>)
	where F: Fn(String)
	{
		if seen.contains(&self.name) {
			return;
		}

		f(self.name.clone());
		seen.insert(self.name.clone());

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

	pub fn generate(&mut self, data : &mut Parser) {
		for elem in data.node.iter(){
			println!("elem > {:?}", elem);
			for item in elem.rules.chars() {
				println!("rules > {:?}", item);
				if item.is_alphabetic() {
					let tmp = Node::new(item.to_string().clone(), Types::Fac(Fact{name: item.to_string().clone(), valid: false, invalid: false,}) );
					self.edges.push(tmp.clone());
				}
			}
		}
		self.start_node();
	}

	pub fn foo(&self, node: &Node) {
		println!("name: {}", node.name);
	}

	pub fn start_node(&self)
	{
		self.traverse(&|d| println!("{}", d), &mut HashSet::new());
		let f = self.first();
		self.foo(&*f.borrow());
		let h = self.second();
		self.foo(&*h.borrow());
	}
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
//
// 	root
// }
