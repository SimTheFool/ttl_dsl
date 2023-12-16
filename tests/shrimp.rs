use crate::utils::*;
use custom_dsl::domain::resolution::{ResolvedResource, ResolvedResourceValue};
use files_shrimp::*;
use regex::Regex;

#[macro_use]
mod utils;
mod files_shrimp;

const INDEX: &str = r#"
    {
        nuyens: 70000

        stats:
        {
            <? ./stats_base
                with con : 1
                with agi : 4
                with rea : 2
                with for : 1
                with vol : 4
                with log : 6
                with int : 4
                with cha : 2
                with ess : 6 >
            <? ./stats_techno
                with res : 7
                with submersion : 1 >
        }

        inventory:
        {
            <@ ./drones/crawler
                with <? ./drones_mods/monture >
                with <? ./drones_mods/monture >
            >

            <@ ./drones/kanmushi
                with <? ./utils/quantity with q: 2 >
            >
        }
    }
"#;

#[test]
fn it_shoud_assemble_shrimp() {
    let (app, resolver, config) = MockedApp::new();

    config.borrow_mut().add_layer("FINAL_STATS");
    config.borrow_mut().add_layer("FINAL_STATS_END");
    config.borrow_mut().add_layer("BUY_FINAL");

    resolver.borrow_mut().mock_file("./stats_base", STATS_BASE);
    resolver
        .borrow_mut()
        .mock_file("./stats_techno", STATS_TECHNO);
    resolver
        .borrow_mut()
        .mock_file("./drones/rules", DRONE_RULES);
    resolver
        .borrow_mut()
        .mock_file("./drones/crawler", DRONE_CRAWLER);
    resolver
        .borrow_mut()
        .mock_file("./drones/kanmushi", DRONE_KANMUSHI);
    resolver
        .borrow_mut()
        .mock_file("./drones_mods/monture", DRONE_MOD_MONTURE);
    resolver.borrow_mut().mock_file("./utils/buy", UTILS_BUY);
    resolver
        .borrow_mut()
        .mock_file("./utils/quantity", UTILS_QUANTITY);

    let resources = app.assemble_from_str(INDEX);
    let resources = print_unwrap!(resources);

    println!("RESOURCES: {:#?}", resources);

    /* Testing base stats */
    assert_resource_at!(resources : "stats.con" => Number 1.0);
    assert_resource_at!(resources : "stats.con_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.agi" => Number 4.0);
    assert_resource_at!(resources : "stats.agi_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.rea" => Number 2.0);
    assert_resource_at!(resources : "stats.rea_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.for" => Number 1.0);
    assert_resource_at!(resources : "stats.for_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.vol" => Number 4.0);
    assert_resource_at!(resources : "stats.vol_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.log" => Number 6.0);
    assert_resource_at!(resources : "stats.log_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.int" => Number 4.0);
    assert_resource_at!(resources : "stats.int_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.cha" => Number 2.0);
    assert_resource_at!(resources : "stats.cha_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.ess" => Number 6.0);
    assert_resource_at!(resources : "stats.edge" => Number 4.0);
    assert_resource_at!(resources : "stats.resist_phy" => Number 1.0);
    assert_resource_at!(resources : "stats.resist_ment" => Number 4.0);
    assert_resource_at!(resources : "stats.def_phy" => Number 6.0);
    assert_resource_at!(resources : "stats.def_ment" => Number 8.0);
    assert_resource_at!(resources : "stats.init_dice" => Number 1.0);
    assert_resource_at!(resources : "stats.init_base" => Number 6.0);
    assert_resource_at!(resources : "stats.action_maj" => Number 1.0);
    assert_resource_at!(resources : "stats.action_min" => Number 2.0);
    assert_resource_at!(resources : "stats.hit_phy" => Number 8.0);
    assert_resource_at!(resources : "stats.hit_stun" => Number 10.0);
    assert_resource_at!(resources : "stats.hit_over" => Number 1.0);
    assert_resource_at!(resources : "stats.heal" => Number 5.0);

    assert_resource_at!(resources : "stats.res" => Number 7.0);
    assert_resource_at!(resources : "stats.submersion" => Number 1.0);
    assert_resource_at!(resources : "stats.resist_drain" => Number 3.0);
    assert_resource_at!(resources : "stats.firewall" => Number 4.0);
    assert_resource_at!(resources : "stats.traitement" => Number 6.0);
    assert_resource_at!(resources : "stats.corruption" => Number 4.0);
    assert_resource_at!(resources : "stats.attaque" => Number 2.0);

    /* Testing drone rules */
    assert_resource_at!(resources : "inventory.Crawler.stats.hit" => Number 11.0);

    /* Testing drone mods */
    let crawler_slot_regex =
        Regex::new(r#"inventory\.Crawler\.slots\.([a-zA-Z0-9]+)\.name"#).unwrap();
    let crawler_slots = resources
        .iter()
        .filter(|ResolvedResource { identifier, .. }| match identifier {
            Some(i) => crawler_slot_regex.is_match(i),
            _ => false,
        })
        .collect::<Vec<_>>();
    assert_eq!(crawler_slots.len(), 2);

    /* Testing buy util */
    assert_resource_at!(resources : "inventory.Crawler.price" => Number {9500.0 + 2500.0 + 2500.0});
    assert_resource_at!(resources : "inventory.Kanmushi.price" => Number {450.0 * 2.0});
    assert_resource_at!(resources : "nuyens" => Number {70000.0 - 9500.0 - 2500.0 - 2500.0 - 450.0 * 2.0});
}
