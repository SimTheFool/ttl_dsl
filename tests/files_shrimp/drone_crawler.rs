pub const DRONE_CRAWLER: &str = r#"
@NAME Crawler

{
    <? ./drones/rules >
    
    name: Crawler
    manufacturer: Aztechnology
    description: Moyen arthropode mécanique

    price: 0
    quantity: 1
    
    status: free
    concealment: 2
    
    stats:
    {
        hit: 8
        maniability: 3
        speed_step: 8
        speed_max: 30
        autopilot: 2
        resistance: 6
        armor: 2
    }
    
    <? ./utils/buy with cost: 9500>
}
"#;
