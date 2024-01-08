use super::json_formatter_helper::{self as helper, JSONValue};
use crate::{domain::resolution::ResolvedResource, ports::FormatterPort, result::AppResult};

pub struct JsonFormatter {}
impl JsonFormatter {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}
impl FormatterPort for JsonFormatter {
    type Format = JSONValue;

    fn format(&self, resources: Vec<ResolvedResource>) -> AppResult<Self::Format> {
        let json = resources
            .into_iter()
            .try_fold(JSONValue::Null, |mut json_acc, resource| {
                let key_types = helper::get_keytypes(resource.identifier.as_deref());
                let entry = helper::get_json_entry_for_keys(&mut json_acc, key_types)?;
                let json_value = helper::get_json_value(resource)?;
                *entry = json_value;
                AppResult::Ok(json_acc)
            })?;

        Ok(json)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::resolution::ResolvedResourceBuilder;
    use serde_json::json;

    #[test]
    fn it_should_format_to_json() {
        let resources = vec![
            ResolvedResourceBuilder::default()
                .identifier(Some("a.b".to_string()))
                .build_as_string("abc")
                .unwrap(),
            ResolvedResourceBuilder::default()
                .identifier(Some("a.c".to_string()))
                .build_as_number(22.0)
                .unwrap(),
        ];
        let formatter = JsonFormatter {};
        let result = formatter.format(resources).unwrap();

        assert_eq!(
            result,
            json!(
                {
                    "a": {
                        "b": {
                            "value": "abc",
                            "metas": []
                        },
                        "c": {
                            "value": 22.0,
                            "metas": []
                        }
                    }
                }
            )
        );
    }
}