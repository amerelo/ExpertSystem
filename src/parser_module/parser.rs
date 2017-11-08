use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct Parser {

}

impl Parser
{
    pub fn parse() -> Vec<String> {
        let args: Vec<_> = env::args().collect();
        let mut f = File::open(&args[1]).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        let mut end: Vec<String> = vec![];
        let v: Vec<&str> = contents.split('\n').collect();
        for lm in v {
            match lm.split('#').nth(0) {
                 Some(x) => {
                     if x.len() > 0 {end.push(x.trim().clone().to_string())}
                 },
                 None => {println!("no elem in line to split")},
            }
        }
       end
    }
}
