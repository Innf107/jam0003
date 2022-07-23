use crate::syntax::{Machine, Builtin, Program, Stream, Value, Statement};
use std::collections::HashMap;

struct Factory {
    machines: HashMap<String, Machine>,
    streams: HashMap<String, Stream>,
}

impl Factory {
    pub fn new(program: Program) -> Self {
        Factory {
            machines: program
                .machines
                .into_iter()
                .map(|m| (m.name.clone(), Machine::Defined(m.body)))
                .collect(),
            streams: HashMap::new(),
        }
    }

    pub fn run_builtin_machine(&self, builtin: &Builtin, value: Value) -> Value {
        todo!()
    }

    pub fn run_defined_machine(&self, statements: &[Statement], value: Value) -> Value {
        for stmt in statements {

        }

        todo!()
    }

    pub fn run_machine(&self, machine: &Machine, value: Value) -> Value {
        match machine {
            Machine::Var(var) => {
                self.run_machine(self.machines.get(var).expect("undefined stream."), value)
            }
            Machine::Builtin(builtin) => self.run_builtin_machine(builtin, value),
            Machine::Defined(statements) => self.run_defined_machine(statements, value)
        }
    }

    pub fn advance_stream(&self, stream: &mut Stream) -> Value {
        match stream {
            Stream::Var(var) => {
                self.advance_stream(&mut self.streams.get(var).expect("undefined stream."))
            }
            Stream::NumConst(float) => Value::Num(*float),
            Stream::StringConst(string) => Value::Str(string.clone()),
            Stream::Pipe(mut stream, machine) => {
                let value = self.advance_stream(&mut stream);
                self.run_machine(&machine, value)
            }
            Stream::Zip(streams) => Value::Tuple(
                streams
                    .into_iter()
                    .map(|s| self.advance_stream(s))
                    .collect(),
            ),
            Stream::Limit(stream, limit) => {
                if *limit == 0 {
                    Value::Null
                } else {
                    *limit -= 1;
                    self.advance_stream(stream)
                }
            }
            Stream::Cond(cond_stream, then_stream, else_stream) => {
                if let Value::Bool(cond) = self.advance_stream(cond_stream) {
                    if cond {
                        self.advance_stream(then_stream)
                    } else {
                        self.advance_stream(else_stream)
                    }
                } else {
                    panic!("Error: non-bool in conditional")
                }
            }
        }
    }
}
