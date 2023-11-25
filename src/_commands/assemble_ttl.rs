use crate::{
    ports::TTLInputPort,
    resource::{ResolvedResources, ResourceContext, ResourceMapper},
    utils::result::AppResult,
};
use indexmap::IndexMap;

use crate::ast;

fn assemble_file(
    file_str: &str,
    input_port: impl TTLInputPort,
) -> AppResult<Vec<ResolvedResources>> {
    let resource_mapper = ResourceMapper::new(&input_port);

    let value = ast::File::try_from(file_str)?.value;

    let resources_map = resource_mapper.ast_values_to_resource_map(
        value,
        None,
        ResourceContext {
            variables: None,
            path: None,
        },
    )?;

    let resources_map = resources_map
        .into_iter()
        .map(|(k, v)| {
            let new_kv = (k.clone(), v.try_compute_references()?);
            return AppResult::Ok(new_kv);
        })
        .try_fold(
            IndexMap::<String, ResolvedResources>::new(),
            |mut map, kv| {
                let (k, v) = kv?;
                map.insert(k, v);
                AppResult::Ok(map)
            },
        )?;

    Ok(resources_map.into_iter().map(|(_, v)| v).collect())
}

#[cfg(test)]
mod tests {
    use super::assemble_file;
    use crate::{ports::TTLMockedInputAdapter, resource::ResolvedResources};

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
            ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("var05".to_string()));
                assert_eq!(x.value, "hello");
            }
            _ => panic!("Should be a string"),
        }

        match second_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var07".to_string()));
                assert_eq!(x.value, 7.0);
            }
            _ => panic!("Should be a number"),
        }

        match third_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var08".to_string()));
                assert_eq!(x.value, 8.0);
            }
            _ => panic!("Should be a number"),
        }
    }

    #[test]
    fn it_should_create_resources_with_context() {
        let mocked_input = TTLMockedInputAdapter::new();
        let values = assemble_file(
            r#"{
                var05: "hello"
                var06: {
                    var07: 07
                }
            }"#,
            mocked_input,
        )
        .unwrap();

        assert_eq!(values.len(), 2);

        let first_ressource = values.get(0).unwrap();
        let second_ressource = values.get(1).unwrap();

        match first_ressource {
            ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("var05".to_string()));
                assert_eq!(x.value, "hello");
                assert_eq!(x.context.variables, None);
                assert_eq!(x.context.path, Some("var05".to_string()));
            }
            _ => panic!("Should be a string"),
        }

        match second_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var07".to_string()));
                assert_eq!(x.value, 7.0);
                assert_eq!(x.context.variables, None);
                assert_eq!(x.context.path, Some("var06.var07".to_string()));
            }
            _ => panic!("Should be a number"),
        }
    }

    #[test]
    fn it_should_create_resources_with_import() {
        let mut mocked_input = TTLMockedInputAdapter::new();
        mocked_input.mock_file(
            "./stats",
            r#"{
                somevar01: var01
                somevar02: var02
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
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("somevar01".to_string()));
                assert_eq!(x.value, 1.0);
            }
            _ => panic!("Should be a number"),
        }

        match second_ressource {
            ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("somevar02".to_string()));
                assert_eq!(x.value, "002");
            }
            _ => panic!("Should be a string"),
        }

        match third_ressource {
            ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("someothervar".to_string()));
                assert_eq!(x.value, "statistics");
            }
            _ => panic!("Should be a string"),
        }

        match fourth_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var03".to_string()));
                assert_eq!(x.value, 3.0);
            }
            _ => panic!("Should be a number"),
        }
    }
}
