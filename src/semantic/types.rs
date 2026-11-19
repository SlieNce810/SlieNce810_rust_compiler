#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Array(ArrayType),
    Function(FunctionType),
    // 额外增加：避免错误传播导致大量误报
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayType {
    pub element: Box<Type>,
    pub length: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub return_type: Box<Type>,
    pub params: Vec<Type>,
}