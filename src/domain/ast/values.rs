use super::{
    objects::Object,
    parser::Rule,
    primitives::{Number, Ref, String, Text, Variable},
};
use pest_ast::FromPest;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::value))]
pub enum Value {
    Text(Text),
    String(String),
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
    String(String),
    Number(Number),
    Reference(Ref),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::metas))]
pub struct Metas(pub Vec<Meta>);

#[cfg(test)]
mod tests {
    use crate::{
        as_variant,
        domain::ast::{
            parser::TTLParser,
            values::{Declaration, Value},
        },
    };
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_declaration() {
        let str = r#"["meta1" 15]var01: 745"#;
        let mut pairs = TTLParser::parse(super::Rule::declaration, str).unwrap();
        println!("{:#?}", pairs);
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
        let meta_0 = as_variant!(metas.get(0).unwrap(), super::Meta::String);
        assert_eq!(meta_0.0, "meta1");

        let meta_1 = as_variant!(metas.get(1).unwrap(), super::Meta::Number);
        assert_eq!(meta_1.0, 15.0);
    }

    #[test]
    fn it_should_parse_reference() {
        let str = r#"$var001"#;

        let mut pairs = TTLParser::parse(super::Rule::reference, str).unwrap();
        let reference = super::Ref::from_pest(&mut pairs).unwrap();

        assert_eq!(reference.get_var_name(), "var001");
    }

    #[test]
    fn it_should_parse_declaration_with_reference() {
        let str = r#"somevar: $var001"#;

        let mut pairs = TTLParser::parse(super::Rule::declaration, str).unwrap();
        let declaration = Declaration::from_pest(&mut pairs).unwrap();

        let identifier = declaration.identifier;
        let value = declaration.value;

        assert_eq!(identifier.0, "somevar");
        let value = as_variant!(value, Value::Reference);
        assert_eq!(value.get_var_name(), "var001")
    }
}
