use super::{parser::Rule, values::Value};
use crate::{
    ast::parser::TTLParser,
    utils::result::{AppError, AppResult},
};
use pest::Parser;
use pest_ast::FromPest;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::file))]
pub struct File {
    pub value: Value,
    _eoi: EOI,
}
impl TryFrom<&str> for File {
    type Error = AppError;

    fn try_from(s: &str) -> AppResult<Self> {
        use from_pest::FromPest;
        let mut pairs =
            TTLParser::parse(Rule::file, s).map_err(|e| AppError::String(e.to_string()))?;
        let file = File::from_pest(&mut pairs).map_err(|e| AppError::String(e.to_string()))?;
        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{objects::ObjectElem, values::Value};
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_file() {
        let str = r#"
        {
            var02: 745
            var03: "hello"
        }
        "#;

        let mut pairs = super::TTLParser::parse(super::Rule::file, str).unwrap();
        let file = super::File::from_pest(&mut pairs).unwrap();
        let value = file.value;

        let object_elems = match value {
            Value::Object(s) => s.0,
            _ => panic!("Unexpected value"),
        };

        let first_declaration = match object_elems.get(0).unwrap() {
            ObjectElem::Declaration(d) => d,
            _ => panic!("Shoudl be declaration"),
        };
        let first_var = &first_declaration.identifier;
        let first_value = &first_declaration.value;
        let first_value = match first_value {
            Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        let second_declaration = match object_elems.get(1).unwrap() {
            ObjectElem::Declaration(d) => d,
            _ => panic!("Shoudl be declaration"),
        };
        let second_var = &second_declaration.identifier;
        let second_value = &second_declaration.value;
        let second_value = match second_value {
            super::Value::String(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        assert_eq!(first_var.0, "var02");
        assert_eq!(first_value, 745.0);
        assert_eq!(second_var.0, "var03");
        assert_eq!(second_value, "hello");
    }
}
