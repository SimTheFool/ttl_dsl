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
    use crate::domain::ast::{
        import::Import,
        objects::{Object, ObjectElem},
        parser::TTLParser,
        values::Value,
    };
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_reference_in_object() {
        let str = r#"{
            var02: reference
            var03: aaaa
        }"#;

        let mut pairs = TTLParser::parse(super::Rule::object, str).unwrap();
        let Object(elems) = Object::from_pest(&mut pairs).unwrap();

        assert_eq!(elems.len(), 2);

        let first_declaration = match elems.get(0).unwrap() {
            ObjectElem::Declaration(d) => d,
            _ => panic!("Should be declaration"),
        };

        let id = &first_declaration.identifier;
        let value = &first_declaration.value;

        assert_eq!(id.0, "var02");
        let value = match value {
            Value::Reference(r) => r.0.clone(),
            _ => panic!("Unexpected value"),
        };
        assert_eq!(value, "reference")
    }

    #[test]
    fn it_should_parse_object() {
        let str = r#"{
            << ./import
                with var01: 01
            var02: 745
            var03: "hello"
        }"#;

        let mut pairs = TTLParser::parse(super::Rule::object, str).unwrap();
        let Object(elems) = Object::from_pest(&mut pairs).unwrap();

        assert_eq!(elems.len(), 3);

        let Import {
            declarations,
            import_path: path,
        } = match elems.get(0).unwrap() {
            ObjectElem::Import(i) => i,
            _ => panic!("Should be import"),
        };

        let second_declaration = match elems.get(1).unwrap() {
            ObjectElem::Declaration(d) => d,
            _ => panic!("Shoudl be declaration"),
        };

        let third_declaration = match elems.get(2).unwrap() {
            ObjectElem::Declaration(d) => d,
            _ => panic!("Shoudl be declaration"),
        };

        let second_var = &second_declaration.identifier;
        let second_value = &second_declaration.value;
        let second_value = match second_value {
            Value::Number(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        let third_var = &third_declaration.identifier;
        let third_value = &third_declaration.value;
        let third_value = match third_value {
            Value::String(s) => s.0.clone(),
            _ => panic!("Unexpected value"),
        };

        assert_eq!(path.0, "./import");
        assert_eq!(declarations.len(), 1);

        let first_declaration = declarations.get(0).unwrap();
        let first_var = &first_declaration.identifier;
        let first_value = &first_declaration.value;
        let first_value = match first_value {
            Value::Number(n) => n.0,
            _ => panic!("Unexpected value"),
        };

        assert_eq!(first_var.0, "var01");
        assert_eq!(first_value, 1.0);
        assert_eq!(second_var.0, "var02");
        assert_eq!(second_value, 745.0);
        assert_eq!(third_var.0, "var03");
        assert_eq!(third_value, "hello");
    }
}
