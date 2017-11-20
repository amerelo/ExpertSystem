extern crate colored;

mod parser_module;
mod rpn_module;
mod graph_module;

use std::rc::Rc;
use std::cell::RefCell;
use parser_module::parser::Parser;
use rpn_module::rpn::Rpn;
use graph_module::graph::Node;
use graph_module::graph::Types;
use std::env;
use colored::*;

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
    let args: Vec<String> = env::args().collect();
    let mut node: Rc<RefCell<Node>>;
	//start_node(&*tmp.borrow());
	let mut data:  Parser = Parser{node: vec![], val_init: vec![], val_search: vec![] };

	if let Err(e) = data.parse() {
		panic!("Error : {}", e);
	}


    if args.len() > 3 {
        data.input();
        println!("{}", "==========================".cyan());
    }

    Rpn::prefixparse(&mut data);

	let mut graph: Node = Node{name: "MasterNode".to_string(), classe: Types::None, edges: vec![]};
	graph.generate(&mut data);

    if args.len() > 2 {
        for elem in graph.edges.iter() {
            if data.val_search.contains(&elem.borrow().name) {
                graph.print_node(elem, 0);
            }
        }
        println!("{}","==========================".cyan());
    }

    for search in data.val_search.iter() {
        node = graph.get_node_by_name(search.clone());
       // print!("{} ", search);
        if let Types::Fac(ref fac) = node.borrow().classe {
            //println!("fact => {:?}", fac);
            graph.show_fact(fac);
            print!(" ");
        }
    }

    println!("");
}
