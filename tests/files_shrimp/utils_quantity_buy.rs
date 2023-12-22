pub const UTILS_QUANTITY_BUY: &str = r#"
{
    price: 0
    quantity: 1
}
@TRANSFORM BUY_FINAL
> $.price += $cost * $.quantity
> nuyens -= $cost * $.quantity
"#;
