use crate::ast::*;
use chumsky::prelude::*;

pub fn number_parser() -> impl Parser<char, String, Error = Simple<char>> + Clone {
    let frac = just::<char, _, Simple<char>>('.').chain::<char, _, _>(text::digits(10));

    let exp = just('e')
        .or(just('E'))
        .chain(just('+').or(just('-')).or_not())
        .chain::<char, _, _>(text::digits(10));

    just('-')
        .or_not()
        .chain::<char, _, _>(text::int(10))
        .chain::<char, _, _>(frac.or_not().flatten())
        .chain::<char, _, _>(exp.or_not().flatten())
        .collect::<String>()
}

pub fn expression_parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    recursive(|expr| {
        let atom = choice((
            number_parser().padded().map(Expression::Literal),
            text::ident().padded().map(Expression::Variable),
            expr.delimited_by(just('(').padded(), just(')').padded()),
        ));

        let op = choice((
            just('*').padded().map(|_| BinaryOp::Mul),
            just('/').padded().map(|_| BinaryOp::Div),
        ));

        let prod = atom
            .clone()
            .padded()
            .then(op.then(atom.clone()).padded().repeated())
            .foldl(|lhs, (op, rhs)| Expression::Binary(Box::new(BinaryExpr { op, lhs, rhs })));

        let op = choice((
            just('+').padded().map(|_| BinaryOp::Add),
            just('-').padded().map(|_| BinaryOp::Sub),
        ));

        let sum = prod
            .clone()
            .padded()
            .then(op.then(prod.clone()).padded().repeated())
            .foldl(|lhs, (op, rhs)| Expression::Binary(Box::new(BinaryExpr { op, lhs, rhs })));

        let op = choice((
            just('>').padded().map(|_| BinaryOp::CmpL),
            just('<').padded().map(|_| BinaryOp::CmpR),
        ));

        choice((
            sum.clone()
                .then(op)
                .then(sum.clone())
                .map(|((lhs, op), rhs)| Expression::Binary(Box::new(BinaryExpr { op, lhs, rhs }))),
            sum.clone(),
        ))
    })
}

pub fn let_parser() -> impl Parser<char, Let, Error = Simple<char>> {
    text::keyword("let")
        .padded()
        .ignore_then(text::ident().padded())
        .then_ignore(just("=").padded())
        .then(expression_parser())
        .then_ignore(just(';').padded())
        .map(|(variable, value)| Let { variable, value })
}

pub fn print_parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    just("print")
        .padded()
        .ignore_then(expression_parser())
        .then_ignore(just(';').padded())
}

pub fn statement_parser() -> impl Parser<char, Statement, Error = Simple<char>> {
    recursive(|statement| {
        let if_parser = text::keyword("if")
            .padded()
            .ignore_then(expression_parser())
            .then(
                statement
                    .clone()
                    .padded()
                    .repeated()
                    .delimited_by(just('[').padded(), just(']').padded()),
            )
            .map(|(condition, body)| If { condition, body });

        let while_parser = text::keyword("while")
            .padded()
            .ignore_then(expression_parser())
            .then(
                statement
                    .clone()
                    .padded()
                    .repeated()
                    .delimited_by(just('[').padded(), just(']').padded()),
            )
            .map(|(condition, body)| While { condition, body });

        choice((
            let_parser().map(Box::new).map(Statement::Let),
            if_parser.map(Box::new).map(Statement::If),
            while_parser.map(Box::new).map(Statement::While),
            print_parser().map(Statement::Print),
        ))
    })
}

pub fn program_parser() -> impl Parser<char, Program, Error = Simple<char>> {
    text::keyword("Program")
        .ignore_then(text::ident().padded())
        .then(text::ident())
        .then(
            statement_parser()
                .padded()
                .repeated()
                .delimited_by(just('[').padded(), just(']').padded()),
        )
        .then_ignore(end())
        .map(|((namespace, name), body)| Program {
            namespace,
            name,
            body,
        })
}
