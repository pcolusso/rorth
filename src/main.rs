use std::{path::Iter, str::Chars};

#[derive(Debug)]
enum Word {
    Push(u32),
    Plus,
    Print,
    Dup,
    Branch
}

fn run(progam: Vec<Word>) {
    let mut int_stack = vec!();

    for word in progam {
        match word {
            Word::Push(x) => int_stack.push(x),
            Word::Print => println!("{:#?}", int_stack.last()),
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
            "print" => Some(Word::Print),
            "+" | "plus" => Some(Word::Plus),
            "dup" => Some(Word::Dup),
            _ => None
        }
    }

}

fn main() {
    let input = "12 dup 32 + print";
    let program: Vec<Word> = CodeStream{input: input.chars()}.into_iter().collect();

    println!("{:#?}", program);
    
    run(program);



    println!("Hello, world!");
}
