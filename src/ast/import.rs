use super::{
    parser::{span_into_string, Rule},
    values::Declaration,
};
use pest_ast::FromPest;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::path))]
pub struct Path(#[pest_ast(outer(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct Import {
    pub path: Path,
    pub declarations: Vec<Declaration>,
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        import::Import,
        parser::TTLParser,
        values::{Declaration, Value},
    };
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_import() {
        let str = r#"<< ./stats
            with var01: 01
            with var02: "002"
        "#;

        let mut pairs = TTLParser::parse(super::Rule::import, str).unwrap();
        let Import { declarations, path } = super::Import::from_pest(&mut pairs).unwrap();

        assert_eq!(path.0, "./stats");

        let first_declaration = declarations.get(0).unwrap();
        let first_var = &first_declaration.identifier;
        let first_value = &first_declaration.value;
        let first_value = match first_value {
            Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        assert_eq!(first_var.0, "var01");
        assert_eq!(first_value, 1.0);

        let second_declaration = declarations.get(1).unwrap();
        let second_var = &second_declaration.identifier;
        let second_value = &second_declaration.value;
        let second_value = match second_value {
            Value::String(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        assert_eq!(second_var.0, "var02");
        assert_eq!(second_value, "002");
    }
}
