mod eval;
mod parser;
mod lexer;
mod syntax;
mod stream;

use syntax::{Decl, Machine, Statement, Stream};

use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};

use syntax::DefMachine;

struct PrintIterator<I>
where
    I: Iterator,
{
    stream: I,
}
impl<I> Iterator for PrintIterator<I>
where
    I: Iterator,
    I::Item: Display,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.next() {
            Some(x) => {
                println!("{}", x);
                Some(x)
            }
            None => None,
        }
    }
}

fn print<A: Display>(stream: impl Iterator<Item = A>) -> impl Iterator<Item = A> {
    PrintIterator { stream }
}

struct IntegersIter {
    current: usize,
}
impl Iterator for IntegersIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current += 1;
        Some(value)
    }
}

fn integers(start: usize) -> IntegersIter {
    IntegersIter { current: start }
}

fn main() {

    let example = vec![Decl::DefMachine(DefMachine {
        name: String::from("fact"),
        body: vec![Statement::Let(
            vec![String::from("x"), String::from("y"), String::from("z")],
            Stream::Pipe(
                Box::new(Stream::Input),
                Box::new(Machine::Var(String::from("dup3"))),
            ),
        )],
        result: Stream::Cond(
            Box::new(Stream::Pipe(
                Box::new(Stream::Var(String::from("z"))),
                Box::new(Machine::Var(String::from("positive"))),
            )),
            Box::new(Stream::Pipe(
                Box::new(Stream::Pipe(
                    Box::new(Stream::Pipe(
                        Box::new(Stream::Zip(vec![
                            Stream::NumConst(-1.),
                            Stream::Var(String::from("x")),
                        ])),
                        Box::new(Machine::Var(String::from("add"))),
                    )),
                    Box::new(Machine::Var(String::from("fact"))),
                )),
                Box::new(Machine::Var(String::from("multiply"))),
            )),
            Box::new(Stream::NumConst(1.)),
        ),
    })];

    let integers = print(integers(5));
    let (mut stream1, mut stream2) = stream::copy_stream(integers);
    let (mut stream2, mut stream3) = copy_stream(stream2);

    for _ in 0..10 {
        println!("s1: {:?}", stream1.next());
        println!("s1: {:?}", stream1.next());
        println!("s2: {:?}", stream2.next());
        println!("s3: {:?}", stream3.next());
    }
}
