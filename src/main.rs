mod parser_module;
mod rpn_module;
mod graph_module;

use std::io;
use std::collections::HashSet;
use parser_module::parser::Parser;
use rpn_module::rpn::Rpn;
use graph_module::graph::Node;

fn foo(node: &Node) {
	println!("foo:datnum: {} dattype:{}", node.datum, node.dattype);
}

fn start_node(g: &Node)
{
	g.traverse(&|d| println!("{}", d), &mut HashSet::new());
	let f = g.first();
	foo(&*f.borrow());
	let h = g.second();
	foo(&*h.borrow());
}

pub fn main() {
	//start_node(&*tmp.borrow());
	let mut lines: Vec<String> = vec![];

	let mut pars:  Parser = Parser{node: vec![], val_init: vec![], val_search: vec![] };

	match pars.parse() {
		Ok(elem) => lines = elem,
		Err(e) => println!("{}", e),
	}
	Rpn::prefixparse(&mut pars);


}
