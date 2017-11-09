mod parser_module;
mod rpn_module;

use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use parser_module::parser::Parser;
use rpn_module::rpn::Rpn;

enum Types {
	Fac(Fact),
	Rul(Rule),
}

enum Operator {
	Not,
	And,
	Or,
	Xor,
}

struct Rule {
	op: Operator,
	edges: Vec<Rc<RefCell<Element>>>,
}

struct Fact {
	name: String,
	valid: bool,
	invalid: bool,
}

struct Element {
	datum: &'static str,
	dattype: &'static str,

	edges: Vec<Rc<RefCell<Element>>>,
	classe: Types,
}

impl Element {
	fn new(datum: &'static str, dattype: &'static str, elem: Types) -> Rc<RefCell<Element>> {
		Rc::new(RefCell::new(Element {
			datum: datum,
			dattype: dattype,
			edges: Vec::new(),
			classe: elem,
		}))
	}

	fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
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

	fn first(&self) -> Rc<RefCell<Element>> {
		self.edges[0].clone()
	}
	fn second(&self) -> Rc<RefCell<Element>> {
		self.edges[1].clone()
	}
}

fn foo(node: &Element) {
	println!("foo:datnum: {} dattype:{}", node.datum, node.dattype);
}

fn init() -> Rc<RefCell<Element>> {

	let root = Element::new("A", "ALPHA", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );

	let b = Element::new("B", "Beta", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
	let c = Element::new("C", "Charly", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
	let d = Element::new("D", "Delta", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
	let e = Element::new("E", "Epsilon", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );
	let f = Element::new("F", "FALSE", Types::Fac(Fact{name: "toto".to_string(), valid: false, invalid: false }) );

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

fn start_node(g: &Element)
{
	g.traverse(&|d| println!("{}", d), &mut HashSet::new());
	let f = g.first();
	foo(&*f.borrow());
	let h = g.second();
	foo(&*h.borrow());
}

pub fn main() {
	// let tmp = init();
	// start_node(&*tmp.borrow());
	let mut lines: Vec<String> = vec![];

	match Parser::parse() {
		Ok(elem) => lines = elem,
		Err(e) => println!("{}", e),
	}

	let mut pars:  Parser = Parser{node: vec![], val_init: vec![], val_search: vec![] };

	let source = String::from("(A + B) | D");
	let output = Rpn::prefix(source);

	println!("{}", output);
}
