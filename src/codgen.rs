use std::collections::HashMap;

use inkwell::{module::*, context::Context, execution_engine::{ExecutionEngine, JitFunction}, values::PointerValue};

use crate::syntax::*;


pub struct JITBackend<'ctxt> {
    ctxt: &'ctxt Context,
    modu: Module<'ctxt>,
    xeng: ExecutionEngine<'ctxt>,
    bind: HashMap<String, Vec<(Type, PointerValue<'ctxt>)>>,
}

pub enum CompileError {
}

impl<'ctxt> JITBackend<'ctxt> {
    fn tycheck(&self, item: Item) -> Result<(), CompileError> {
        match item {
            Item::Expr(expr) => {
            },
            Item::Stmt(stmt) => {
            }
        }
        Ok(())
    }
    fn compile(&self, item: Item) -> Result<JitFunction<'ctxt, ()>, CompileError> {
        match item {
            Item::Expr(expr) => {
                todo!()
            },
            Item::Stmt(stmt) => {
                todo!()
            }
        }
    }
    fn execute(&self) {
    }
}
