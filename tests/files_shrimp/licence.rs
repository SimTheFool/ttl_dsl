pub const LICENCE: &str = r#"
{
    name: $name
    description: $description

    quality: i$quality
    price: 0
    quantity: $quality

    <? ./utils/buy with cost: 200>
}"#;
