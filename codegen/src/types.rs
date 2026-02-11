/// Intermediate representation for code generation.

#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub kind: SchemaKind,
}

#[derive(Debug, Clone)]
pub enum SchemaKind {
    Struct(StructDef),
    Enum(EnumDef),
    TypeAlias(String),
    StringNewtype,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub rust_name: String,
    pub ty: FieldType,
    pub required: bool,
    pub serde_rename: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    String,
    F64,
    I64,
    U64,
    Bool,
    Vec(Box<FieldType>),
    HashMap(Box<FieldType>),
    Option(Box<FieldType>),
    Ref(String),
}

impl FieldType {
    pub fn to_rust_type(&self) -> String {
        match self {
            FieldType::String => "String".to_string(),
            FieldType::F64 => "f64".to_string(),
            FieldType::I64 => "i64".to_string(),
            FieldType::U64 => "u64".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Vec(inner) => format!("Vec<{}>", inner.to_rust_type()),
            FieldType::HashMap(inner) => {
                format!("std::collections::HashMap<String, {}>", inner.to_rust_type())
            }
            FieldType::Option(inner) => format!("Option<{}>", inner.to_rust_type()),
            FieldType::Ref(name) => name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub variants: Vec<EnumVariant>,
    pub rename_all: Option<String>,
    pub is_untagged: bool,
    pub is_complex: bool,
    pub complex_variants: Vec<ComplexVariant>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub original: String,
}

#[derive(Debug, Clone)]
pub struct ComplexVariant {
    pub name: String,
    pub ty: FieldType,
}
