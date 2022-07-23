#[derive(Debug, Clone)]
pub enum Decl {
    DefMachine(DefMachine),
}
#[derive(Debug, Clone)]
pub struct DefMachine {
    pub name: String,
    pub body: Vec<Statement>,
    pub result: Stream,
}
#[derive(Debug, Clone)]
pub enum Statement {
    Let(Vec<String>, Stream),   // let x, y = s;
    RunStream(Stream),          // s;
}
#[derive(Debug, Clone)]
pub enum Stream {
    Var(String),                                    // x
    NumLit(f64),                                    // n
    Input,                                          // input
    Pipe(Box<Stream>, Box<Machine>),                // s -> m
    Tuple(Vec<Stream>),                             // s₁ , .. , sₙ
    Merge(Vec<Stream>),                             // s₁ | .. | sₙ
    Cond(Box<Stream>, Box<Stream>, Box<Stream>),    // s₁ ? s₂ : s₃
}

#[derive(Debug, Clone)]

pub enum Machine {
    Named(String),  // x
    Const(f64)      // f
}
