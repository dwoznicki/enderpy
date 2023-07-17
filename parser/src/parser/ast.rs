use miette::{SourceOffset, SourceSpan};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)] // #[serde(tag = "type")]
pub struct Node {
    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,
}

impl Node {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl From<Node> for SourceSpan {
    fn from(val: Node) -> Self {
        Self::new(
            SourceOffset::from(val.start as usize),
            SourceOffset::from(val.len() as usize),
        )
    }
}

// The following structs are used to represent the AST
// https://docs.python.org/3/library/ast.html#abstract-grammar
#[derive(Debug, Clone)]
pub struct Module {
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    AssignStatement(Assign),
    AnnAssignStatement(AnnAssign),
    AugAssignStatement(AugAssign),
    ExpressionStatement(Expression),
    Assert(Assert),
    Pass(Pass),
    Delete(Delete),
    Return(Return),
    Raise(Raise),
    Break(Break),
    Continue(Continue),
    Import(Import),
    ImportFrom(ImportFrom),
    Global(Global),
    Nonlocal(Nonlocal),
    IfStatement(If),
    WhileStatement(While),
    ForStatement(For),
    WithStatement(With),
    TryStatement(Try),
    TryStarStatement(TryStar),
    FunctionDef(FunctionDef),
    ClassDef(ClassDef),
    Match(Match),
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub node: Node,
    pub targets: Vec<Expression>,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct AnnAssign {
    pub node: Node,
    pub target: Expression,
    pub annotation: Expression,
    pub value: Option<Expression>,
    pub simple: bool,
}

#[derive(Debug, Clone)]
pub struct AugAssign {
    pub node: Node,
    pub target: Expression,
    pub op: AugAssignOp,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub enum AugAssignOp {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

#[derive(Debug, Clone)]
pub struct Assert {
    pub node: Node,
    pub test: Expression,
    pub msg: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Pass {
    pub node: Node,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub node: Node,
    pub targets: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub node: Node,
    pub value: Option<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.Raise
#[derive(Debug, Clone)]
pub struct Raise {
    pub node: Node,
    pub exc: Option<Expression>,
    pub cause: Option<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.Break
#[derive(Debug, Clone)]
pub struct Break {
    pub node: Node,
}

// https://docs.python.org/3/library/ast.html#ast.Continue
#[derive(Debug, Clone)]
pub struct Continue {
    pub node: Node,
}

// https://docs.python.org/3/library/ast.html#ast.Import
#[derive(Debug, Clone)]
pub struct Import {
    pub node: Node,
    pub names: Vec<Alias>,
}

// https://docs.python.org/3/library/ast.html#ast.alias
#[derive(Debug, Clone)]
pub struct Alias {
    pub node: Node,
    pub name: String,
    pub asname: Option<String>,
}

// https://docs.python.org/3/library/ast.html#ast.ImportFrom
#[derive(Debug, Clone)]
pub struct ImportFrom {
    pub node: Node,
    pub module: String,
    pub names: Vec<Alias>,
    pub level: usize,
}

// https://docs.python.org/3/library/ast.html#ast.Global
#[derive(Debug, Clone)]
pub struct Global {
    pub node: Node,
    pub names: Vec<String>,
}

// https://docs.python.org/3/library/ast.html#ast.Nonlocal
#[derive(Debug, Clone)]
pub struct Nonlocal {
    pub node: Node,
    pub names: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(Box<Constant>),
    List(Box<List>),
    Tuple(Box<Tuple>),
    Dict(Box<Dict>),
    Set(Box<Set>),
    Name(Box<Name>),
    BoolOp(Box<BoolOperation>),
    UnaryOp(Box<UnaryOperation>),
    BinOp(Box<BinOp>),
    NamedExpr(Box<NamedExpression>),
    Yield(Box<Yield>),
    YieldFrom(Box<YieldFrom>),
    Starred(Box<Starred>),
    Generator(Box<Generator>),
    ListComp(Box<ListComp>),
    SetComp(Box<SetComp>),
    DictComp(Box<DictComp>),
    Attribute(Box<Attribute>),
    Subscript(Box<Subscript>),
    Slice(Box<Slice>),
    Call(Box<Call>),
    Await(Box<Await>),
    Compare(Box<Compare>),
    Lambda(Box<Lambda>),
    IfExp(Box<IfExp>),
    JoinedStr(Box<JoinedStr>),
    FormattedValue(Box<FormattedValue>),
}

// https://docs.python.org/3/reference/expressions.html#atom-identifiers
#[derive(Debug, Clone)]
pub struct Name {
    pub node: Node,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Constant {
    pub node: Node,
    pub value: ConstantValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantValue {
    None,
    Bool(bool),
    Str(String),
    Bytes(Vec<u8>),
    Tuple(Vec<Constant>),
    // Numbers are string because we don't care about the value rn.
    Int(String),
    Float(String),
    Complex { real: String, imaginary: String },
}

#[derive(Debug, Clone)]
pub struct List {
    pub node: Node,
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct Tuple {
    pub node: Node,
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct Dict {
    pub node: Node,
    pub keys: Vec<Expression>,
    pub values: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub node: Node,
    pub elements: Vec<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.BoolOp
#[derive(Debug, Clone)]
pub struct BoolOperation {
    pub node: Node,
    pub op: BooleanOperator,
    pub values: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum BooleanOperator {
    And,
    Or,
}

// https://docs.python.org/3/library/ast.html#ast.UnaryOp
#[derive(Debug, Clone)]
pub struct UnaryOperation {
    pub node: Node,
    pub op: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Invert,
    UAdd,
    USub,
}

// https://docs.python.org/3/library/ast.html#ast.BinOp
#[derive(Debug, Clone)]
pub struct BinOp {
    pub node: Node,
    pub op: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

// https://docs.python.org/3/library/ast.html#ast.NamedExpr
#[derive(Debug, Clone)]
pub struct NamedExpression {
    pub node: Node,
    pub target: Box<Expression>,
    pub value: Box<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.Yield
#[derive(Debug, Clone)]
pub struct Yield {
    pub node: Node,
    pub value: Option<Box<Expression>>,
}

// https://docs.python.org/3/library/ast.html#ast.YieldFrom
#[derive(Debug, Clone)]
pub struct YieldFrom {
    pub node: Node,
    pub value: Box<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.Starred
#[derive(Debug, Clone)]
pub struct Starred {
    pub node: Node,
    pub value: Box<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.GeneratorExp
#[derive(Debug, Clone)]
pub struct Generator {
    pub node: Node,
    pub element: Box<Expression>,
    pub generators: Vec<Comprehension>,
}

#[derive(Debug, Clone)]
pub struct ListComp {
    pub node: Node,
    pub element: Box<Expression>,
    pub generators: Vec<Comprehension>,
}

#[derive(Debug, Clone)]
pub struct SetComp {
    pub node: Node,
    pub element: Box<Expression>,
    pub generators: Vec<Comprehension>,
}

#[derive(Debug, Clone)]
pub struct DictComp {
    pub node: Node,
    pub key: Box<Expression>,
    pub value: Box<Expression>,
    pub generators: Vec<Comprehension>,
}

// https://docs.python.org/3/library/ast.html#ast.comprehension
#[derive(Debug, Clone)]
pub struct Comprehension {
    pub node: Node,
    pub target: Box<Expression>,
    pub iter: Box<Expression>,
    pub ifs: Vec<Expression>,
    pub is_async: bool,
}

// https://docs.python.org/3/library/ast.html#ast.Attribute
#[derive(Debug, Clone)]
pub struct Attribute {
    pub node: Node,
    pub value: Box<Expression>,
    pub attr: String,
}

// https://docs.python.org/3/library/ast.html#ast.Subscript
#[derive(Debug, Clone)]
pub struct Subscript {
    pub node: Node,
    pub value: Box<Expression>,
    pub slice: Box<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.Slice
// can be used for Subscript
#[derive(Debug, Clone)]
pub struct Slice {
    pub node: Node,
    pub lower: Option<Box<Expression>>,
    pub upper: Option<Box<Expression>>,
    pub step: Option<Box<Expression>>,
}

// https://docs.python.org/3/library/ast.html#ast.Call
#[derive(Debug, Clone)]
pub struct Call {
    pub node: Node,
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
    pub keywords: Vec<Keyword>,
    pub starargs: Option<Box<Expression>>,
    pub kwargs: Option<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct Keyword {
    pub node: Node,
    pub arg: Option<String>,
    pub value: Box<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.Await
#[derive(Debug, Clone)]
pub struct Await {
    pub node: Node,
    pub value: Box<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.Compare
#[derive(Debug, Clone)]
pub struct Compare {
    pub node: Node,
    pub left: Box<Expression>,
    pub ops: Vec<ComparisonOperator>,
    pub comparators: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

// https://docs.python.org/3/library/ast.html#ast.Lambda
#[derive(Debug, Clone)]
pub struct Lambda {
    pub node: Node,
    pub args: Arguments,
    pub body: Box<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.arguments
#[derive(Debug, Clone)]
pub struct Arguments {
    pub node: Node,
    pub posonlyargs: Vec<Arg>,
    pub args: Vec<Arg>,
    pub vararg: Option<Arg>,
    pub kwonlyargs: Vec<Arg>,
    pub kw_defaults: Vec<Option<Expression>>,
    pub kwarg: Option<Arg>,
    pub defaults: Vec<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.arg
#[derive(Debug, Clone)]
pub struct Arg {
    pub node: Node,
    pub arg: String,
    pub annotation: Option<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.IfExp
#[derive(Debug, Clone)]
pub struct IfExp {
    pub node: Node,
    pub test: Box<Expression>,
    pub body: Box<Expression>,
    pub orelse: Box<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.FormattedValue
#[derive(Debug, Clone)]
pub struct FormattedValue {
    pub node: Node,
    pub value: Box<Expression>,
    pub conversion: Option<i32>,
    pub format_spec: Option<Box<Expression>>,
}

// https://docs.python.org/3/library/ast.html#ast.JoinedStr
#[derive(Debug, Clone)]
pub struct JoinedStr {
    pub node: Node,
    pub values: Vec<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.If
#[derive(Debug, Clone)]
pub struct If {
    pub node: Node,
    pub test: Box<Expression>,
    pub body: Vec<Statement>,
    pub orelse: Vec<Statement>,
}

impl If {
    pub fn update_orelse(&mut self, other_or_else: Vec<Statement>) {
        self.orelse = other_or_else;
    }
}

// https://docs.python.org/3/library/ast.html#ast.While
#[derive(Debug, Clone)]
pub struct While {
    pub node: Node,
    pub test: Box<Expression>,
    pub body: Vec<Statement>,
    pub orelse: Vec<Statement>,
}

// https://docs.python.org/3/library/ast.html#ast.For
#[derive(Debug, Clone)]
pub struct For {
    pub node: Node,
    pub target: Box<Expression>,
    pub iter: Box<Expression>,
    pub body: Vec<Statement>,
    pub orelse: Vec<Statement>,
}

// https://docs.python.org/3/library/ast.html#ast.With
#[derive(Debug, Clone)]
pub struct With {
    pub node: Node,
    pub items: Vec<WithItem>,
    pub body: Vec<Statement>,
}

// https://docs.python.org/3/library/ast.html#ast.withitem
// can be used for With
#[derive(Debug, Clone)]
pub struct WithItem {
    pub node: Node,
    pub context_expr: Box<Expression>,
    pub optional_vars: Option<Box<Expression>>,
}

// https://docs.python.org/3/library/ast.html#ast.Try
#[derive(Debug, Clone)]
pub struct Try {
    pub node: Node,
    pub body: Vec<Statement>,
    pub handlers: Vec<ExceptHandler>,
    pub orelse: Vec<Statement>,
    pub finalbody: Vec<Statement>,
}

// https://docs.python.org/3/library/ast.html#ast.TryStar
#[derive(Debug, Clone)]
pub struct TryStar {
    pub node: Node,
    pub body: Vec<Statement>,
    pub handlers: Vec<ExceptHandler>,
    pub orelse: Vec<Statement>,
    pub finalbody: Vec<Statement>,
}

// https://docs.python.org/3/library/ast.html#ast.ExceptHandler
#[derive(Debug, Clone)]
pub struct ExceptHandler {
    pub node: Node,
    pub typ: Option<Box<Expression>>,
    pub name: Option<String>,
    pub body: Vec<Statement>,
}

// https://docs.python.org/3/library/ast.html#functiondef
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub node: Node,
    pub name: String,
    pub args: Arguments,
    pub body: Vec<Statement>,
    pub decorator_list: Vec<Expression>,
    pub returns: Option<Box<Expression>>,
    pub type_comment: Option<String>,
}

// https://docs.python.org/3/library/ast.html#ast.ClassDef
#[derive(Debug, Clone)]
pub struct ClassDef {
    pub node: Node,
    pub name: String,
    pub bases: Vec<Expression>,
    pub keywords: Vec<Keyword>,
    pub body: Vec<Statement>,
    pub decorator_list: Vec<Expression>,
}

// https://docs.python.org/3/library/ast.html#ast.Match
#[derive(Debug, Clone)]
pub struct Match {
    pub node: Node,
    pub subject: Box<Expression>,
    pub cases: Vec<MatchCase>,
}

// https://docs.python.org/3/library/ast.html#ast.match_case
#[derive(Debug, Clone)]
pub struct MatchCase {
    pub node: Node,
    pub pattern: Box<MatchPattern>,
    pub guard: Option<Box<Expression>>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum MatchPattern {
    MatchValue(MatchValue),
    MatchSingleton(Box<Expression>),
    MatchSequence(Vec<MatchPattern>),
    MatchStar(Box<Expression>),
    MatchMapping(MatchMapping),
    MatchAs(MatchAs),
    MatchClass(MatchClass),
    MatchOr(Vec<MatchPattern>),
}

#[derive(Debug, Clone)]
pub struct MatchValue {
    pub node: Node,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct MatchAs {
    pub node: Node,
    pub name: Option<String>,
    pub pattern: Option<Box<MatchPattern>>,
}

#[derive(Debug, Clone)]
pub struct MatchMapping {
    pub node: Node,
    pub keys: Vec<Expression>,
    pub patterns: Vec<MatchPattern>,
    pub rest: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MatchClass {
    pub node: Node,
    pub cls: Box<Expression>,
    pub patterns: Vec<MatchPattern>,
    pub kwd_attrs: Vec<String>,
    pub kwd_patterns: Vec<MatchPattern>,
}