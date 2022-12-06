use std::collections::HashMap;
use std::io::BufRead;

mod ops;

type Program = Vec<Word>;
type Dictionary = HashMap<String, Word>;
type NativeOp = fn(&mut Machine);

enum Op {
    Native(NativeOp),
    Interpreted(Program),
    Push(u32)
}

pub struct Machine {
    int_stack: Vec<u32>,
    dictionary: Dictionary
}

impl Machine {
    fn new() -> Self {
        let mut dictionary = HashMap::new();

        dictionary.insert(".".into(),   Word { name: "Print".into(),  op: Op::Native(ops::print)});
        dictionary.insert("bye".into(), Word { name: "Exit".into(),   op: Op::Native(ops::bye)});
        dictionary.insert("+".into(),   Word { name: "Add".into(),    op: Op::Native(ops::plus)});
        dictionary.insert("dup".into(), Word { name: "Double".into(), op: Op::Native(ops::double)});
        dictionary.insert("?".into(), Word { name: "Inspect".into(), op: Op::Native(ops::inspect)});

        let int_stack = vec!();
        Self { int_stack, dictionary }
    }

    fn run(&mut self, input: &str) {
        for command in input.split_whitespace() {
            match self.dictionary.get(command) {
                Some(word) => match &word.op {
                    Op::Interpreted(program) => unimplemented!(),
                    Op::Native(func) => func(self),
                    Op::Push(_) => unimplemented!()
                }
                None => {
                    if let Ok(x) = command.parse::<u32>() {
                        self.int_stack.push(x)
                    } else {
                        panic!("Cannot interpret {}", command)
                    }
                }
            }
        }
    }
}

struct Word {
    name: String,
    op: Op
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let mut machine = Machine::new();

    let _ = args.drain(0..1);

    if let Some(filename) = args.pop() {
        if let Ok(file) = std::fs::File::open(filename) {
            let reader = std::io::BufReader::new(file);
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    machine.run(&line);
                }
            }
        }
    } else {
        let stdin = std::io::stdin();

        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                machine.run(&line);
            }
        }
    }

    
}
