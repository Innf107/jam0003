use std::ops::Range;

use crate::lexer::Token;
use crate::syntax::*;
use chumsky::prelude::*;

fn parser() -> impl Parser<Token, Program, Error = Simple<Token>> {
    let ident = filter_map(|span: Range<usize>, tok| match tok {
        Token::Ident(ident) => Ok(ident),
        _ => Err(Simple::custom(span, "Expected an identifier")),
    });

    let float = filter_map(|span: Range<usize>, tok| match tok {
        Token::FloatLit(str) => Ok(str.parse::<f64>().unwrap()),
        Token::IntLit(i) => Ok(i as f64),
        _ => Err(Simple::custom(span, "Expected a number literal")),
    });

    let positive_int = filter_map(|span: Range<usize>, tok| match tok {
        Token::IntLit(i) if i > 0 => Ok(i as usize),
        _ => Err(Simple::custom(span, "Expected a positive integer"))
    });

    let string = filter_map(|span: Range<usize>, tok| match tok {
        Token::StringLit(str) => Ok(str),
        _ => Err(Simple::custom(span, "Expected a string literal")),
    });

    let machine = ident.map(|x| {Machine::Var(x)});

    let stream: Recursive<Token, Stream, _> = recursive(|stream| {
        let stream_leaf = 
            ident.map(|name| {Stream::Var(name)})       // x
            .or(float.map(|f| {Stream::NumConst(f)})) // n
            .or(string.map(|s| {Stream::StringConst(s)})) // s
            .or(just(Token::Input).to(Stream::Var(String::from("Input")))) // input
            .or(just(Token::Lparen).then(stream.clone()).then_ignore(just(Token::Rparen)).map(|(_, s)| {s})); // ( s )

        let limit = 
            stream_leaf.clone()
            .then_ignore(just(Token::Lbrace))
            .then(positive_int)
            .then_ignore(just(Token::Rbrace))
            .map(|(stream, count)| {Stream::Limit(Box::new(stream), count)});

        let stream_limit = limit.or(stream_leaf.clone());

        let stream_zip = 
            stream_limit.clone()
            .separated_by(just(Token::Comma))
            .map(|streams| {
                if streams.len() == 1 {
                    streams[0].clone()
                } else {
                    Stream::Zip(streams)
                }
            });

        let cond_ = 
            stream_zip.clone()
            .then_ignore(just(Token::QuestionMark))
            .then(stream.clone())
            .then_ignore(just(Token::Colon))
            .then(stream.clone())
            .map(|((pred, then), else_)| {
                Stream::Cond(Box::new(pred), Box::new(then), Box::new(else_))
            });

        let stream_cond = cond_.or(stream_zip);

        let pipe = 
        stream_cond.clone()
            .then_ignore(just(Token::Arrow))
            .then(machine.separated_by(just(Token::Arrow)))
            .map(|(stream, machines)|{
                machines.into_iter().fold(stream, |stream, machine|{
                    Stream::Pipe(Box::new(stream), Box::new(machine))
                })
            });
        
        pipe.or(stream_cond)
    });
    let let_ = 
        just(Token::Let)
        .then(ident.separated_by(just(Token::Comma)))
        .then_ignore(just(Token::Equals))
        .then(stream.clone())
        .map(|((_, vars), stream)| { Statement::Let(vars, stream)});


    let consume_stream = stream.clone().map(|s| {Statement::ConsumeStream(s)});

    let statement = let_.or(consume_stream);


    let machine_def = 
        ident
        .then_ignore(just(Token::Lbrace))
        .then_ignore(just(Token::Rbrace))
        .then(statement.then_ignore(just(Token::Semicolon)).repeated())
        .then(stream.clone())
        .map(|((name, body), result)| MachineDef { name, body, result });
    
    machine_def
        .separated_by(just(Token::Semicolon))
        .map(|machines| Program { machines })
}
