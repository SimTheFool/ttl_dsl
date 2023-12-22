pub const DRONE_KANMUSHI: &str = r#"
@NAME Kanmushi

{
    <? ./drones/rules >
    
    name: Kanmushi
    manufacturer: Shiawase
    description: Petit arthropode mécanique
    
    status: free
    concealment: 6
    
    stats:
    {
        hit: 8
        maniability: 2
        speed_step: 4
        speed_max: 15
        autopilot: 2
        resistance: 0
        armor: 0
    }
    
    <? ./utils/quantity_buy with cost: 450>
}
"#;
