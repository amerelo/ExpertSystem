extern crate colored;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ExpertSystem", about = "An Expert System Resolver.")]
struct Opt {
    /// A flag, true if used in the command line.
    #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
    debug: bool,

    #[structopt(short = "a", long = "all", help = "Show All")]
    showall: bool,

    #[structopt(short = "v", long = "Rules", help = "Show Rules")]
    showrules: bool,

    /// An argument of type float, with a default value.
    //#[structopt(short = "s", long = "speed", help = "Set speed", default_value = "42")]
    //speed: f64,

    /// Needed parameter, the first on the command line.
    #[structopt(help = "Input file")]
    input: String,

    // An optional parameter, will be `None` if not present on the
    // command line.
    //#[structopt(help = "Output file, stdout if not present")]
    //output: Option<String>,
}

mod parser_module;
mod rpn_module;
mod graph_module;

use std::rc::Rc;
use std::cell::RefCell;
use parser_module::parser::Parser;
use rpn_module::rpn::Rpn;
use graph_module::graph::Node;
use graph_module::graph::Types;
use colored::*;
use std::process;

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
    let opt = Opt::from_args();

    let mut node: Rc<RefCell<Node>>;
    let mut data:  Parser = Parser{node: vec![], val_init: vec![], val_search: vec![] };
    if let Err(e) = data.parse(&opt.input) {
		println!("Error : {}", e);
        process::exit(1);
	}

    //println!("{:?}", opt);

    // if !std::path::Path::new(File::open(opt.input)).exists() {
    //         println!("Enter valid file !");
    //         process::exit(1);
    // }

    if opt.showall {
        data.input();
        println!("{}", "==========================".cyan());
    }

    Rpn::prefixparse(&mut data);

	let mut graph: Node = Node{name: "MasterNode".to_string(), classe: Types::None, edges: vec![]};
	graph.generate(&mut data);

    if opt.showrules || opt.showall {
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
