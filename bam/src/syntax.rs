#[derive(Debug, Clone)]
pub struct Program {
    pub machines: Vec<MachineDef>,
}
#[derive(Debug, Clone)]
pub struct MachineDef {
    pub name: String,
    pub body: Vec<Statement>,
    pub result: Stream,
}
#[derive(Debug, Clone)]
pub enum Statement {
    Let(Vec<String>, Stream), // let x, y = s;
    ConsumeStream(Stream),    // s;
}
// Id = input
// Five = 5

#[derive(Debug, Clone)]
pub enum Stream {
    // NOTE: just parse this as "Var("input")
    // Input,                                    // input
    Var(String),                                 // x
    NumConst(f64),                               // v
    StringConst(String),                         // str
    Pipe(Box<Stream>, Box<Machine>),             // s -> m
    Zip(Vec<Stream>),                            // s₁ , .. , sₙ
    Cond(Box<Stream>, Box<Stream>, Box<Stream>), // s₁ ? s₂ : s₃
    Limit(Box<Stream>, usize)                    // s{n}
}

#[derive(Debug, Clone)]

pub enum Machine {
    Var(String),
    Builtin(Builtin),
    Defined(Vec<Statement>)
}

#[derive(Debug, Clone)]
pub enum Builtin {
    Add,
    Mul,
    Dup2,
    Dup3,
}

#[derive(Debug, Clone)]
pub enum Value {
    /// All streams are infinite.
    /// When a stream is empty is keeps returning Null.
    Null,
    Num(f64),
    Str(String),
    Bool(bool),
    Tuple(Vec<Value>),
}
