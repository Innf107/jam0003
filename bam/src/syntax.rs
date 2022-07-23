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
    // Input,                                       // input
    Var(String),                                 // x
    NumConst(f64),                               // v
    StringConst(String),                        // str
    Pipe(Box<Stream>, Box<Machine>),             // s -> m
    Zip(Vec<Stream>),                            // s₁ , .. , sₙ
    Cond(Box<Stream>, Box<Stream>, Box<Stream>), // s₁ ? s₂ : s₃
    Limit(Box<Stream>, usize)                    // s{n}
}

#[derive(Debug, Clone)]

pub enum Machine {
    Named(String), // x
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(f64),
    String(String),
    Tuple(Vec<Value>),
}
