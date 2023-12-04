use super::{span_into_string, Rule};
use pest_ast::FromPest;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::transform_layer))]
pub struct TransformLayer(#[pest_ast(inner(with(span_into_string)))] pub String);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::transform_rule))]
pub struct TransformRule(#[pest_ast(outer(with(span_into_string)))] pub String);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::transform))]
pub struct Transform {
    pub layer: TransformLayer,
    pub rules: Option<Vec<TransformRule>>,
}

#[cfg(test)]
mod tests {
    use super::Transform;
    use crate::domain::ast::{TTLParser, TransformRule};
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_transform_rule() {
        let str = r#"$.resist_drain += floor($ctx.$trad / 2)"#;

        let mut pairs = TTLParser::parse(super::Rule::transform_rule, str).unwrap();
        let rule = TransformRule::from_pest(&mut pairs).unwrap().0;

        assert_eq!(rule, "$.resist_drain += floor($ctx.$trad / 2)");
    }

    #[test]
    fn it_should_parse_transform() {
        let str = r#"@TRANSFORM FINAL_LAYER
        > $.resist_phy += $ctx.con
        > $.resist_ment += $ctx.vol
        > $.hit += floor($ctx.con / 2)
        > $.heal += $ctx.con + $ctx.vol"#;

        let mut pairs = TTLParser::parse(super::Rule::transform, str).unwrap();
        let transform = Transform::from_pest(&mut pairs).unwrap();
        let layer = transform.layer.0;
        let rules = transform.rules.unwrap();

        assert_eq!(layer, "FINAL_LAYER");
        assert_eq!(rules.len(), 4);
        assert_eq!(rules.get(0).unwrap().0, "$.resist_phy += $ctx.con");
        assert_eq!(rules.get(1).unwrap().0, "$.resist_ment += $ctx.vol");
        assert_eq!(rules.get(2).unwrap().0, "$.hit += floor($ctx.con / 2)");
        assert_eq!(rules.get(3).unwrap().0, "$.heal += $ctx.con + $ctx.vol");
    }
}
