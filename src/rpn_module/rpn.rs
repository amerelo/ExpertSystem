use parser_module::parser::Parser;

pub struct Rpn {

}

impl Rpn
{
    pub fn prefix(source : &String) -> String {

        let tokens : Vec<char> = source.chars().collect();
        let mut opstack : Vec<char> = vec![];
        let mut output : Vec<char> = vec![];

        let mut opshit = 0;
        let mut alphashit = 0;
        for token in tokens {
            if token.is_alphabetic()
            {
                if (alphashit > 0)
                {
                    return String::from("WTF ALPHA");
                }
                alphashit += 1;
                opshit = 0;
                output.push(token.clone());
            }
            else if token.eq(& '(')
            {
                opstack.push(token.clone());
            }
            else if token.eq(& ')')
            {
                let mut topopstack = getpop(&mut opstack);
                while !topopstack.eq(& '(') && !topopstack.eq(& '0') {
                    output.push(topopstack);
                    topopstack = getpop(&mut opstack);
                }
            }
            else if isoperator(token)
            {
                if !token.eq(&'!') {
                    alphashit = 0;
                }
                if opshit > 0 && !token.eq(&'!') {
                    return String::from("WTF OPERAND");
                }
                opshit += 1;
                while opstack.len() > 0 && opprec(peekop(&opstack)) >= opprec(token) {
                    output.push(getpop(&mut opstack));
                }
                opstack.push(token);
            }
        }

        while opstack.len() > 0 {
            output.push(getpop(&mut opstack));
        }

        let s: String = output.into_iter().collect();
        s
    }

    pub fn prefixparse(pars : &mut Parser) {
        for nod in pars.node.iter_mut() {

            nod.rules = Rpn::prefix(&nod.rules);
            nod.facts = Rpn::prefix(&nod.facts);
            println!("{:?}", nod);
        }
    }
}

fn getpop(stack: &mut Vec<char>) -> char
{
	if let Some(x) = stack.pop() {
		return x;
	}
	return '0';
}

fn isoperator(op:char) -> bool {
	if op.eq(& '+') {
		return true;
	}
	if op.eq(& '^') {
		return true;
	}
	if op.eq(& '|') {
		return true;
	}
	if op.eq(& '!') {
		return true;
	}
	false
}

fn opprec(op:char) -> i32 {
	if op.eq(& '!') {
		return 5;
	}
	if op.eq(& '+') {
		return 4;
	}
	if op.eq(& '|') {
		return 3;
	}
	if op.eq(& '^') {
		return 2;
	}
	if op.eq(& '(') {
		return 1;
	}
	0
}

fn peekop(stack: & Vec<char>) -> char {
	let i = stack.len();
	let c : char;
	c = stack[i - 1];
	c
}
