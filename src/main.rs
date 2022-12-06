use std::{path::Iter, str::Chars, fmt, collections::HashMap};

type Program = Vec<Word>;
type Dictionary = HashMap<String, Word>;
type NativeOp = fn(&mut Machine);

enum Op {
    Native(NativeOp),
    Interpreted(Program),
    Push(u32)
}

struct Machine {
    int_stack: Vec<u32>,
    dictionary: Dictionary
}

impl Machine {
    fn new() -> Self {
        let mut dictionary = HashMap::new();

        dictionary.insert(".".into(),   Word { name: "Print".into(), op: Op::Native(print)});
        dictionary.insert("bye".into(), Word { name: "Exit".into(),  op: Op::Native(bye)});
        dictionary.insert("+".into(),   Word { name: "Add".into(),   op: Op::Native(plus)});
        dictionary.insert("dup".into(), Word { name: "Double".into(), op: Op::Native(double)});

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

fn print(machine: &mut Machine) {
    let x = machine.int_stack.pop().unwrap();
    println!("{}", x);
}

fn bye(_: &mut Machine) {
    std::process::exit(0);
}

fn plus(machine: &mut Machine) {
    let x = machine.int_stack.pop().unwrap();
    let y = machine.int_stack.pop().unwrap();
    machine.int_stack.push(x.saturating_add(y));
}

fn double(machine: &mut Machine) {
    let x= machine.int_stack.pop().unwrap();
    machine.int_stack.push(x.saturating_add(x));
}

fn main() {
    let input = "10 dup 20 + . bye";
    let mut machine = Machine::new();

    machine.run(input);
}
