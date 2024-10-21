use crate::lexer::*;

enum Expr {
    Literal(Token),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Group(Box<Expr>),
}

// Primary ::= "true" | "false" | "nil" | NUMBER | STRING | "(" <Expression> ")"
// Unary ::= "!" <Unary>
//         | "-" <Unary>
//         | <Primary>
// Factor ::= <Unary> "/" <Factor>
//          | <Unary> "*" <Factor>
//          | <Unary>
// Term ::= <Factor> "+" <Term>
//        | <Factor> "-" <Term>
//        | <Factor>
// Comparison ::= <Term> ">" <Comparison>
//              | <Term> ">=" <Comparison>
//              | <Term> "<" <Comparison>
//              | <Term> "<=" <Comparison>
//              | <Term>
// Equality ::= <Comparison> "==" <Equality>
//            | <Comparison> "!=" <Equality>
//            | <Comparison>
// Expression ::= <Equality>

struct Parser {}

impl Parser {
    fn parse_expression(&self) {}
}
