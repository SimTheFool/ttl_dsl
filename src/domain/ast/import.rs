use super::{
    parser::{span_into_string, Rule},
    Value, Variable,
};
use pest_ast::FromPest;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_id))]
pub struct ImportId(#[pest_ast(outer(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_variable))]
pub struct ImportVariable {
    pub identifier: Variable,
    pub value: Value,
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_config))]
pub enum ImportConfig {
    Variable(ImportVariable),
    Import(Import),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_anon_mark))]
pub struct ImportAnonMark();

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_default_name_mark))]
pub struct ImportDefaultNameMark();

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_uniq_mark))]
pub struct ImportUniqMark();

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_named_mark))]
pub struct ImportNamedMark(#[pest_ast(inner(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_mark))]
pub enum ImportMark {
    Anon(ImportAnonMark),
    Default(ImportDefaultNameMark),
    Named(ImportNamedMark),
    Uniq(ImportUniqMark),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct Import {
    pub import_mark: ImportMark,
    pub import_id: ImportId,
    pub import_config: Vec<ImportConfig>,
}

#[cfg(test)]
mod tests {
    use crate::domain::ast::Import;
    use crate::domain::ast::{parser::TTLParser, values::Value, ImportConfig, ImportVariable};
    use crate::{as_variant, print_unwrap};
    use from_pest::FromPest;
    use pest::Parser;

    use super::ImportNamedMark;

    #[test]
    fn it_should_parse_import() {
        let str = r#"
        <? ./stats
            with var01: 01
            with var02: "002" >
        "#
        .trim();

        let mut pairs = print_unwrap!(TTLParser::parse(super::Rule::import, str));
        let Import {
            import_config,
            import_id,
            ..
        } = print_unwrap!(super::Import::from_pest(&mut pairs));

        assert_eq!(import_id.0, "./stats");

        let first_element = import_config.get(0).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(first_element, ImportConfig::Variable);
        assert_eq!(identifier.0, "var01");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 1.0);

        let second_element = import_config.get(1).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(second_element, ImportConfig::Variable);
        assert_eq!(identifier.0, "var02");
        let value = as_variant!(value, Value::String);
        assert_eq!(value.0, "002");
    }

    #[test]
    fn it_should_parse_nested_import() {
        let str = r#"
        <? ./root
            with aaa: 01
            with <? ./nested
                with zzz: 02
                with yyy: 03 >
            with bbb: "002" >
        "#
        .trim();

        let mut pairs = TTLParser::parse(super::Rule::import, str).unwrap();
        let Import {
            import_config,
            import_id,
            ..
        } = super::Import::from_pest(&mut pairs).unwrap();

        assert_eq!(import_id.0, "./root");

        /* Testing top level declarations */
        let first_element = import_config.get(0).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(first_element, ImportConfig::Variable);

        assert_eq!(identifier.0, "aaa");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 1.0);

        let third_element = import_config.get(2).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(third_element, ImportConfig::Variable);
        assert_eq!(identifier.0, "bbb");
        let value = as_variant!(value, Value::String);
        assert_eq!(value.0, "002");

        /* Testing nested import */
        let import = import_config.get(1).unwrap();
        let Import {
            import_config,
            import_id,
            ..
        } = as_variant!(import, ImportConfig::Import);

        assert_eq!(import_id.0, "./nested");

        let first_config = import_config.get(0).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(first_config, ImportConfig::Variable);
        assert_eq!(identifier.0, "zzz");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 2.0);

        let second_config = import_config.get(1).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(second_config, ImportConfig::Variable);
        assert_eq!(identifier.0, "yyy");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 3.0);
    }

    #[test]
    fn it_should_parse_default_name_import() {
        let str = r#"
        <@ ./root with aaa: 01 >
        "#
        .trim();

        let mut pairs = print_unwrap!(TTLParser::parse(super::Rule::import, str));
        let Import {
            import_config,
            import_id,
            import_mark,
        } = print_unwrap!(super::Import::from_pest(&mut pairs));

        assert_eq!(import_id.0, "./root");
        let _import_mark = as_variant!(import_mark, super::ImportMark::Default);

        let first_element = import_config.get(0).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(first_element, ImportConfig::Variable);

        assert_eq!(identifier.0, "aaa");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 1.0);
    }

    #[test]
    fn it_should_parse_uniq_import() {
        let str = r#"
        <! ./root >
        "#
        .trim();

        let mut pairs = TTLParser::parse(super::Rule::import, str).unwrap();
        let Import { import_mark, .. } = super::Import::from_pest(&mut pairs).unwrap();

        let _import_mark = as_variant!(import_mark, super::ImportMark::Uniq);
    }

    #[test]
    fn it_should_parse_named_import() {
        let str = r#"
        <something indeed| ./root >
        "#
        .trim();

        let mut pairs = TTLParser::parse(super::Rule::import, str).unwrap();
        let Import { import_mark, .. } = super::Import::from_pest(&mut pairs).unwrap();

        let ImportNamedMark(name) = as_variant!(import_mark, super::ImportMark::Named);
        assert_eq!(name, "something indeed");
    }
}
