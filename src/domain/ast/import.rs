use super::{
    parser::{span_into_string, Rule},
    Value, Variable,
};
use pest_ast::FromPest;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_path))]
pub struct ImportPath(#[pest_ast(outer(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_variable))]
pub struct ImportVariable {
    pub identifier: Variable,
    pub value: Value,
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import_elem))]
pub enum ImportElement {
    Variable(ImportVariable),
    Import(Import),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct Import {
    pub import_path: ImportPath,
    pub import_elements: Vec<ImportElement>,
}

#[cfg(test)]
mod tests {
    use crate::as_variant;
    use crate::domain::ast::{
        import::Import, parser::TTLParser, values::Value, ImportElement, ImportVariable,
    };
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_import() {
        let str = r#"
        << ./stats
            with var01: 01
            with var02: "002" >
        "#
        .trim();

        let mut pairs = TTLParser::parse(super::Rule::import, str).unwrap();
        let Import {
            import_elements,
            import_path: path,
        } = super::Import::from_pest(&mut pairs).unwrap();

        assert_eq!(path.0, "./stats");

        let first_element = import_elements.get(0).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(first_element, ImportElement::Variable);
        assert_eq!(identifier.0, "var01");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 1.0);

        let second_element = import_elements.get(1).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(second_element, ImportElement::Variable);
        assert_eq!(identifier.0, "var02");
        let value = as_variant!(value, Value::String);
        assert_eq!(value.0, "002");
    }

    #[test]
    fn it_should_parse_nested_import() {
        let str = r#"
        << ./root
            with aaa: 01
            with << ./nested
                with zzz: 02
                with yyy: 03 >
            with bbb: "002" >
        "#
        .trim();

        let mut pairs = TTLParser::parse(super::Rule::import, str).unwrap();
        let Import {
            import_elements,
            import_path,
        } = super::Import::from_pest(&mut pairs).unwrap();

        assert_eq!(import_path.0, "./root");

        /* Testing top level declarations */
        let first_element = import_elements.get(0).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(first_element, ImportElement::Variable);

        assert_eq!(identifier.0, "aaa");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 1.0);

        let third_element = import_elements.get(2).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(third_element, ImportElement::Variable);
        assert_eq!(identifier.0, "bbb");
        let value = as_variant!(value, Value::String);
        assert_eq!(value.0, "002");

        /* Testing nested import */
        let import_element = import_elements.get(1).unwrap();
        let Import {
            import_elements,
            import_path,
        } = as_variant!(import_element, ImportElement::Import);
        assert_eq!(import_path.0, "./nested");

        let first_element = import_elements.get(0).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(first_element, ImportElement::Variable);
        assert_eq!(identifier.0, "zzz");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 2.0);

        let second_element = import_elements.get(1).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(second_element, ImportElement::Variable);
        assert_eq!(identifier.0, "yyy");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 3.0);
    }
}
