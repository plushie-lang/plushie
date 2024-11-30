pub mod ast;
pub mod error;
pub mod parser;

use crate::{error::Error, parser::ProgramParser};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "js")]
use oxc::{
    allocator::{Allocator, Box as OxcBox, Vec as OxcVec},
    ast::{ast as oxc_ast, AstBuilder},
    codegen::Codegen,
    span::{SourceType, Span},
};

struct AstConverter<'a> {
    ast_builder: AstBuilder<'a>,
}

impl<'a> AstConverter<'a> {
    pub fn new(ast_builder: AstBuilder<'a>) -> Self {
        Self { ast_builder }
    }

    pub fn convert(&'a self, program: ast::Program) -> oxc_ast::Program<'a> {
        let ast_builder = self.ast_builder;

        let body =
            ast_builder.vec_from_iter(program.stmts.iter().map(|stmt| self.convert_stmt(stmt)));

        ast_builder.program(
            Span::default(),
            SourceType::cjs(),
            "",
            ast_builder.vec(),
            None,
            ast_builder.vec(),
            body,
        )
    }

    fn convert_stmt(&'a self, stmt: &ast::Stmt) -> oxc_ast::Statement<'a> {
        let ast_builder = self.ast_builder;

        match stmt {
            ast::Stmt::Decl(ast::Decl { ident, ty, expr }) => {
                oxc_ast::Statement::VariableDeclaration(ast_builder.alloc_variable_declaration(
                    Span::default(),
                    oxc_ast::VariableDeclarationKind::Let,
                    ast_builder.vec1(
                        ast_builder.variable_declarator(
                            Span::default(),
                            oxc_ast::VariableDeclarationKind::Let,
                            ast_builder.binding_pattern(
                                ast_builder.binding_pattern_kind_binding_identifier(
                                    Span::default(),
                                    ident,
                                ),
                                None::<oxc_ast::TSTypeAnnotation>,
                                false,
                            ),
                            Some(self.convert_expr(expr)),
                            false,
                        ),
                    ),
                    false,
                ))
            }
            ast::Stmt::Func(func) => oxc_ast::Statement::EmptyStatement(
                ast_builder.alloc_empty_statement(Span::default()),
            ),
            ast::Stmt::Expr(expr) => oxc_ast::Statement::ExpressionStatement(
                ast_builder.alloc_expression_statement(Span::default(), self.convert_expr(expr)),
            ),
        }
    }

    fn convert_expr(&self, expr: &ast::Expr) -> oxc_ast::Expression<'a> {
        let ast_builder = self.ast_builder;

        match expr {
            ast::Expr::Ribbon(value) => {
                ast_builder.expression_string_literal(Span::default(), value)
            }
            ast::Expr::Call(call) => {
                todo!();

                // ast_builder.expression_call(
                //     Span::default(),
                //     // ast_builder.expression_call(Span::default(), callee, type_parameters, arguments, optional),
                //     todo!(),
                // )
            }
            ast::Expr::Cozy(_) => todo!(),
        }
    }
}

#[cfg(feature = "js")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn compile_to_js(source: &str) -> String {
    let parser = ProgramParser::new();

    let program = parser
        .parse(&source)
        .map_err(|error| Error::from_parse_error("unknown", &source, error))
        .expect("Failed");

    let allocator = Allocator::default();

    let converter = AstConverter::new(AstBuilder::new(&allocator));
    let program = converter.convert(program);
    let js = Codegen::new().build(&program);

    js.code
}
