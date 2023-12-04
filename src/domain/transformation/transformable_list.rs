use crate::domain::resource::{state::Resolved, ResolvedResources, Resource};
use evalexpr::{
    Context, ContextWithMutableVariables, EmptyContextWithBuiltinFunctions, EvalexprResult, Value,
};
use indexmap::IndexMap;

pub struct TransformableList {
    value: IndexMap<String, Value>,
    origin: IndexMap<String, ResolvedResources>,
}

impl TransformableList {
    fn set(&mut self, key: String, value: Value) {
        let old_resource = self.origin.get(&key);
        let new_resource = match (old_resource, &value) {
            (None, Value::String(s)) => ResolvedResources::String(
                Resource::<String, Resolved>::default_with_value(s.to_string()),
            ),
            (None, Value::Float(l)) => {
                ResolvedResources::Number(Resource::<f64, Resolved>::default_with_value(l.clone()))
            }
            (Some(ResolvedResources::String(rs)), Value::String(s)) => {
                ResolvedResources::String(rs.from_with_value(s.to_string()))
            }
            (Some(ResolvedResources::Number(rs)), Value::Float(l)) => {
                ResolvedResources::Number(rs.from_with_value(l.clone()))
            }
            (Some(ResolvedResources::Number(rs)), Value::String(s)) => {
                ResolvedResources::String(rs.from_with_value(s.to_string()))
            }
            (Some(ResolvedResources::String(rs)), Value::Float(l)) => {
                ResolvedResources::Number(rs.from_with_value(l.clone()))
            }
            _ => panic!("Invalid type"),
        };

        self.value.insert(key.clone(), value);
        self.origin.insert(key, new_resource);
    }
}

impl From<IndexMap<String, ResolvedResources>> for TransformableList {
    fn from(value: IndexMap<String, ResolvedResources>) -> Self {
        let mut index_map = IndexMap::<String, Value>::new();

        for (key, value) in &value {
            match value {
                ResolvedResources::String(s) => {
                    index_map.insert(key.to_string(), Value::String(s.value.clone()));
                }
                ResolvedResources::Number(l) => {
                    index_map.insert(key.to_string(), Value::Float(l.value.clone()));
                }
            }
        }

        TransformableList {
            value: index_map,
            origin: value,
        }
    }
}

impl Into<IndexMap<String, ResolvedResources>> for TransformableList {
    fn into(self) -> IndexMap<String, ResolvedResources> {
        self.origin
    }
}

impl Context for TransformableList {
    fn are_builtin_functions_disabled(&self) -> bool {
        false
    }

    fn set_builtin_functions_disabled(&mut self, _: bool) -> EvalexprResult<()> {
        Ok(())
    }

    fn call_function(
        &self,
        _identifier: &str,
        _arg: &evalexpr::Value,
    ) -> EvalexprResult<evalexpr::Value> {
        let ctx = EmptyContextWithBuiltinFunctions {};
        ctx.call_function(_identifier, _arg)
    }

    fn get_value(&self, identifier: &str) -> Option<&Value> {
        self.value.get(identifier)
    }
}

impl ContextWithMutableVariables for TransformableList {
    fn set_value(&mut self, _identifier: String, _value: Value) -> EvalexprResult<()> {
        self.set(_identifier, _value);
        Ok(())
    }
}
