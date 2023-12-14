use super::{parser::Rule, span_into_string, values::Value, Transform};
use crate::{
    domain::ast::parser::TTLParser,
    utils::result::{AppError, AppResult},
};
use pest::Parser;
use pest_ast::FromPest;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::name))]
pub struct Name(#[pest_ast(inner(with(span_into_string)))] pub String);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::file))]
pub struct File {
    pub name: Option<Name>,
    pub value: Value,
    pub transforms: Option<Vec<Transform>>,
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
    use crate::{
        as_variant,
        domain::ast::{objects::ObjectElem, values::Value},
    };
    use from_pest::FromPest;
    use pest::Parser;

    use super::File;

    #[test]
    fn it_should_parse_file() {
        let str = r#"
            {
                var02: 745
                var03: "hello"
            }

            @TRANSFORM FIRST_LAYER
            > x *= 2
            > x += 2
        "#;

        let mut pairs = super::TTLParser::parse(super::Rule::file, str).unwrap();
        let file = super::File::from_pest(&mut pairs).unwrap();
        let value = file.value;
        let transforms = file.transforms.unwrap();

        assert_eq!(transforms.len(), 1);
        let transform = transforms.get(0).unwrap();

        match &transform.rules {
            Some(rules) => {
                assert_eq!(rules.len(), 2);
                assert_eq!(rules.get(0).unwrap().0, "x *= 2");
                assert_eq!(rules.get(1).unwrap().0, "x += 2");
            }
            None => panic!("Should have rules"),
        }

        let object_elems = as_variant!(value, Value::Object).0;

        let first_declaration = as_variant!(object_elems.get(0).unwrap(), ObjectElem::Declaration);
        let first_var = &first_declaration.identifier;
        let first_value = as_variant!(&first_declaration.value, Value::Number);

        assert_eq!(first_var.0, "var02");
        assert_eq!(first_value.0, 745.0);

        let second_declaration = as_variant!(object_elems.get(1).unwrap(), ObjectElem::Declaration);
        let second_var = &second_declaration.identifier;
        let second_value = as_variant!(&second_declaration.value, Value::String);

        assert_eq!(second_var.0, "var03");
        assert_eq!(second_value.0, "hello");
    }

    #[test]
    fn it_should_parse_id_in_file() {
        let str = r#"
            @NAME Something_indeed

            {
                var02: 745
            }
        "#;

        let mut pairs = super::TTLParser::parse(super::Rule::file, str).unwrap();
        println!("{:#?}", pairs);
        let File { name, .. } = super::File::from_pest(&mut pairs).unwrap();

        match name {
            Some(super::Name(name)) => assert_eq!(name, "Something_indeed"),
            None => panic!("Should have an id"),
        }
    }
}
