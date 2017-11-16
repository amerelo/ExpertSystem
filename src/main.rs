extern crate colored;


mod parser_module;
mod rpn_module;
mod graph_module;

use parser_module::parser::Parser;
use rpn_module::rpn::Rpn;
use graph_module::graph::Node;
use graph_module::graph::Types;

// fn foo(node: &Node) {
// 	println!("foo:datnum: {} dattype:{}", node.datum, node.dattype);
// }
//
// fn start_node(g: &Node)
// {
// 	g.traverse(&|d| println!("{}", d), &mut HashSet::new());
// 	let f = g.first();
// 	foo(&*f.borrow());
// 	let h = g.second();
// 	foo(&*h.borrow());
// }

pub fn main() {
	//start_node(&*tmp.borrow());
	let mut data:  Parser = Parser{node: vec![], val_init: vec![], val_search: vec![] };

	if let Err(e) = data.parse() {
		panic!("Error : {}", e);
	}
	Rpn::prefixparse(&mut data);
	let mut graph: Node = Node{name: "MasterNode".to_string(), classe: Types::None, edges: vec![], find: false};
	graph.generate(&mut data);
}
