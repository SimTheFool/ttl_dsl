use crate::{
    domain::resolution::{ResolvedResource, ResolvedResourceBuilder, ResolvedResourceValue},
    result::AppResult,
};
use evalexpr::{
    Context, ContextWithMutableVariables, EmptyContextWithBuiltinFunctions, EvalexprResult, Value,
};
use indexmap::IndexMap;

pub struct TransformableList {
    value: IndexMap<String, Value>,
    origin: IndexMap<String, ResolvedResource>,
}

impl TransformableList {
    fn set(&mut self, key: String, value: Value) -> AppResult<()> {
        let old_resource = self.origin.get(&key);
        let new_resource = match (old_resource, &value) {
            (None, Value::String(s)) => ResolvedResourceBuilder::default().build_as_string(s)?,
            (None, Value::Float(l)) => ResolvedResourceBuilder::default().build_as_number(*l)?,
            (Some(rs), Value::String(s)) => ResolvedResource {
                value: ResolvedResourceValue::String(s.clone()),
                ..rs.clone()
            },
            (Some(rs), Value::Float(n)) => ResolvedResource {
                value: ResolvedResourceValue::Number(*n),
                ..rs.clone()
            },
            (Some(rs), Value::Int(n)) => ResolvedResource {
                value: ResolvedResourceValue::Number(*n as f64),
                ..rs.clone()
            },
            (_, x) => panic!("Unhandled evaluated type {:#?}", x),
        };

        self.value.insert(key.clone(), value);
        self.origin.insert(key, new_resource);
        Ok(())
    }
}

impl From<IndexMap<String, ResolvedResource>> for TransformableList {
    fn from(resource_map: IndexMap<String, ResolvedResource>) -> Self {
        let mut index_map = IndexMap::<String, Value>::new();

        for (key, resource) in &resource_map {
            match &resource.value {
                ResolvedResourceValue::String(s) => {
                    index_map.insert(key.to_string(), Value::String(s.clone()));
                }
                ResolvedResourceValue::Number(l) => {
                    index_map.insert(key.to_string(), Value::Float(*l));
                }
                ResolvedResourceValue::Null => {
                    index_map.insert(key.to_string(), Value::Empty);
                }
            }
        }

        TransformableList {
            value: index_map,
            origin: resource_map,
        }
    }
}

impl From<TransformableList> for Vec<ResolvedResource> {
    fn from(transformable_list: TransformableList) -> Self {
        transformable_list
            .origin
            .into_iter()
            .map(|(_, v)| v)
            .collect()
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
        self.set(_identifier, _value)
            .map_err(|e| evalexpr::EvalexprError::CustomMessage(e.to_string()))?;
        Ok(())
    }
}
