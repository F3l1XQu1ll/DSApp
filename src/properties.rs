use std::{borrow::Cow, collections::BTreeMap};

pub mod character;
pub mod strings;

pub type StoreKey = Cow<'static, str>;

#[derive(PartialEq, PartialOrd, Clone, serde::Deserialize, serde::Serialize)]
pub enum Value {
    Str(Cow<'static, str>),
    Val(f64),
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Val(value)
    }
}

impl From<Cow<'static, str>> for Value {
    fn from(value: Cow<'static, str>) -> Self {
        Self::Str(value)
    }
}

impl From<&'static str> for Value {
    fn from(value: &'static str) -> Self {
        Value::Str(Cow::Borrowed(value))
    }
}

impl Value {
    pub fn to_str(self) -> Option<Cow<'static, str>> {
        if let Value::Str(string) = self {
            Some(string)
        } else {
            None
        }
    }

    pub fn to_val(self) -> Option<f64> {
        if let Value::Val(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn str(self) -> Cow<'static, str> {
        self.to_str().unwrap()
    }

    pub fn val(self) -> f64 {
        self.to_val().unwrap()
    }

    pub fn to_str_mut(&mut self) -> Option<&mut String> {
        if let Value::Str(string) = self {
            Some(string.to_mut())
        } else {
            None
        }
    }

    pub fn to_val_mut(&mut self) -> Option<&mut f64> {
        if let Value::Val(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn str_mut(&mut self) -> &mut String {
        self.to_str_mut().unwrap()
    }

    pub fn val_mut(&mut self) -> &mut f64 {
        self.to_val_mut().unwrap()
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum Check {
    Equal(Value, Value),
    Greater(Value, Value),
    Smaller(Value, Value),
}

impl Check {
    pub fn eval(&self) -> bool {
        match self {
            Check::Equal(p, q) => p == q,
            Check::Greater(p, q) => p > q,
            Check::Smaller(p, q) => p < q,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum Operation {
    Add(Box<Operation>, Box<Operation>),
    Sub(Box<Operation>, Box<Operation>),
    Mul(Box<Operation>, Box<Operation>),
    Div(Box<Operation>, Box<Operation>),
    // Check(Check, Box<Operation>, Option<Box<Operation>>),
    Gt(
        Box<Operation>,
        Box<Operation>,
        Box<Operation>,
        Option<Box<Operation>>,
    ),
    Set(Cow<'static, str>, Box<Operation>),
    Get(Cow<'static, str>),
    Load(Value), // use a value in an operation
}

#[macro_export]
macro_rules! ld {
    ($str: expr) => {
        Operation::Load($str.into())
    };
}

#[macro_export]
macro_rules! get {
    ($str: expr) => {
        Operation::Get($str.into())
    };
}

#[macro_export]
macro_rules! add {
    ($p: expr, $q: expr) => {
        Operation::Add(Box::new($p), Box::new($q))
    };
}

#[macro_export]
macro_rules! sub {
    ($p: expr, $q: expr) => {
        Operation::Sub(Box::new($p), Box::new($q))
    };
}

#[macro_export]
macro_rules! mul {
    ($p: expr, $q: expr) => {
        Operation::Mul(Box::new($p), Box::new($q))
    };
}

#[macro_export]
macro_rules! div {
    ($p: expr, $q: expr) => {
        Operation::Div(Box::new($p), Box::new($q))
    };
}

#[macro_export]
macro_rules! gt {
    ($p: expr, $q: expr, $r: expr, $s:expr) => {
        Operation::Gt(Box::new($p), Box::new($q), Box::new($r), Some(Box::new($s)))
    };
}

macro_rules! op_eval {
    ($p: expr, $q: expr, $store: expr, $op: tt) => {
        if let (Value::Val(p), Value::Val(q)) = ($p.eval($store).unwrap(), $q.eval($store).unwrap())
        {
            Some((p $op q).into())
        } else {
            None
        }
    };
}

impl Operation {
    pub fn eval(&self, store: &mut BTreeMap<StoreKey, Value>) -> Option<Value> {
        match self {
            Operation::Add(p, q) => {
                op_eval!(p, q, store, +)
            }
            Operation::Sub(p, q) => {
                op_eval!(p, q, store, -)
            }
            Operation::Mul(p, q) => {
                op_eval!(p, q, store, *)
            }
            Operation::Div(p, q) => {
                op_eval!(p, q, store, /)
            }
            Operation::Gt(p, q, r, s) => {
                if p.eval(store).unwrap() > q.eval(store).unwrap() {
                    r.eval(store)
                } else if let Some(s) = s {
                    s.eval(store)
                } else {
                    None
                }
            }
            Operation::Set(s, o) => {
                if let Some(value) = o.eval(store) {
                    store.insert(s.to_owned(), value);
                }
                None
            }
            Operation::Get(s) => store.get(s).cloned(),
            Operation::Load(v) => Some(v.clone()),
        }
        .into()
    }
}

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct PropertiesManager {
    pub properties: BTreeMap<StoreKey, Value>,
    pub operations: BTreeMap<StoreKey, Operation>,
}

impl PropertiesManager {
    pub fn add_prop(&mut self, key: impl Into<StoreKey>, prop: impl Into<Value>) {
        self.properties.insert(key.into(), prop.into());
    }

    pub fn add_op(&mut self, key: StoreKey, op: Operation) {
        self.operations.insert(key, op);
    }

    pub fn handle(&mut self, key: impl Into<StoreKey>, default: impl Into<Value>) -> &mut Value {
        self.properties.entry(key.into()).or_insert(default.into())
    }

    pub fn children_mut(
        &mut self,
        key: impl Into<StoreKey>,
    ) -> impl Iterator<Item = (&StoreKey, &mut Value)> {
        let key: StoreKey = key.into();
        self.properties
            .range_mut(key.clone()..)
            .take_while(move |(k, _)| k.starts_with(key.as_ref()))
    }
}
