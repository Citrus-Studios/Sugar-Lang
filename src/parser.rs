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
    Mod(Box<Expr>, Box<Expr>),

    Eq(Box<Expr>, Box<Expr>),
    NEq(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    EGt(Box<Expr>, Box<Expr>),
    ELt(Box<Expr>, Box<Expr>),
    LNot(Box<Expr>),
    LAnd(Box<Expr>, Box<Expr>),
    LOr(Box<Expr>, Box<Expr>),

    Var(String),

    Declare(String, Vec<String>),
    Define(String, Vec<String>, Vec<Expr>),
    FunctionCall(String, Vec<Expr>),

    Assign(String, Box<Expr>),
    ReAssign(String, Box<Expr>),

    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),
    ForLoop(Box<Expr>, Box<Expr>, Box<Expr>, Vec<Expr>),

    Byte(u8),

    Pass,
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
        Declare Ident(name) Equals declare_args[args] => Expr {
            span: span!(),
            node: Expr_::Declare(name, args)
        },
        Define Ident(name) define_args[args] Equals exprs[block] => Expr {
            span: span!(),
            node: Expr_::Define(name, args, block)
        },
        Define Ident(name) Equals exprs[block] => Expr {
            span: span!(),
            node: Expr_::Define(name, vec![], block)
        }
    }

    exprwrap: Vec<Expr> {
        expr[a] => a,
        => vec![]
    }

    expr: Vec<Expr> {
        Variable Ident(name) Equals exprs[e] SemiColon exprwrap[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::Assign(name, Box::new(e[0].clone()))
            }];
            e.append(&mut m);
            e
        },
        Mutate Ident(name) Equals exprs[e] SemiColon exprwrap[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::ReAssign(name, Box::new(e[0].clone()))
            }];
            e.append(&mut m);
            e
        },
        Return Byte(e) SemiColon exprwrap[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::Byte(e)
            }];
            e.append(&mut m);
            e
        },
        Return Ident(name) SemiColon exprwrap[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::Var(name)
            }];
            e.append(&mut m);
            e
        },
        Return expr[e] SemiColon => e,
        If exprs[e] SemiColon LBrace expr[b] RBrace SemiColon Else SemiColon LBrace expr[c] RBrace SemiColon exprwrap[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::IfElse(Box::new(e[0].clone()), b.clone(), c.clone())
            }];
            e.append(&mut m);
            e
        },
        For expr[a] term[b] SemiColon expr[c] LBrace expr[e] RBrace SemiColon exprwrap[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::ForLoop(Box::new(a[0].clone()), Box::new(b.clone()), Box::new(c[0].clone()), e.clone())
            }];
            e.append(&mut m);
            e
        },
        Pass SemiColon exprwrap[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::Pass
            }];
            e.append(&mut m);
            e
        }
    }

    exprs: Vec<Expr> {
        LBrace expr[e] RBrace => e,
        term[a] => vec![a],
        Pass SemiColon exprwrap[mut m] => {
            let mut e = vec![Expr {
                span: span!(),
                node: Expr_::Pass
            }];
            e.append(&mut m);
            e
        }
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
        atom[a] Equals Equals atom[b] => Expr {
            span: span!(),
            node: Expr_::Eq(Box::new(a), Box::new(b))
        },
        atom[a] Bang Equals atom[b] => Expr {
            span: span!(),
            node: Expr_::NEq(Box::new(a), Box::new(b))
        },
        atom[a] Gt atom[b] => Expr {
            span: span!(),
            node: Expr_::Gt(Box::new(a), Box::new(b))
        },
        atom[a] Lt atom[b] => Expr {
            span: span!(),
            node: Expr_::Lt(Box::new(a), Box::new(b))
        },
        atom[a] Gt Equals atom[b] => Expr {
            span: span!(),
            node: Expr_::EGt(Box::new(a), Box::new(b))
        },
        atom[a] Lt Equals atom[b] => Expr {
            span: span!(),
            node: Expr_::ELt(Box::new(a), Box::new(b))
        },
        atom[a] Pipe Pipe atom[b] => Expr {
            span: span!(),
            node: Expr_::LOr(Box::new(a), Box::new(b))
        },
        atom[a] Ampersand Ampersand atom[b] => Expr {
            span: span!(),
            node: Expr_::LAnd(Box::new(a), Box::new(b))
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
        fact[a] Percent atom[b] => Expr {
            span: span!(),
            node: Expr_::Mod(Box::new(a), Box::new(b))
        },
        atom[a] => a
    }

    atom: Expr {
        Ident(a) => Expr {
            span: span!(),
            node: Expr_::Var(a)
        },
        Byte(x) => Expr {
            span: span!(),
            node: Expr_::Byte(x)
        },
        LParen term[a] RParen => a,
        Minus atom[b] => Expr {
            span: span!(),
            node: Expr_::Sub(Box::new(Expr {
                span: span!(),
                node: Expr_::Byte(0)
            }), Box::new(b))
        },
    }

    declare_args: Vec<String> {
        Ident(arg) => vec![arg],
        Bang => vec![String::from("void")],
        Ident(arg) Minus Gt declare_args[mut second_arg] => {
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
        Ident(arg) => vec![arg],
        Ident(arg) define_args[mut second_arg] => {
            let mut arg = vec![arg];
            arg.append(&mut second_arg);
            arg
        }
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}
