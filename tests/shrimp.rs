use std::collections::HashSet;

use crate::utils::*;
use custom_dsl::domain::resolution::{ResolvedResource, ResolvedResourceValue};
use files_shrimp::*;
use regex::Regex;

#[macro_use]
mod utils;
mod files_shrimp;

const INDEX: &str = r#"
    {
        name: Shrimp

        <? ./metatypes/human >
        tags! "13 ans"
        tags! technorigger

        knowledges! Jeux tridéos
        knowledges! Séries tridéos
        knowledges! Drônes

        nuyens: 70000

        stats:
        {
            <? ./stats/base
                with con : 1
                with agi : 4
                with rea : 2
                with for : 1
                with vol : 4
                with log : 6
                with int : 4
                with cha : 2
                with ess : 6 >
            <? ./stats/techno
                with res : 7
                with submersion : 1 >
        }

        skills:
        {
            <! ./skill
                with name: combat rapproché
                with score: 1
            >
            <! ./skill
                with name: perception
                with score: 1
            >
            <! ./skill
                with name: furtivité
                with score: 1
            >
            <! ./skill
                with name: athlétisme
                with score: 3
            >
            <! ./skill
                with name: électronique
                with score: 4
            >
            <! ./skill
                with <? ./skill/spec with name: "Ingénierie" >
                with name: ingénierie
                with score: 6
            >
            <! ./skill
                with name: pilotage
                with score: 6
                with <? ./skill/spec with name: "Appareils aux sols" >
            >
            <! ./skill
                with name: technomancie
                with score: 6
                with <? ./skill/spec with name: "Compilation" >
                with <? ./skill/mast with name: "Inscription" >
            >       
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

    resolver.borrow_mut().mock_file("./stats/base", STATS_BASE);
    resolver
        .borrow_mut()
        .mock_file("./stats/techno", STATS_TECHNO);
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
    resolver
        .borrow_mut()
        .mock_file("./metatypes/human", METATYPE_HUMAN);
    resolver.borrow_mut().mock_file("./skill", SKILL);
    resolver.borrow_mut().mock_file("./skill/spec", SKILL_SPEC);
    resolver.borrow_mut().mock_file("./skill/mast", SKILL_MAST);

    let resources = app.assemble_from_str(INDEX);
    let resources = print_unwrap!(resources);

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

    /* Testing tags */
    let tags_regex = Regex::new(r#"tags\.([a-zA-Z0-9]+)"#).unwrap();
    let tags = resources
        .iter()
        .filter(|ResolvedResource { identifier, .. }| match identifier {
            Some(i) => tags_regex.is_match(i),
            _ => false,
        })
        .collect::<Vec<_>>();
    assert_eq!(tags.len(), 3);

    let first_tag = *tags.get(1).unwrap();
    let value = as_variant!(&first_tag.value, ResolvedResourceValue::String);
    assert_eq!(value, "13 ans");
    let second_tag = tags.get(2).unwrap();
    let value = as_variant!(&second_tag.value, ResolvedResourceValue::String);
    assert_eq!(value, "technorigger");

    /* Testing skills */
    let skills = get_uniq_keys("skills", vec!["score", "name"], &resources);
    assert_eq!(skills.len(), 8);
    skills.iter().for_each(|vec| match vec.as_slice() {
        [score, name] => {
            let name = as_variant!(&name.value, ResolvedResourceValue::String);
            let score = as_variant!(&score.value, ResolvedResourceValue::Number);
            match name.as_str() {
                "combat rapproché" => assert_eq!(score, &1.0),
                "perception" => assert_eq!(score, &1.0),
                "furtivité" => assert_eq!(score, &1.0),
                "athlétisme" => assert_eq!(score, &3.0),
                "électronique" => assert_eq!(score, &4.0),
                "ingénierie" => assert_eq!(score, &6.0),
                "pilotage" => assert_eq!(score, &6.0),
                "technomancie" => assert_eq!(score, &6.0),
                _ => panic!("Unknown skill: {}", name),
            }
        }
        _ => panic!("Unknown skill"),
    });

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

fn get_uniq_keys<'a>(
    prefix: &str,
    suffixes: Vec<&str>,
    resources: &'a Vec<ResolvedResource>,
) -> Vec<Vec<&'a ResolvedResource>> {
    let regex = Regex::new(format!(r#"{}\.([a-zA-Z0-9]+)"#, prefix).as_str()).unwrap();
    let uniq_keys = resources
        .iter()
        .filter_map(|ResolvedResource { identifier, .. }| match identifier {
            Some(i) => {
                let capture = regex
                    .captures(i)
                    .map(|c| c.get(0))
                    .flatten()
                    .map(|c| c.as_str());
                capture
            }
            _ => None,
        })
        .collect::<HashSet<_>>();

    let resources_tuples: Vec<Vec<_>> = uniq_keys
        .into_iter()
        .map(|t| {
            let resources: Vec<&ResolvedResource> = suffixes
                .iter()
                .map(|s| format!("{}.{}", t, s))
                .map(|s| {
                    resources
                        .iter()
                        .find(|r| r.identifier.as_ref().map(|i| i == &s).unwrap_or(false))
                        .unwrap()
                })
                .collect();

            resources
        })
        .collect();

    resources_tuples
}
