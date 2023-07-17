// This file holds nodes that represent the AST of a file.
// This holds the same information as the parser AST, but
// in a more convenient format for the type checker.
// NOTE: at this point we're not sure what nodes need to be
// here, so this has the minimum amount of nodes needed to
// get the type checker working. But can be expanded.

use parser::ast::{Import, ImportFrom, Module, Statement};

use crate::{
    ast_visitor::TraversalVisitor,
    symbol_table::{SymbolTable, SymbolTableType},
};

pub enum ImportKinds {
    Import(Import),
    ImportFrom(ImportFrom),
}

pub struct EnderpyFile {
    pub ast: Module,
    pub names: SymbolTable,
    // all the imports inside the file
    pub imports: Vec<ImportKinds>,
    // high level definitions inside the file
    pub defs: Vec<Statement>,
}

impl EnderpyFile {
    pub fn from(ast: &Module) -> Self {
        let mut converter = ASTConverter::new();
        converter.convert(ast)
    }
}

/// Converts python AST to Enderpy file. This is a high level structure used by
/// the rest of type checker components.
/// This has many responsibilities and tightly coupled to EnderpyFile.
struct ASTConverter {
    imports: Vec<ImportKinds>,
    defs: Vec<Statement>,
}

impl ASTConverter {
    pub fn new() -> Self {
        return ASTConverter {
            imports: vec![],
            defs: vec![],
        };
    }
}

impl ASTConverter {
    fn convert(&mut self, tree: &Module) -> EnderpyFile {
        for stmt in &tree.body {
            self.visit_stmt(stmt);
        }

        EnderpyFile {
            ast: *tree,
            names: SymbolTable::new(SymbolTableType::Module, 1),
            defs: self.defs,
            imports: self.imports,
        }
    }
}

impl TraversalVisitor for ASTConverter {
    fn visit_import(&mut self, i: &Import) {
        let import = i.clone();
        self.imports.push(ImportKinds::Import(import));
    }

    fn visit_import_from(&mut self, i: &ImportFrom) {
        let import = i.clone();
        self.imports.push(ImportKinds::ImportFrom(import));
    }

    fn visit_function_def(&mut self, f: &parser::ast::FunctionDef) {
        let func = f.clone();
        self.defs.push(Statement::FunctionDef(func));
    }
}