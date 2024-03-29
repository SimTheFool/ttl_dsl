use super::{
    objects::Object,
    parser::Rule,
    primitives::{Number, Ref, String, Text, Variable},
    Boolean,
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
    Bool(Boolean),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::declare_direct_mark))]
pub struct DeclarationDirectMark();

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::declare_uniq_mark))]
pub struct DeclarationUniqMark();

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::declare_mark))]
pub enum DeclarationMark {
    Direct(DeclarationDirectMark),
    Uniq(DeclarationUniqMark),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::declaration))]
pub struct Declaration {
    pub metas: Option<Metas>,
    pub identifier: Variable,
    pub mark: DeclarationMark,
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
        print_unwrap,
    };
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_declaration() {
        let str = r#"["meta1" 15] var01! 745"#;
        let mut pairs = TTLParser::parse(super::Rule::declaration, str).unwrap();
        let Declaration {
            metas,
            identifier,
            mark,
            value,
        } = Declaration::from_pest(&mut pairs).unwrap();

        as_variant!(mark, super::DeclarationMark::Uniq);

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
    fn it_should_parse_declare_with_reference() {
        let str = r#"somevar: $var001"#;

        let mut pairs = TTLParser::parse(super::Rule::declaration, str).unwrap();
        let Declaration {
            identifier,
            mark,
            value,
            ..
        } = Declaration::from_pest(&mut pairs).unwrap();

        as_variant!(mark, super::DeclarationMark::Direct);

        assert_eq!(identifier.0, "somevar");
        let value = as_variant!(value, Value::Reference);
        assert_eq!(value.get_var_name(), "var001")
    }

    #[test]
    fn it_should_parse_string_declaration() {
        let str = r#"somevar: "aaa""#;

        let mut pairs = print_unwrap!(TTLParser::parse(super::Rule::declaration, str));
        let Declaration { value, .. } = print_unwrap!(super::Declaration::from_pest(&mut pairs));

        as_variant!(value, Value::String);
    }

    #[test]
    fn it_should_parse_boolean_declaration() {
        let str = r#"somevar: false"#;

        let mut pairs = print_unwrap!(TTLParser::parse(super::Rule::declaration, str));
        let Declaration { value, .. } = print_unwrap!(super::Declaration::from_pest(&mut pairs));

        let value = as_variant!(value, Value::Bool);
        assert!(!value.0);
    }
}
