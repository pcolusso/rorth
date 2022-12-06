use std::{path::Iter, str::Chars, fmt};

enum Word {
    Push(u32),
    Plus,
    Print,
    Dup,
    Bye,
    Branch
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Word::Push(x) => write!(f, "↑ {}", x),
            Word::Plus => write!(f, "+"),
            Word::Print => write!(f, "↓ "),
            Word::Bye => write!(f, "ⓧ"),
            Word::Dup => write!(f, "x2"),
            Word::Branch => write!(f, " ⑂")
        }
    }
}

fn run(progam: Vec<Word>) {
    let mut int_stack = vec!();

    for word in progam {
        match word {
            Word::Push(x) => int_stack.push(x),
            Word::Print => println!("{:#?}", int_stack.pop()),
            Word::Plus => {
                if int_stack.len() >= 2 {
                    let x = int_stack.pop().unwrap();
                    let y = int_stack.pop().unwrap();
                    int_stack.push(x + y)
                } else {
                    panic!("OOB");
                }
            },
            Word::Dup => {
                if let Some(x) = int_stack.pop() {
                    int_stack.push(x + x);
                }
            },
            Word::Bye => { std::process::exit(0); }
            Word::Branch => {},
        }
    }
}

struct CodeStream<'a> {
    input: Chars<'a>
}

impl Iterator for CodeStream<'_> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = vec!();
        
        // Get a word
        loop {
            let char = self.input.next();

            match char {
                None | Some(' ') => break,
                Some(x) => buf.push(x)
            }
        }

        let chunk: String = buf.into_iter().collect();

        if let Ok(x) = chunk.parse::<u32>() {
            return Some(Word::Push(x));
        }

        match chunk.as_str() {
            "print" | "." => Some(Word::Print),
            "+" | "plus" => Some(Word::Plus),
            "bye" => Some(Word::Bye),
            "dup" => Some(Word::Dup),
            _ => None
        }
    }

}

fn main() {
    let input = "10 dup 20 + . bye";
    let program: Vec<Word> = CodeStream{input: input.chars()}.into_iter().collect();

    for w in &program {
        print!("{} ", w);
    }
    println!("");
    
    run(program);

    println!("Hello, world!");
}
