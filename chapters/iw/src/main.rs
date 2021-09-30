extern crate swc_common;
extern crate swc_ecma_ast;
extern crate swc_ecma_parser;

use std::path::Path;

use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use swc_ecma_ast::Lit::Num;

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm
        .load_file(Path::new("./src/test.js"))
        .expect("failed to load test.js");
    // let fm = cm.new_source_file(
    //     FileName::Custom("test.js".into()),
    //     "function foo() {}".into(),
    // );
    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(Default::default()),
        // JscTarget defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let _module = parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parser module");

    for b in _module.body {
        if b.is_stmt() {
            let stmt = b.stmt().unwrap();
            if stmt.is_expr() {
                let expr_stmt = stmt.expr().unwrap();
                let expr = expr_stmt.expr;
                if expr.is_bin() {
                    let bin_expr = expr.bin().unwrap();
                    let left_expr = bin_expr.left;
                    let right_expr = bin_expr.right;
                    if left_expr.is_lit() && right_expr.is_lit() {
                        let left_lit = left_expr.lit().unwrap();
                        let right_lit = right_expr.lit().unwrap();
                        let left_value = match left_lit {
                            Num(n) => n.value,
                            _ => 0f64,
                        };
                        let right_value = match right_lit {
                            Num(n) => n.value,
                            _ => 0f64,
                        };
                    }
                }
            }
        }
    }
}
