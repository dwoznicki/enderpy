use parser::ast::{Expression, Statement};

use crate::{
    ast_visitor::TraversalVisitor,
    state::State,
    symbol_table::{NodeType, SymbolTableNode, SymbolScope}, semanal_utils::{self, get_expr_type},
};

pub struct SemanticAnalyzer<'a> {
    state: &'a State<'a>,
    // TODO: Replace errors with another type
    errors: Vec<String>,
    scope: SymbolScope,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(state: &State) -> Self {
        return SemanticAnalyzer {
            state,
            errors: vec![],
            scope: SymbolScope::Global
        };
    }

    pub fn analyze(&self) {
        for stmt in self.state.file.defs {
            self.visit_stmt(&stmt)
        }
    }

    fn add_expression_to_symbol_table(&self, name: String, node_type: NodeType, node: &Statement, public: bool) {
        let symbol_node = SymbolTableNode{ name, node: *node, typ: node_type, module_public: public, module_hidden: false, implicit: false, scope: self.scope }
        self.state.file.names.add_symbol(name, symbol_node)
    }
}

// Get defined names in the statement. A single statement can define more than one symbol.
// e.g. a, b = 1, 1
fn get_names(statement: &Statement) -> Vec<String> {
    match statement {
        parser::ast::Statement::AssignStatement(a) => a
            .targets
            .into_iter()
            .map(|f| get_expr_bind_name(&f))
            .collect(),
        _ => panic!("not implemented"),
    }
}

fn get_expr_bind_name(expr: &Expression) -> String {
    match expr {
        parser::ast::Expression::Name(n) => n.id,
        _ => panic!("not implemented"),
    }
}

impl<'a> TraversalVisitor for SemanticAnalyzer<'a> {
    fn visit_stmt(&mut self, s: &parser::ast::Statement) {
        // map all statements and call visit
        match s {
            parser::ast::Statement::ExpressionStatement(e) => self.visit_expr(e),
            parser::ast::Statement::Import(i) => self.visit_import(i),
            parser::ast::Statement::ImportFrom(i) => self.visit_import_from(i),
            parser::ast::Statement::AssignStatement(a) => self.visit_assign(a),
            parser::ast::Statement::AnnAssignStatement(a) => self.visit_ann_assign(a),
            parser::ast::Statement::AugAssignStatement(a) => self.visit_aug_assign(a),
            parser::ast::Statement::Assert(a) => self.visit_assert(a),
            parser::ast::Statement::Pass(p) => self.visit_pass(p),
            parser::ast::Statement::Delete(d) => self.visit_delete(d),
            parser::ast::Statement::Return(r) => self.visit_return(r),
            parser::ast::Statement::Raise(r) => self.visit_raise(r),
            parser::ast::Statement::Break(b) => self.visit_break(b),
            parser::ast::Statement::Continue(c) => self.visit_continue(c),
            parser::ast::Statement::Global(g) => self.visit_global(g),
            parser::ast::Statement::Nonlocal(n) => self.visit_nonlocal(n),
            parser::ast::Statement::IfStatement(i) => self.visit_if(i),
            parser::ast::Statement::WhileStatement(w) => self.visit_while(w),
            parser::ast::Statement::ForStatement(f) => self.visit_for(f),
            parser::ast::Statement::WithStatement(w) => self.visit_with(w),
            parser::ast::Statement::TryStatement(t) => self.visit_try(t),
            parser::ast::Statement::TryStarStatement(t) => self.visit_try_star(t),
            parser::ast::Statement::FunctionDef(f) => self.visit_function_def(f),
            parser::ast::Statement::ClassDef(c) => self.visit_class_def(c),
            parser::ast::Statement::Match(m) => self.visit_match(m),
        }
    }

    fn visit_expr(&mut self, e: &parser::ast::Expression) {
        match e {
            parser::ast::Expression::Constant(c) => self.visit_constant(c),
            parser::ast::Expression::List(l) => self.visit_list(l),
            parser::ast::Expression::Tuple(t) => self.visit_tuple(t),
            parser::ast::Expression::Dict(d) => self.visit_dict(d),
            parser::ast::Expression::Set(s) => self.visit_set(s),
            parser::ast::Expression::Name(n) => self.visit_name(n),
            parser::ast::Expression::BoolOp(b) => self.visit_bool_op(b),
            parser::ast::Expression::UnaryOp(u) => self.visit_unary_op(u),
            parser::ast::Expression::BinOp(b) => self.visit_bin_op(b),
            parser::ast::Expression::NamedExpr(n) => self.visit_named_expr(n),
            parser::ast::Expression::Yield(y) => self.visit_yield(y),
            parser::ast::Expression::YieldFrom(y) => self.visit_yield_from(y),
            parser::ast::Expression::Starred(s) => self.visit_starred(s),
            parser::ast::Expression::Generator(g) => self.visit_generator(g),
            parser::ast::Expression::ListComp(l) => self.visit_list_comp(l),
            parser::ast::Expression::SetComp(s) => self.visit_set_comp(s),
            parser::ast::Expression::DictComp(d) => self.visit_dict_comp(d),
            parser::ast::Expression::Attribute(a) => self.visit_attribute(a),
            parser::ast::Expression::Subscript(s) => self.visit_subscript(s),
            parser::ast::Expression::Slice(s) => self.visit_slice(s),
            parser::ast::Expression::Call(c) => self.visit_call(c),
            parser::ast::Expression::Await(a) => self.visit_await(a),
            parser::ast::Expression::Compare(c) => self.visit_compare(c),
            parser::ast::Expression::Lambda(l) => self.visit_lambda(l),
            parser::ast::Expression::IfExp(i) => self.visit_if_exp(i),
            parser::ast::Expression::JoinedStr(j) => self.visit_joined_str(j),
            parser::ast::Expression::FormattedValue(f) => self.visit_formatted_value(f),
        }
    }

    fn visit_import(&mut self, i: &parser::ast::Import) {
        todo!();
    }

    fn visit_import_from(&mut self, i: &parser::ast::ImportFrom) {
        todo!();
    }

    fn visit_if(&mut self, i: &parser::ast::If) {
        for stmt in &i.body {
            self.visit_stmt(&stmt);
        }
        for stmt in &i.orelse {
            self.visit_stmt(&stmt);
        }
    }

    fn visit_while(&mut self, w: &parser::ast::While) {
        for stmt in &w.body {
            self.visit_stmt(&stmt)
        }
    }

    fn visit_for(&mut self, f: &parser::ast::For) {
        for stmt in &f.body {
            self.visit_stmt(&stmt);
        }
    }

    fn visit_with(&mut self, w: &parser::ast::With) {
        for stmt in &w.body {
            self.visit_stmt(&stmt);
        }
        for with_items in &w.items {
            self.visit_expr(&*&with_items.context_expr);
            match &with_items.optional_vars {
                Some(items) => self.visit_expr(&items),
                None => (),
            }
        }
    }

    fn visit_try(&mut self, t: &parser::ast::Try) {
        for stmt in &t.body {
            self.visit_stmt(&stmt);
        }
        for stmt in &t.orelse {
            self.visit_stmt(&stmt);
        }
        for stmt in &t.finalbody {
            self.visit_stmt(&stmt);
        }
        // TODO: need to visit exception handler name and type but let's keep it simple for now
        for handler in &t.handlers {
            for stmt in &handler.body {
                self.visit_stmt(&stmt);
            }
        }
    }

    fn visit_try_star(&mut self, t: &parser::ast::TryStar) {
        for stmt in &t.body {
            self.visit_stmt(&stmt);
        }
        for stmt in &t.orelse {
            self.visit_stmt(&stmt);
        }
        for stmt in &t.finalbody {
            self.visit_stmt(&stmt);
        }
        // TODO: need to visit exception handler name and type but let's keep it simple for now
        for handler in &t.handlers {
            for stmt in &handler.body {
                self.visit_stmt(&stmt);
            }
        }
    }

    fn visit_function_def(&mut self, f: &parser::ast::FunctionDef) {
        for stmt in &f.body {
            self.visit_stmt(&stmt);
        }
    }

    fn visit_class_def(&mut self, c: &parser::ast::ClassDef) {
        for stmt in &c.body {
            self.visit_stmt(&stmt);
        }
    }

    fn visit_match(&mut self, m: &parser::ast::Match) {
        for case in &m.cases {
            for stmt in &case.body {
                self.visit_stmt(&stmt);
            }
        }
    }

    fn visit_constant(&mut self, c: &parser::ast::Constant) {
        todo!()
    }

    fn visit_list(&mut self, l: &parser::ast::List) {
        todo!()
    }

    fn visit_tuple(&mut self, t: &parser::ast::Tuple) {
        todo!()
    }

    fn visit_dict(&mut self, d: &parser::ast::Dict) {
        todo!()
    }

    fn visit_set(&mut self, s: &parser::ast::Set) {
        todo!()
    }

    fn visit_name(&mut self, n: &parser::ast::Name) {
        todo!()
    }

    fn visit_bool_op(&mut self, b: &parser::ast::BoolOperation) {
        todo!()
    }

    fn visit_unary_op(&mut self, u: &parser::ast::UnaryOperation) {
        todo!()
    }

    fn visit_bin_op(&mut self, b: &parser::ast::BinOp) {
        todo!()
    }

    fn visit_named_expr(&mut self, n: &parser::ast::NamedExpression) {
        todo!()
    }

    fn visit_yield(&mut self, y: &parser::ast::Yield) {
        todo!()
    }

    fn visit_yield_from(&mut self, y: &parser::ast::YieldFrom) {
        todo!()
    }

    fn visit_starred(&mut self, s: &parser::ast::Starred) {
        todo!()
    }

    fn visit_generator(&mut self, g: &parser::ast::Generator) {
        todo!()
    }

    fn visit_list_comp(&mut self, l: &parser::ast::ListComp) {
        todo!()
    }

    fn visit_set_comp(&mut self, s: &parser::ast::SetComp) {
        todo!()
    }

    fn visit_dict_comp(&mut self, d: &parser::ast::DictComp) {
        todo!()
    }

    fn visit_attribute(&mut self, a: &parser::ast::Attribute) {
        todo!()
    }

    fn visit_subscript(&mut self, s: &parser::ast::Subscript) {
        todo!()
    }

    fn visit_slice(&mut self, s: &parser::ast::Slice) {
        todo!()
    }

    fn visit_call(&mut self, c: &parser::ast::Call) {
        todo!()
    }

    fn visit_await(&mut self, a: &parser::ast::Await) {
        todo!()
    }

    fn visit_compare(&mut self, c: &parser::ast::Compare) {
        todo!()
    }

    fn visit_lambda(&mut self, l: &parser::ast::Lambda) {
        todo!()
    }

    fn visit_if_exp(&mut self, i: &parser::ast::IfExp) {
        todo!()
    }

    fn visit_joined_str(&mut self, j: &parser::ast::JoinedStr) {
        todo!()
    }

    fn visit_formatted_value(&mut self, f: &parser::ast::FormattedValue) {
        todo!()
    }

    fn visit_alias(&mut self, a: &parser::ast::Alias) {
        todo!()
    }

    // TODO: Maybe we need to provide the full ast::Statement instead of the enum value?
    // Not sure but it makes it easier to store the node in Symbol Table
    // But puts more weight on the traverse code to decode the type
    fn visit_assign(&mut self, a: &parser::ast::Assign) {
        if a.targets.len() > 1 {
            panic!("assignment to multiple targets not implemented")
            
        }
        let name = semanal_utils::get_target_name(&a.targets.last().unwrap());
        let nt = get_expr_type(&a.value);
        self.add_expression_to_symbol_table(name, nt,&Statement::AssignStatement(*a), false);
        
    }

    fn visit_ann_assign(&mut self, a: &parser::ast::AnnAssign) {
        todo!()
    }

    fn visit_aug_assign(&mut self, a: &parser::ast::AugAssign) {
        todo!()
    }

    fn visit_assert(&mut self, a: &parser::ast::Assert) {
        todo!()
    }

    fn visit_pass(&mut self, p: &parser::ast::Pass) {
        todo!()
    }

    fn visit_delete(&mut self, d: &parser::ast::Delete) {
        todo!()
    }

    fn visit_return(&mut self, r: &parser::ast::Return) {
        todo!()
    }

    fn visit_raise(&mut self, r: &parser::ast::Raise) {
        todo!()
    }

    fn visit_break(&mut self, b: &parser::ast::Break) {
        todo!()
    }

    fn visit_continue(&mut self, c: &parser::ast::Continue) {
        todo!()
    }

    fn visit_global(&mut self, g: &parser::ast::Global) {
        todo!()
    }

    fn visit_nonlocal(&mut self, n: &parser::ast::Nonlocal) {
        todo!()
    }
}