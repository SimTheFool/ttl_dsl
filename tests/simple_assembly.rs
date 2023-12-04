#[macro_use]
mod utils;
use custom_dsl::domain::resource::ResolvedResources;
use utils::*;

const INDEX: &str = r#"
    {
        stats: {
            << ./stats
                with con : 5
                with vol : 3
            << ./magician
                with mag : 4
                with initiation : 1
                with trad : "vol"
        }
    }
    "#;

const STATS: &str = r#"
    {
        con: con
        vol: vol
        ["con"]
        resist_phy: 0
        ["vol"]
        resist_ment: 0
        hit: 8
        ["vol" "con"]
        heal: 0
    }
    
    @TRANSFORM FINAL_STATS
    > $.resist_phy += $.con
    > $.resist_ment += $.vol
    > $.hit += floor($.con / 2)
    > $.heal += $.con + $.vol
    "#;

const MAGICIAN: &str = r#"
    {
        mag: mag
        initiation: initiation
        [trad]
        resist_drain: 0
    }
    
    @TRANSFORM FINAL_STATS_END
    > $.resist_drain += floor($.$trad / 2)
    "#;

#[test]
fn it_shoud_assemble_from_different_files() {
    let (app, resolver, config) = MockedApp::new();

    config.borrow_mut().add_layer("FINAL_STATS");
    config.borrow_mut().add_layer("FINAL_STATS_END");
    resolver.borrow_mut().mock_file("./stats", STATS);
    resolver.borrow_mut().mock_file("./magician", MAGICIAN);

    let resolved_resources = app.assemble_from_str(INDEX);
    let resolved_resources = unwrap_or_print_error!(resolved_resources);

    assert_eq!(resolved_resources.len(), 9);

    match resolved_resources.get(0).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 5.0);
            assert_eq!(nb.identifier, Some("con".to_string()));
        }
        _ => panic!("Expected a number"),
    }

    match resolved_resources.get(1).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 3.0);
            assert_eq!(nb.identifier, Some("vol".to_string()));
        }
        _ => panic!("Expected a number"),
    }

    match resolved_resources.get(2).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 5.0);
            assert_eq!(nb.identifier, Some("resist_phy".to_string()));
        }
        _ => panic!("Expected a number"),
    }

    match resolved_resources.get(3).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 3.0);
            assert_eq!(nb.identifier, Some("resist_ment".to_string()));
        }
        _ => panic!("Expected a number"),
    }

    match resolved_resources.get(4).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 10.0);
            assert_eq!(nb.identifier, Some("hit".to_string()));
        }
        _ => panic!("Expected a number"),
    }

    match resolved_resources.get(5).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 8.0);
            assert_eq!(nb.identifier, Some("heal".to_string()));
        }
        _ => panic!("Expected a number"),
    }

    match resolved_resources.get(6).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 4.0);
            assert_eq!(nb.identifier, Some("mag".to_string()));
        }
        _ => panic!("Expected a number"),
    }

    match resolved_resources.get(7).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 1.0);
            assert_eq!(nb.identifier, Some("initiation".to_string()));
        }
        _ => panic!("Expected a number"),
    }

    match resolved_resources.get(8).unwrap() {
        ResolvedResources::Number(nb) => {
            assert_eq!(nb.value, 1.0);
            assert_eq!(nb.identifier, Some("resist_drain".to_string()));
        }
        _ => panic!("Expected a number"),
    }
}
