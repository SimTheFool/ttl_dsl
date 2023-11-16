pub mod parser;

use crate::{
    assembler::parser::ast::{self, Value},
    utils::result::{AppError, AppResult},
};
use from_pest::FromPest;
use pest::Parser;

#[derive(PartialEq, Debug)]
struct TTLResource<T> {
    identifier: String,
    value: T,
    metas: Vec<String>,
}

impl<T> TTLResource<T> {
    fn new(value: T, identifier: String) -> Self {
        Self {
            identifier,
            value,
            metas: Vec::new(),
        }
    }

    fn add_meta(&mut self, meta: String) {
        self.metas.push(meta);
    }
}

#[derive(PartialEq, Debug)]
enum TTLResources {
    String(TTLResource<String>),
    Number(TTLResource<f64>),
    Boolean(TTLResource<bool>),
}

fn assemble_file(file_str: &str) -> AppResult<Vec<TTLResources>> {
    let mut pairs = parser::TTLParser::parse(parser::Rule::file, file_str)
        .map_err(|e| AppError::String(e.to_string()))?;

    let file = ast::File::from_pest(&mut pairs).map_err(|e| AppError::String(e.to_string()))?;
    let value = file.value;

    fn ast_to_value(val: ast::Value, path: Option<String>) -> Vec<TTLResources> {
        match val {
            Value::String(s) => vec![TTLResources::String(TTLResource::new(
                s.0,
                path.unwrap_or_else(|| "".to_string()),
            ))],
            Value::Number(n) => vec![TTLResources::Number(TTLResource::new(
                n.0,
                path.unwrap_or_else(|| "".to_string()),
            ))],
            Value::Object(o) => {
                let mut res = Vec::new();
                for decl in o.0 {
                    let new_path = match path {
                        Some(ref p) => {
                            let mut new_path = p.to_string();
                            new_path.push_str(".");
                            new_path.push_str(&decl.0 .0);
                            Some(new_path)
                        }
                        None => Some(decl.0 .0.to_string()),
                    };

                    res.extend(ast_to_value(decl.1, new_path));
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
                var04: 745
                var05: "hello"
                var06: {
                    var07: 07
                    var08: 08
                }
            }"#,
        )
        .unwrap();

        let expected_values = vec![
            TTLResources::Number(TTLResource::new(745.0, "var04".to_string())),
            TTLResources::String(TTLResource::new("hello".to_string(), "var05".to_string())),
            TTLResources::Number(TTLResource::new(7.0, "var06.var07".to_string())),
            TTLResources::Number(TTLResource::new(8.0, "var06.var08".to_string())),
        ];

        assert_eq!(values, expected_values);
    }
}
