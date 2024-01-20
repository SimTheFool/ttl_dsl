use super::parser::{span_into_str, span_into_string, Rule};
use pest_ast::FromPest;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::identifier))]
pub struct Variable(#[pest_ast(outer(with(span_into_string)))] pub std::string::String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::reference))]
pub struct Ref(#[pest_ast(outer(with(span_into_string)))] pub std::string::String);
impl Ref {
    pub fn get_var_name(&self) -> &str {
        self.0
            .strip_prefix('$')
            .expect("Variable does not start with $")
    }
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::string))]
pub struct String(#[pest_ast(inner(with(span_into_string)))] pub std::string::String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::text))]
pub struct Text(#[pest_ast(inner(with(span_into_string)))] pub std::string::String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::number))]
pub struct Number(
    #[pest_ast(outer(with(span_into_str), with(str::parse::<f64>), with(Result::unwrap)))] pub f64,
);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::boolean))]
pub struct Boolean(
    #[pest_ast(outer(with(span_into_str), with(str::parse::<bool>), with(Result::unwrap)))] pub bool,
);

#[cfg(test)]
mod tests {
    use crate::{domain::ast::parser::TTLParser, print_unwrap};
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_identifier() {
        let str = "var01";
        let mut pairs = print_unwrap!(TTLParser::parse(super::Rule::identifier, str));
        let variable = print_unwrap!(super::Variable::from_pest(&mut pairs));

        assert_eq!(variable.0, "var01");
    }

    #[test]
    fn it_should_parse_text() {
        let str = r#"Moyen arthropode mécanique"#;
        let mut pairs = print_unwrap!(TTLParser::parse(super::Rule::text, str));
        let variable = print_unwrap!(super::Text::from_pest(&mut pairs));

        assert_eq!(variable.0, "Moyen arthropode mécanique");
    }
}
