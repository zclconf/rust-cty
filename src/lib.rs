//! A dynamic type system with runtime safety checks.

use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Bool,
    Number,
    List(Box<Type>),
    Map(Box<Type>),
    Set(Box<Type>),
    Tuple(Vec<Type>),
    Object(HashMap<String, Type>),
    Any,
}

type Mapping = HashMap<String, ValueInner>;
type Sequence = Vec<ValueInner>;

#[derive(Debug, Clone)]
enum ValueInner {
    String(std::string::String),
    Bool(bool),
    Sequence(Sequence),
    Mapping(Mapping),
    Unknown,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    ty: Type,
    v: ValueInner,
}

impl Value {
    pub fn null(ty: Type) -> Value {
        Value {
            ty: ty,
            v: ValueInner::Null,
        }
    }

    pub fn unknown(ty: Type) -> Value {
        Value {
            ty: ty,
            v: ValueInner::Unknown,
        }
    }

    pub fn dynamic() -> Value {
        Value {
            ty: Type::Any,
            v: ValueInner::Unknown,
        }
    }

    pub fn string(v: &str) -> Value {
        // cty strings are always NFC-normalized.
        let s = v.nfc().collect::<String>();
        Value {
            ty: Type::String,
            v: ValueInner::String(s),
        }
    }

    pub fn bool(v: bool) -> Value {
        Value {
            ty: Type::Bool,
            v: ValueInner::Bool(v),
        }
    }

    pub fn list(ty: Type, vals: &[Value]) -> Value {
        let mut v: Sequence = Vec::with_capacity(vals.len());
        for ev in vals {
            if ev.type_() != &ty {
                panic!(
                    "element type {:?} does not match list type {:?}",
                    *ev.type_(),
                    ty
                )
            }
            v.push(ev.v.clone())
        }
        Value::list_raw(ty, v)
    }

    fn list_raw(ty: Type, vals: Sequence) -> Value {
        Value {
            ty: ty,
            v: ValueInner::Sequence(vals),
        }
    }

    pub fn map(ty: Type, vals: HashMap<String, Value>) -> Value {
        let mut v: Mapping = HashMap::with_capacity(vals.len());
        for (ek, ev) in vals {
            if ev.type_() != &ty {
                panic!(
                    "element type {:?} does not match list type {:?}",
                    *ev.type_(),
                    ty
                )
            }
            v.insert(ek, ev.v.clone());
        }
        Value::map_raw(ty, v)
    }

    fn map_raw(ty: Type, vals: Mapping) -> Value {
        Value {
            ty: ty,
            v: ValueInner::Mapping(vals),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.ty
    }
}

impl PartialEq for ValueInner {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (ValueInner::Unknown, _) => false,
            (_, ValueInner::Unknown) => false,
            (ValueInner::Null, ValueInner::Null) => true,
            (ValueInner::Bool(a), ValueInner::Bool(b)) => a == b,
            (ValueInner::String(ref a), ValueInner::String(ref b)) => a == b,
            (_, _) => false,
        }
    }
}

impl Eq for Type {}
impl Eq for ValueInner {}
impl Eq for Value {}
