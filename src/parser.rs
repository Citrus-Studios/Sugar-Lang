#![allow(unused_braces)]

use crate::lexer::{Span, Token};
use crate::parser::Token::*;

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Span,
    pub node: Expr_,
}

#[derive(Debug, Clone)]
pub enum Expr_ {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var(String),
    Declare(String, Vec<String>),
    Define(String, Vec<String>, Vec<Expr>),
    FunctionCall(String, Vec<Expr>),
    Assign(String, Box<Expr>),
    Byte(u8),
}

plex::parser! {
    fn parse_(Token, Span);

    // combine two spans
    (a, b) {
        Span {
            lo: a.lo,
            hi: b.hi,
        }
    }

    program: Program {
        statements[s] => Program { stmts: s }
    }

    statements: Vec<Expr> {
        => vec![],
        statements[mut st] outer[e] SemiColon => {
            st.push(e);
            st
        }
    }

    outer: Expr {
        Declare ident[name] Equals declare_args[args] => Expr {
            span: span!(),
            node: Expr_::Declare(name, args)
        },
        Define ident[name] define_args[args] Equals exprs[block] => Expr {
            span: span!(),
            node: Expr_::Define(name, args, block)
        },
        Define ident[name] Equals exprs[block] => Expr {
            span: span!(),
            node: Expr_::Define(name, vec![], block)
        }
    }

    expr: Vec<Expr> {
        Variable ident[name] Equals exprs[e] SemiColon expr[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::Assign(name, Box::new(e[0].clone()))
            }];
            e.append(&mut m);
            e
        },
        Return ident[name] SemiColon => vec![Expr {
            span: span!(),
            node: Expr_::Var(name)
        }],
        Return expr[e] SemiColon => e,
    }

    exprs: Vec<Expr> {
        LBrace expr[e] RBrace => e,
        term[e] => vec![e]
    }

    term: Expr {
        term[a] Plus fact[b] => Expr {
            span: span!(),
            node: Expr_::Add(Box::new(a), Box::new(b))
        },
        term[a] Minus fact[b] => Expr {
            span: span!(),
            node: Expr_::Sub(Box::new(a), Box::new(b))
        },
        fact[a] => a
    }

    fact: Expr {
        fact[a] Star atom[b] => Expr {
            span: span!(),
            node: Expr_::Mul(Box::new(a), Box::new(b))
        },
        fact[a] Slash atom[b] => Expr {
            span: span!(),
            node: Expr_::Div(Box::new(a), Box::new(b))
        },
        atom[a] => a
    }

    atom: Expr {
        ident[a] => Expr {
            span: span!(),
            node: Expr_::Var(a)
        },
        Byte(x) => Expr {
            span: span!(),
            node: Expr_::Byte(x)
        },
        LParen term[a] RParen => a
    }

    declare_args: Vec<String> {
        ident[arg] => vec![arg],
        Bang => vec![String::from("void")],
        ident[arg] Minus Gt declare_args[mut second_arg] => {
            let mut arg = vec![arg];
            arg.append(&mut second_arg);
            arg
        }
        Bang Minus Gt declare_args[mut second_arg] => {
            let mut arg = vec![String::from("void")];
            arg.append(&mut second_arg);
            arg
        }
    }

    define_args: Vec<String> {
        ident[arg] => vec![arg],
        ident[arg] define_args[mut second_arg] => {
            let mut arg = vec![arg];
            arg.append(&mut second_arg);
            arg
        }
    }

    ident: String {
        Ident(a) => a
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}
