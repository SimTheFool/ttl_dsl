pub const UTILS_QUALITY_BUY: &str = r#"
{
    price: 0
    quality: 1
}
@TRANSFORM BUY_FINAL
> $.price += $cost * $.quality
> nuyens -= $cost * $.quality
"#;
