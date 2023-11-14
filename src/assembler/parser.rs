use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "assembler/ttl.pest"]
pub struct TTLParser;

#[cfg(test)]
mod tests {
    use pest::{iterators::Pair, Parser};

    #[test]
    fn it_should_parse_variable() {
        let str = "var01 f...";
        let pairs = super::TTLParser::parse(super::Rule::variable, str).unwrap();

        assert!(pairs.len() == 1);
        let pair = pairs.clone().next().unwrap();
        let span = pair.as_span();
        let inner = pair.into_inner();

        assert_eq!(inner.len(), 0);
        assert_eq!(span.as_str(), str);
    }

    #[test]
    fn it_should_parse_declaration() {
        let str = "var01: 745";
        let pairs = super::TTLParser::parse(super::Rule::declaration, str).unwrap();

        assert!(pairs.len() == 1);
        let pair = pairs.clone().next().unwrap();
        let span = pair.as_span();
        let inner = pair.into_inner();

        assert_eq!(inner.len(), 2);
        assert_eq!(span.as_str(), str);

        inner.for_each(|r| {
            let rule = r.as_rule();
            match rule {
                super::Rule::variable => {
                    assert_eq!(r.as_span().as_str(), "var01");
                }
                super::Rule::value => {
                    assert_eq!(r.as_span().as_str(), "745");
                }
                _ => {
                    assert!(false);
                }
            }
        });
    }

    #[test]
    fn it_should_parse_object() {
        let str = r#"{
            var02: 745
            var03: "hello"
        }"#;

        let pairs = super::TTLParser::parse(super::Rule::object, str).unwrap();

        assert!(pairs.len() == 1);
        let pair = pairs.clone().next().unwrap();
        let span = pair.as_span();
        let inner = pair.into_inner();

        assert_eq!(inner.len(), 2);
        assert_eq!(span.as_str(), str);

        let inner_pairs = inner.collect::<Vec<Pair<_>>>();
        let first_declaration = inner_pairs.get(0).unwrap();
        let second_declaration = inner_pairs.get(1).unwrap();

        assert_eq!(first_declaration.as_rule(), super::Rule::declaration);
        assert_eq!(second_declaration.as_rule(), super::Rule::declaration);
        assert_eq!(first_declaration.as_span().as_str(), "var02: 745");
        assert_eq!(second_declaration.as_span().as_str(), "var03: \"hello\"");
    }

    #[test]
    fn it_should_parse_file() {
        let str = r#"
        {
            var02: 745
            var03: "hello"
        }
        "#;

        let pairs = super::TTLParser::parse(super::Rule::file, str).unwrap();

        assert!(pairs.len() == 1);
        let pair = pairs.clone().next().unwrap();

        let span = pair.as_span();
        assert_eq!(span.as_str(), str);

        let value_pair = pair
            .into_inner()
            .find(|p| p.as_rule() == super::Rule::value)
            .unwrap();

        let object_pair = value_pair.into_inner().next_back().unwrap();

        let declarations_pairs = object_pair.into_inner().collect::<Vec<Pair<_>>>();

        let first_declaration = declarations_pairs.get(0).unwrap();
        let second_declaration = declarations_pairs.get(1).unwrap();

        assert_eq!(first_declaration.as_rule(), super::Rule::declaration);

        assert_eq!(second_declaration.as_rule(), super::Rule::declaration);

        assert_eq!(first_declaration.as_span().as_str(), "var02: 745");

        assert_eq!(second_declaration.as_span().as_str(), "var03: \"hello\"");
    }
}
