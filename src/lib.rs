pub mod ast;
pub mod error;
pub mod parser;

use crate::{error::Error, parser::ProgramParser};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "js")]
use oxc::{
    allocator::{
        Allocator,
        // Box as OxcBox, Vec as OxcVec
    },
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
            SourceType::mjs(),
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
            ast::Stmt::Decl(ast::Decl { ident, ty: _, expr }) => {
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
            ast::Stmt::Func(ast::Func { ident, stmts }) => {
                oxc_ast::Statement::FunctionDeclaration(ast_builder.alloc_function(
                    oxc_ast::FunctionType::FunctionDeclaration,
                    Span::default(),
                    Some(ast_builder.binding_identifier(Span::default(), ident)),
                    false,
                    false,
                    false,
                    None::<oxc_ast::TSTypeParameterDeclaration>,
                    None::<oxc_ast::TSThisParameter>,
                    ast_builder.alloc_formal_parameters(
                        Span::default(),
                        oxc_ast::FormalParameterKind::FormalParameter,
                        ast_builder.vec(),
                        None::<oxc_ast::BindingRestElement>,
                    ),
                    None::<oxc_ast::TSTypeAnnotation>,
                    Some(ast_builder.alloc_function_body(
                        Span::default(),
                        ast_builder.vec(),
                        ast_builder.vec_from_iter(stmts.iter().map(|s| self.convert_stmt(s))),
                    )),
                ))
            }
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
            ast::Expr::Call(ast::Call { ident, args }) => ast_builder.expression_call(
                Span::default(),
                ast_builder.expression_identifier_reference(Span::default(), ident),
                None::<oxc_ast::TSTypeParameterInstantiation>,
                ast_builder.vec_from_iter(
                    args.iter()
                        .map(|arg| oxc_ast::Argument::from(self.convert_expr(arg))),
                ),
                false,
            ),
            ast::Expr::Cozy(value) => {
                ast_builder.expression_identifier_reference(Span::default(), value)
            }
        }
    }
}

const JS_PRELUDE: &str = include_str!("prelude.js");

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

    format!("{JS_PRELUDE}{}", js.code)
}
