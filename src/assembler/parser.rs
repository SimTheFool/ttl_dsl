use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "assembler/ttl.pest"]
pub struct TTLParser;

pub mod ast {
    use super::Rule;
    use pest::Span;
    use pest_ast::FromPest;

    fn span_into_string(span: Span) -> String {
        span.as_str().to_string()
    }
    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    #[derive(Debug, PartialEq, FromPest)]
    #[pest_ast(rule(Rule::variable))]
    pub struct Variable(#[pest_ast(outer(with(span_into_string)))] pub String);

    #[derive(Debug, PartialEq, FromPest)]
    #[pest_ast(rule(Rule::string))]
    pub struct StringLit(#[pest_ast(inner(with(span_into_string)))] pub String);

    #[derive(Debug, PartialEq, FromPest)]
    #[pest_ast(rule(Rule::number))]
    pub struct Number(
        #[pest_ast(outer(with(span_into_str), with(str::parse::<f64>), with(Result::unwrap)))]
        pub  f64,
    );

    #[derive(Debug, PartialEq, FromPest)]
    #[pest_ast(rule(Rule::object))]
    pub struct Object(pub Vec<Declaration>);

    #[derive(Debug, PartialEq, FromPest)]
    #[pest_ast(rule(Rule::value))]
    pub enum Value {
        String(StringLit),
        Number(Number),
        Object(Object),
    }

    #[derive(Debug, PartialEq, FromPest)]
    #[pest_ast(rule(Rule::declaration))]
    pub struct Declaration(pub Variable, pub Value);

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::EOI))]
    struct EOI;

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::file))]
    pub struct File {
        pub value: Value,
        _eoi: EOI,
    }
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::assembler::parser::ast::{Declaration, Variable};

    #[test]
    fn it_should_parse_variable() {
        let str = "var01";
        let mut pairs = super::TTLParser::parse(super::Rule::variable, str).unwrap();
        let variable = Variable::from_pest(&mut pairs).unwrap();

        assert_eq!(variable.0, "var01");
    }

    #[test]
    fn it_should_parse_declaration() {
        let str = "var01: 745";
        let mut pairs = super::TTLParser::parse(super::Rule::declaration, str).unwrap();
        let declaration = Declaration::from_pest(&mut pairs).unwrap();

        let variable = declaration.0;
        let value = declaration.1;
        let value = match value {
            super::ast::Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        assert_eq!(variable.0, "var01");
        assert_eq!(value, 745.0);
    }

    #[test]
    fn it_should_parse_object() {
        let str = r#"{
            var02: 745
            var03: "hello"
        }"#;

        let mut pairs = super::TTLParser::parse(super::Rule::object, str).unwrap();
        let object = super::ast::Object::from_pest(&mut pairs).unwrap();

        let declarations = object.0;

        assert_eq!(declarations.len(), 2);

        let first_declaration = declarations.get(0).unwrap();
        let first_var = &first_declaration.0;
        let first_value = &first_declaration.1;
        let first_value = match first_value {
            super::ast::Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        let second_declaration = declarations.get(1).unwrap();
        let second_var = &second_declaration.0;
        let second_value = &second_declaration.1;
        let second_value = match second_value {
            super::ast::Value::String(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        assert_eq!(first_var.0, "var02");
        assert_eq!(first_value, 745.0);
        assert_eq!(second_var.0, "var03");
        assert_eq!(second_value, "hello");
    }

    #[test]
    fn it_should_parse_file() {
        let str = r#"
        {
            var02: 745
            var03: "hello"
        }
        "#;

        let mut pairs = super::TTLParser::parse(super::Rule::file, str).unwrap();
        let file = super::ast::File::from_pest(&mut pairs).unwrap();
        let value = file.value;

        let declarations = match value {
            super::ast::Value::Object(s) => s.0,
            _ => panic!("Unexpected value"),
        };

        let first_declaration = declarations.get(0).unwrap();
        let first_var = &first_declaration.0;
        let first_value = &first_declaration.1;
        let first_value = match first_value {
            super::ast::Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        let second_declaration = declarations.get(1).unwrap();
        let second_var = &second_declaration.0;
        let second_value = &second_declaration.1;
        let second_value = match second_value {
            super::ast::Value::String(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        assert_eq!(first_var.0, "var02");
        assert_eq!(first_value, 745.0);
        assert_eq!(second_var.0, "var03");
        assert_eq!(second_value, "hello");
    }
}
