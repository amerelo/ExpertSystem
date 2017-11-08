mod parser_module;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use parser_module::parser::Parser;

struct Rule {
	name: String,
}

struct Fact {
	name: String,
}

enum Types {
	Fac(Fact),
	Rul(Rule),
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
	let root = Element::new("A", "ALPHA", Types::Fac(Fact{name: "toto".to_string() }) );

	let b = Element::new("B", "Beta", Types::Fac(Fact{name: "toto".to_string() }) );
	let c = Element::new("C", "Charly", Types::Fac(Fact{name: "toto".to_string() }) );
	let d = Element::new("D", "Delta", Types::Fac(Fact{name: "toto".to_string() }) );
	let e = Element::new("E", "Epsilon", Types::Fac(Fact{name: "toto".to_string() }) );
	let f = Element::new("F", "FALSE", Types::Fac(Fact{name: "toto".to_string() }) );

	{
		let mut mut_root = root.borrow_mut();
		mut_root.edges.push(b.clone());
		mut_root.edges.push(c.clone());
		mut_root.edges.push(d.clone());

		let mut mut_c = c.borrow_mut();
		mut_c.edges.push(e.clone());
		mut_c.edges.push(f.clone());
		mut_c.edges.push(root.clone());
	}

	root
}

pub fn main() {
	let g = init();
	let g = g.borrow();
	g.traverse(&|d| println!("{}", d), &mut HashSet::new());
	let f = g.first();
	foo(&*f.borrow());
	let h = g.second();
	foo(&*h.borrow());

	let lines: Vec<String> = Parser::parse();

	for line in lines {
		println!("{}", line);
	}
}
