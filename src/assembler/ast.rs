use crate::utils::result::AppError;
use crate::utils::result::AppResult;
use pest::Parser;
use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "assembler/ttl.pest"]
struct TTLParser;

fn span_into_string(span: Span) -> String {
    span.as_str().to_string()
}
fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::identifier))]
pub struct Variable(#[pest_ast(outer(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::reference))]
pub struct Ref(#[pest_ast(outer(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::string))]
pub struct StringLit(#[pest_ast(inner(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::number))]
pub struct Number(
    #[pest_ast(outer(with(span_into_str), with(str::parse::<f64>), with(Result::unwrap)))] pub f64,
);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::object_element))]
pub enum ObjectElem {
    Declaration(Declaration),
    Integration(Integration),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::object))]
pub struct Object(pub Vec<ObjectElem>);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::meta))]
pub enum Meta {
    String(StringLit),
    Number(Number),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::metas))]
pub struct Metas(pub Vec<Meta>);
impl Into<(Option<Vec<String>>, Option<Vec<String>>)> for Metas {
    fn into(self) -> (Option<Vec<String>>, Option<Vec<String>>) {
        let mut meta_lit = None;
        let meta_ref = None;
        for meta in self.0 {
            match meta {
                Meta::String(s) => meta_lit.get_or_insert(vec![]).push(s.0),
                Meta::Number(n) => meta_lit.get_or_insert(vec![]).push(n.0.to_string()),
            };
        }

        (meta_lit, meta_ref)
    }
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::value))]
pub enum Value {
    String(StringLit),
    Number(Number),
    Object(Object),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::path))]
pub struct Path(#[pest_ast(outer(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::integration))]
pub struct Integration {
    pub path: Path,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::declaration))]
pub struct Declaration {
    pub metas: Option<Metas>,
    pub identifier: Variable,
    pub value: Value,
}

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
    use crate::assembler::ast::{Integration, Object};

    use super::{Declaration, Variable};
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_identifier() {
        let str = "var01";
        let mut pairs = super::TTLParser::parse(super::Rule::identifier, str).unwrap();
        let variable = Variable::from_pest(&mut pairs).unwrap();

        assert_eq!(variable.0, "var01");
    }

    #[test]
    fn it_should_parse_declaration() {
        let str = r#"["meta1" 15]var01: 745"#;
        let mut pairs = super::TTLParser::parse(super::Rule::declaration, str).unwrap();
        let declaration = Declaration::from_pest(&mut pairs).unwrap();

        let identifier = declaration.identifier;
        let value = declaration.value;
        let metas = declaration.metas;
        let value = match value {
            super::Value::Number(m) => m.0,
            _ => panic!("Unexpected value"),
        };

        assert_eq!(identifier.0, "var01");
        assert_eq!(value, 745.0);

        let metas = metas.unwrap().0;
        assert_eq!(metas.len(), 2);
        match metas.get(0).unwrap() {
            super::Meta::String(s) => assert_eq!(s.0, "meta1"),
            _ => panic!("Unexpected meta"),
        }
        match metas.get(1).unwrap() {
            super::Meta::Number(n) => assert_eq!(n.0, 15.0),
            _ => panic!("Unexpected meta"),
        }
    }

    #[test]
    fn it_should_parse_integration() {
        let str = r#"<< ./stats
            with var01: 01
            with var02: "002"
        "#;

        let mut pairs = super::TTLParser::parse(super::Rule::integration, str).unwrap();
        let Integration { declarations, path } = super::Integration::from_pest(&mut pairs).unwrap();

        assert_eq!(path.0, "./stats");

        let first_declaration = declarations.get(0).unwrap();
        let first_var = &first_declaration.identifier;
        let first_value = &first_declaration.value;
        let first_value = match first_value {
            super::Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        assert_eq!(first_var.0, "var01");
        assert_eq!(first_value, 1.0);

        let second_declaration = declarations.get(1).unwrap();
        let second_var = &second_declaration.identifier;
        let second_value = &second_declaration.value;
        let second_value = match second_value {
            super::Value::String(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        assert_eq!(second_var.0, "var02");
        assert_eq!(second_value, "002");
    }

    #[test]
    fn it_should_parse_object() {
        let str = r#"{
            << ./integration
                with var01: 01
            var02: 745
            var03: "hello"
        }"#;

        let mut pairs = super::TTLParser::parse(super::Rule::object, str).unwrap();
        let Object(elems) = super::Object::from_pest(&mut pairs).unwrap();

        assert_eq!(elems.len(), 3);

        let Integration { declarations, path } = match elems.get(0).unwrap() {
            super::ObjectElem::Integration(i) => i,
            _ => panic!("Should be integration"),
        };

        let second_declaration = match elems.get(1).unwrap() {
            super::ObjectElem::Declaration(d) => d,
            _ => panic!("Shoudl be declaration"),
        };

        let third_declaration = match elems.get(2).unwrap() {
            super::ObjectElem::Declaration(d) => d,
            _ => panic!("Shoudl be declaration"),
        };

        let second_var = &second_declaration.identifier;
        let second_value = &second_declaration.value;
        let second_value = match second_value {
            super::Value::Number(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        let third_var = &third_declaration.identifier;
        let third_value = &third_declaration.value;
        let third_value = match third_value {
            super::Value::String(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        assert_eq!(path.0, "./integration");
        assert_eq!(declarations.len(), 1);

        let first_declaration = declarations.get(0).unwrap();
        let first_var = &first_declaration.identifier;
        let first_value = &first_declaration.value;
        let first_value = match first_value {
            super::Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        assert_eq!(first_var.0, "var01");
        assert_eq!(first_value, 1.0);
        assert_eq!(second_var.0, "var02");
        assert_eq!(second_value, 745.0);
        assert_eq!(third_var.0, "var03");
        assert_eq!(third_value, "hello");
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
        let file = super::File::from_pest(&mut pairs).unwrap();
        let value = file.value;

        let object_elems = match value {
            super::Value::Object(s) => s.0,
            _ => panic!("Unexpected value"),
        };

        let first_declaration = match object_elems.get(0).unwrap() {
            super::ObjectElem::Declaration(d) => d,
            _ => panic!("Shoudl be declaration"),
        };
        let first_var = &first_declaration.identifier;
        let first_value = &first_declaration.value;
        let first_value = match first_value {
            super::Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        let second_declaration = match object_elems.get(1).unwrap() {
            super::ObjectElem::Declaration(d) => d,
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
