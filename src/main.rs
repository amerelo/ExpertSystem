use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
mod parser_module;


use parser_module::parser::Parser;
/*
enum Element {
    Fact(Node),
    Rule(Node),
}
*/
struct Node {
    datum: &'static str,
    dattype: &'static str,
    edges: Vec<Rc<RefCell<Node>>>,
}



impl Node {
    fn new(datum: &'static str, dattype: &'static str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            datum: datum,
            dattype: dattype,
            edges: Vec::new(),
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

    fn first(&self) -> Rc<RefCell<Node>> {
        self.edges[0].clone()
    }
    fn second(&self) -> Rc<RefCell<Node>> {
        self.edges[1].clone()
    }
}

fn foo(node: &Node) {
    println!("foo:datnum: {} dattype:{}", node.datum, node.dattype);
}

fn init() -> Rc<RefCell<Node>> {
    let root = Node::new("A", "ALPHA");

    let b = Node::new("B", "Beta");
    let c = Node::new("C", "Charly");
    let d = Node::new("D", "Delta");
    let e = Node::new("E", "Epsilon");
    let f = Node::new("F", "FALSE");

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
