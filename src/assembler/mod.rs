pub mod ast;

use crate::{
    assembler::ast::{File, Value},
    utils::result::{AppError, AppResult},
};

use self::ast::{Meta, Metas};

#[derive(PartialEq, Debug)]
struct TTLResource<T> {
    identifier: String,
    value: T,
    metas_literal: Option<Vec<String>>,
    metas_ref: Option<Vec<String>>,
}

impl<T> TTLResource<T> {
    fn new(
        value: T,
        identifier: String,
        metas_literal: Option<Vec<String>>,
        metas_ref: Option<Vec<String>>,
    ) -> Self {
        Self {
            identifier,
            value,
            metas_literal,
            metas_ref,
        }
    }
}

#[derive(PartialEq, Debug)]
enum TTLResources {
    String(TTLResource<String>),
    Number(TTLResource<f64>),
}

fn assemble_file(file_str: &str) -> AppResult<Vec<TTLResources>> {
    let file = File::try_from(file_str)?;
    let value = file.value;

    fn ast_to_value(val: Value, path: Option<String>, metas: Option<Metas>) -> Vec<TTLResources> {
        let (meta_lit, meta_ref): (Option<Vec<String>>, Option<Vec<String>>) =
            metas.map(|m| m.into()).unwrap_or_else(|| (None, None));

        /* let meta_ref = match path {
            Some(ref p) => meta_ref.and_then(|v| {
                let new_meta = v
                    .iter()
                    .map(|m| {
                        let current_path = p.clone();
                        let mut segments: Vec<&str> = current_path.split(".").collect();
                        if let Some(last_segment) = segments.last_mut() {
                            *last_segment = m.as_str();
                        }
                        let new_meta = segments.join(".");
                        new_meta
                    })
                    .collect();
                Some(new_meta)
            }),
            None => meta_ref,
        }; */

        match val {
            Value::String(s) => vec![TTLResources::String(TTLResource::new(
                s.0,
                path.unwrap_or_else(|| "".to_string()),
                meta_lit,
                meta_ref,
            ))],
            Value::Number(n) => vec![TTLResources::Number(TTLResource::new(
                n.0,
                path.unwrap_or_else(|| "".to_string()),
                meta_lit,
                meta_ref,
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
                    res.extend(ast_to_value(decl.value, new_path, decl.metas));
                }
                res
            }
        }
    }

    let res = ast_to_value(value, None, None);

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
                ["abc" ref01 123]
                var04: 745
                var05: "hello"
                var06: {
                    [ref02]
                    var07: 07
                    var08: 08
                }
            }"#,
        )
        .unwrap();

        assert_eq!(values.len(), 4);

        let first_ressource = values.get(0).unwrap();
        let second_ressource = values.get(1).unwrap();
        let third_ressource = values.get(2).unwrap();
        let fourth_ressource = values.get(3).unwrap();

        assert_eq!(
            first_ressource,
            &TTLResources::Number(TTLResource::new(
                745.0,
                "var04".to_string(),
                Some(vec!["abc".to_string(), "123".to_string()]),
                Some(vec!["ref01".to_string()]),
            ))
        );

        assert_eq!(
            second_ressource,
            &TTLResources::String(TTLResource::new(
                "hello".to_string(),
                "var05".to_string(),
                None,
                None,
            ))
        );

        assert_eq!(
            third_ressource,
            &TTLResources::Number(TTLResource::new(
                7.0,
                "var06.var07".to_string(),
                None,
                Some(vec!["ref02".to_string()]),
            ))
        );

        assert_eq!(
            fourth_ressource,
            &TTLResources::Number(TTLResource::new(8.0, "var06.var08".to_string(), None, None))
        );
    }
}
