pub const IDENTITY_FAKE: &str = r#"
{
    name: $name
    quality: i$quality

    price: 0
    quantity: $quality

    <? ./utils/buy with cost: 2500>
}
"#;
