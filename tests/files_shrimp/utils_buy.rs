pub const UTILS_BUY: &str = r#"
@TRANSFORM BUY_FINAL
> $.price += $cost * $.quantity
> nuyens -= $cost * $.quantity
"#;
