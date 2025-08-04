use std::{env, fs};

use rnix::ast::{AttrSet, AttrpathValue, Entry, Expr, HasEntry};

fn main() {
    let mut iter = env::args().skip(1).peekable();
    if iter.peek().is_none() {
        eprintln!("Usage: dump-ast <file>");
        return;
    }
    for file in iter {
        let content = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(err) => {
                println!("error reading file: {err}");
                return;
            }
        };
        let ast = rnix::Root::parse(&content).ok().unwrap();
        let expr = ast.expr().unwrap();
        let new_expr: Option<Expr> = match expr {
            rnix::ast::Expr::AttrSet(attr_set) => {
                let copy = attr_set.entries();
                Some(rnix::ast::Expr::AttrSet(attr_set))
            }
            _ => None,
        };

        println!("Parsed successfully: {:?}", new_expr);
    }
}
