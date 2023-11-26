use super::{
    objects::Object,
    parser::Rule,
    primitives::{Number, Ref, StringLit, Variable},
};
use pest_ast::FromPest;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::value))]
pub enum Value {
    String(StringLit),
    Number(Number),
    Object(Object),
    Reference(Ref),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::declaration))]
pub struct Declaration {
    pub metas: Option<Metas>,
    pub identifier: Variable,
    pub value: Value,
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::meta))]
pub enum Meta {
    String(StringLit),
    Number(Number),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::metas))]
pub struct Metas(pub Vec<Meta>);

#[cfg(test)]
mod tests {
    use crate::domain::ast::{
        parser::TTLParser,
        values::{Declaration, Value},
    };
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_declaration() {
        let str = r#"["meta1" 15]var01: 745"#;
        let mut pairs = TTLParser::parse(super::Rule::declaration, str).unwrap();
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
    fn it_should_parse_reference() {
        let str = r#"var001"#;

        let mut pairs = TTLParser::parse(super::Rule::reference, str).unwrap();
        let reference = super::Ref::from_pest(&mut pairs).unwrap();

        assert_eq!(reference.0, "var001");
    }

    #[test]
    fn it_should_parse_declaration_with_reference() {
        let str = r#"somevar: var001"#;

        let mut pairs = TTLParser::parse(super::Rule::declaration, str).unwrap();
        let declaration = Declaration::from_pest(&mut pairs).unwrap();

        let identifier = declaration.identifier;
        let value = declaration.value;

        assert_eq!(identifier.0, "somevar");
        let value = match value {
            Value::Reference(r) => r.0.clone(),
            _ => panic!("Unexpected value"),
        };
        assert_eq!(value, "var001")
    }
}
