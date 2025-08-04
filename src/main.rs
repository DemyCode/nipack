use std::{env, fs};

use rnix::ast::HasEntry;

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
                println!("error reading file: {}", err);
                return;
            }
        };
        let ast = rnix::Root::parse(&content).ok().unwrap();
        let expr = ast.expr().unwrap();
        let new_expr = match expr {
            rnix::ast::Expr::AttrSet(attr_set) => {
                for attr in attr_set.entries() {
                    match attr {
                        rnix::ast::Entry::Inherit(inherit) => todo!(),
                        rnix::ast::Entry::AttrpathValue(attrpath_value) => todo!(),
                    }
                }
            }

            _ => {
                eprintln!("Expected an attribute set, found: {:?}", expr);
                return;
            }
        };
    }
}
