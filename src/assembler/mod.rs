mod ast;
pub mod resources;

use self::resources::{Literal, Referenced, Resource, Resources};
use crate::{assembler::resources::ResourceContext, ports::TTLInputPort, utils::result::AppResult};
use indexmap::IndexMap;

fn assemble_file(
    file_str: &str,
    input_port: impl TTLInputPort,
) -> AppResult<Vec<Resources<Literal>>> {
    let file = ast::File::try_from(file_str)?;
    let value = file.value;

    fn ast_to_value(
        val: ast::Value,
        identifier: Option<String>,
        ctx: ResourceContext,
    ) -> IndexMap<String, Resources<Referenced>> {
        let mut resource_map = IndexMap::<String, Resources<Referenced>>::new();

        match val {
            ast::Value::String(s) => {
                let resource =
                    Resources::String(Resource::new(s.0, identifier.clone(), ctx.clone()));

                let resource_path = match (ctx.path, identifier) {
                    (None, None) => "".to_string(),
                    (None, Some(id)) => id,
                    (Some(base), None) => base,
                    (Some(base), Some(id)) => format!("{}.{}", base, id),
                };

                resource_map.insert(resource_path, resource);
            }
            ast::Value::Number(n) => {
                let resource =
                    Resources::Number(Resource::new(n.0, identifier.clone(), ctx.clone()));

                let resource_path = match (ctx.path, identifier) {
                    (None, None) => "".to_string(),
                    (None, Some(id)) => id,
                    (Some(base), None) => base,
                    (Some(base), Some(id)) => format!("{}.{}", base, id),
                };

                resource_map.insert(resource_path, resource);
            }
            ast::Value::Object(o) => {
                for elem in o.0 {
                    match elem {
                        ast::ObjectElem::Declaration(v) => {
                            let resource_path = match (&ctx.path, &v.identifier) {
                                (None, id) => id.0.clone(),
                                (Some(base), id) => format!("{}.{}", base, id.0.clone()),
                            };

                            let sub_resource_map = ast_to_value(
                                v.value,
                                Some(v.identifier.0),
                                ResourceContext {
                                    variables: ctx.variables.clone(),
                                    path: Some(resource_path),
                                },
                            );

                            resource_map.extend(sub_resource_map);
                        }
                        _ => {}
                    };
                }
            }
        };

        resource_map
    }

    let resource_map = ast_to_value(
        value,
        None,
        ResourceContext {
            variables: None,
            path: None,
        },
    );
    let resource_map: IndexMap<String, Resources<Literal>> = resource_map
        .into_iter()
        .map(|(k, v)| {
            let new_kv = (k.clone(), v.try_compute_references()?);
            return AppResult::Ok(new_kv);
        })
        .try_fold(
            IndexMap::<String, Resources<Literal>>::new(),
            |mut map, kv| {
                let (k, v) = kv?;
                map.insert(k, v);
                AppResult::Ok(map)
            },
        )?;

    Ok(resource_map.into_iter().map(|(_, v)| v).collect())
}

#[cfg(test)]
mod tests {
    use super::assemble_file;
    use crate::{
        assembler::resources::{Literal, Resources},
        infras::file_reader::TTLMockedInputAdapter,
    };

    #[test]
    fn it_should_create_resources() {
        let mocked_input = TTLMockedInputAdapter::new();
        let values = assemble_file(
            r#"{
                var05: "hello"
                var06: {
                    var07: 07
                    var08: 08
                }
            }"#,
            mocked_input,
        )
        .unwrap();

        assert_eq!(values.len(), 3);

        let first_ressource = values.get(0).unwrap();
        let second_ressource = values.get(1).unwrap();
        let third_ressource = values.get(2).unwrap();

        match first_ressource {
            Resources::<Literal>::String(x) => {
                assert_eq!(x.identifier, Some("var05".to_string()));
                assert_eq!(x.value, "hello");
            }
            _ => panic!("Should be a string"),
        }

        match second_ressource {
            Resources::<Literal>::Number(x) => {
                assert_eq!(x.identifier, Some("var07".to_string()));
                assert_eq!(x.value, 7.0);
            }
            _ => panic!("Should be a number"),
        }

        match third_ressource {
            Resources::<Literal>::Number(x) => {
                assert_eq!(x.identifier, Some("var08".to_string()));
                assert_eq!(x.value, 8.0);
            }
            _ => panic!("Should be a number"),
        }
    }

    /* #[test]
    fn it_should_create_resources_with_integration() {
        let mut mocked_input = TTLMockedInputAdapter::new();
        mocked_input.mock_file(
            "./stats",
            r#"{
                somevar01: var01,
                somevar02: var02,
                someothervar: "statistics"
            }"#,
        );

        let values = assemble_file(
            r#"{
                << ./stats
                    with var01 : 001
                    with var02 : "002"
                var03: 003
            }"#,
            mocked_input,
        )
        .unwrap();

        assert_eq!(values.len(), 4);

        let first_ressource = values.get(0).unwrap();
        let second_ressource = values.get(1).unwrap();
        let third_ressource = values.get(2).unwrap();
        let fourth_ressource = values.get(3).unwrap();

        match first_ressource {
            Resources::<Literal>::Number(x) => {
                assert_eq!(x.identifier, "somevar01");
                assert_eq!(x.value, 1.0);
            }
            _ => panic!("Should be a number"),
        }

        match second_ressource {
            Resources::<Literal>::String(x) => {
                assert_eq!(x.identifier, "someothervar");
                assert_eq!(x.value, "002");
            }
            _ => panic!("Should be a string"),
        }

        match third_ressource {
            Resources::<Literal>::String(x) => {
                assert_eq!(x.identifier, "somevar02");
                assert_eq!(x.value, "statistics");
            }
            _ => panic!("Should be a string"),
        }

        match fourth_ressource {
            Resources::<Literal>::Number(x) => {
                assert_eq!(x.identifier, "var03");
                assert_eq!(x.value, 3.0);
            }
            _ => panic!("Should be a number"),
        }
    } */
}
