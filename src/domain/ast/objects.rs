use super::{import::Import, parser::Rule, values::Declaration};
use pest_ast::FromPest;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::object_element))]
pub enum ObjectElem {
    Declaration(Declaration),
    Import(Import),
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::object))]
pub struct Object(pub Vec<ObjectElem>);

#[cfg(test)]
mod tests {
    use crate::as_variant;
    use crate::domain::ast::{
        import::Import,
        objects::{Object, ObjectElem},
        parser::TTLParser,
        values::Value,
    };
    use crate::domain::ast::{Declaration, ImportConfig, ImportVariable};
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_object() {
        let str = r#"{
            <? ./import
                with var01: 01 >
            var02: 745
            var03: "hello"
        }"#;

        let mut pairs = TTLParser::parse(super::Rule::object, str).unwrap();
        let Object(elems) = Object::from_pest(&mut pairs).unwrap();

        assert_eq!(elems.len(), 3);

        let second_element = elems.get(1).unwrap();
        let Declaration {
            identifier, value, ..
        } = as_variant!(second_element, ObjectElem::Declaration);
        assert_eq!(identifier.0, "var02");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 745.0);

        let third_element = elems.get(2).unwrap();
        let Declaration {
            identifier, value, ..
        } = as_variant!(third_element, ObjectElem::Declaration);
        assert_eq!(identifier.0, "var03");
        let value = as_variant!(value, Value::String);
        assert_eq!(value.0, "hello");

        let Import {
            import_config,
            import_id,
            ..
        } = as_variant!(elems.get(0).unwrap(), ObjectElem::Import);

        assert_eq!(import_id.0, "./import");
        assert_eq!(import_config.len(), 1);

        let first_elements = import_config.get(0).unwrap();
        let ImportVariable { identifier, value } =
            as_variant!(first_elements, ImportConfig::Variable);
        assert_eq!(identifier.0, "var01");
        let value = as_variant!(value, Value::Number);
        assert_eq!(value.0, 1.0);
    }
}
