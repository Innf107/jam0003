mod syntax;
use syntax::{Decl, Machine, Statement, Stream};

use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};

use syntax::DefMachine;

struct CopyStreamState<I>
where
    I: Iterator,
    I::Item: Clone,
{
    buffers: Vec<VecDeque<I::Item>>,
    iterator: I,
}

struct CopyStream<I>
where
    I: Iterator,
    I::Item: Clone,
{
    state: Rc<RefCell<CopyStreamState<I>>>,
    stream_index: usize,
}

impl<I> Iterator for CopyStream<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let mut state = self.state.borrow_mut();
        match state.buffers[self.stream_index].pop_back() {
            Some(x) => Some(x),
            None => match state.iterator.next() {
                Some(x) => {
                    for (i, buffer) in &mut state.buffers.iter_mut().enumerate() {
                        if i != self.stream_index {
                            buffer.push_front(x.clone())
                        };
                    }
                    Some(x)
                }
                None => None,
            },
        }
    }
}

fn copy_stream<A: Clone, I: Iterator<Item = A>>(
    stream: I,
) -> (impl Iterator<Item = A>, impl Iterator<Item = A>) {
    let state = Rc::new(RefCell::new(CopyStreamState {
        buffers: vec![VecDeque::new(), VecDeque::new()],
        iterator: stream,
    }));
    let stream1 = CopyStream {
        state: state.clone(),
        stream_index: 0,
    };
    let stream2 = CopyStream {
        state,
        stream_index: 1,
    };
    (stream1, stream2)
}

fn unzip<A: Clone, B: Clone, I: Iterator<Item = (A, B)>>(
    stream: I,
) -> (impl Iterator<Item = A>, impl Iterator<Item = B>) {
    let (left, right) = copy_stream(stream);
    (left.map(|(x, _)| x), right.map(|(_, y)| y))
}

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
    let integers = print(integers(5));
    let (mut stream1, mut stream2) = copy_stream(integers);

    let example = vec![Decl::DefMachine(DefMachine {
        name: String::from("fact"),
        body: vec![Statement::Let(
            vec![String::from("x"), String::from("y"), String::from("z")],
            Stream::Pipe(
                Box::new(Stream::Input),
                Box::new(Machine::Named(String::from("dup3"))),
            ),
        )],
        result: Stream::Cond(
            Box::new(Stream::Pipe(
                Box::new(Stream::Var(String::from("z"))),
                Box::new(Machine::Named(String::from("positive"))),
            )),
            Box::new(Stream::Pipe(
                Box::new(Stream::Pipe(
                    Box::new(Stream::Pipe(
                        Box::new(Stream::Tuple(vec![Stream::NumLit(-1.), Stream::Var(String::from("x"))])),
                        Box::new(Machine::Named(String::from("add"))),
                    )),
                    Box::new(Machine::Named(String::from("fact"))),
                )),
                Box::new(Machine::Named(String::from("multiply"))),
            )),
            Box::new(Stream::NumLit(1.)),
        ),
    })];

    for _ in 0..10 {
        println!("s1: {:?}", stream1.next());
        println!("s2: {:?}", stream2.next());
    }
}
