pub mod ast;

use crate::{
    assembler::ast::{File, Value},
    utils::result::AppResult,
};

#[derive(PartialEq, Debug)]
struct TTLResource<T> {
    identifier: String,
    value: T,
    metas: Vec<String>,
}

impl<T> TTLResource<T> {
    fn new(value: T, identifier: String, metas: Option<Vec<String>>) -> Self {
        Self {
            identifier,
            value,
            metas: metas.unwrap_or_else(|| Vec::new()),
        }
    }
}

#[derive(PartialEq, Debug)]
enum TTLResources {
    String(TTLResource<String>),
    Number(TTLResource<f64>),
    Boolean(TTLResource<bool>),
}

fn assemble_file(file_str: &str) -> AppResult<Vec<TTLResources>> {
    let file = File::try_from(file_str)?;
    let value = file.value;

    fn ast_to_value(val: Value, path: Option<String>) -> Vec<TTLResources> {
        match val {
            Value::String(s, metas) => vec![TTLResources::String(TTLResource::new(
                s.0,
                path.unwrap_or_else(|| "".to_string()),
                metas.map(|m| m.into()),
            ))],
            Value::Number(n, metas) => vec![TTLResources::Number(TTLResource::new(
                n.0,
                path.unwrap_or_else(|| "".to_string()),
                metas.map(|m| m.into()),
            ))],
            Value::Object(o) => {
                let mut res = Vec::new();
                for decl in o.0 {
                    let new_path = match path {
                        Some(ref p) => {
                            let mut new_path = p.to_string();
                            new_path.push_str(".");
                            new_path.push_str(&decl.identifier.0);
                            Some(new_path)
                        }
                        None => Some(decl.identifier.0.to_string()),
                    };
                    res.extend(ast_to_value(decl.value, new_path));
                }
                res
            }
        }
    }

    let res = ast_to_value(value, None);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::assemble_file;
    use crate::assembler::{TTLResource, TTLResources};

    #[test]
    fn it_should_create_resource() {
        let values = assemble_file(
            r#"{
                var04: 745 ["abc" "def" 123]
                var05: "hello"
                var06: {
                    var07: 07
                    var08: 08
                }
            }"#,
        )
        .unwrap();

        let expected_values = vec![
            TTLResources::Number(TTLResource::new(
                745.0,
                "var04".to_string(),
                Some(vec![
                    "abc".to_string(),
                    "def".to_string(),
                    "123".to_string(),
                ]),
            )),
            TTLResources::String(TTLResource::new(
                "hello".to_string(),
                "var05".to_string(),
                None,
            )),
            TTLResources::Number(TTLResource::new(7.0, "var06.var07".to_string(), None)),
            TTLResources::Number(TTLResource::new(8.0, "var06.var08".to_string(), None)),
        ];

        assert_eq!(values, expected_values);
    }
}
