pub mod parser;

use crate::{
    ports::TTLInputPort,
    utils::result::{AppError, AppResult},
};
use pest::Parser;

/* fn assemble(input: &str, ttl_input_reader: impl TTLInputPort) -> AppResult<serde_json::Value> {
    let ast = parser::TTLParser::parse(parser::Rule::object, input)
        .map_err(|e| AppError::String(e.to_string()))?;

    let mut json = serde_json::json!({});

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::assemble;
    use crate::infras::file_reader::TTLMockedInputAdapter;

    #[test]
    fn it_should_assemble_input() {
        let input_adapter = TTLMockedInputAdapter::new();

        let json_value: serde_json::Value = assemble(
            r#"{
                var04: 745
                var05: "hello"
            }"#,
            input_adapter,
        )
        .unwrap();

        let expected_json_value = serde_json::json!({
            "var04": 745,
            "var05": "hello"
        });

        assert_eq!(json_value, expected_json_value);
    }
}
 */
