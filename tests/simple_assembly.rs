#[macro_use]
mod utils;
use custom_dsl::domain::resolution::ResolvedResourceValue;
use utils::*;

const INDEX: &str = r#"
    {
        stats:
        {
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

    let resource_0 = resolved_resources.get(0).unwrap();
    assert_eq!(resource_0.identifier, Some("con".to_string()));
    match &resource_0.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &5.0);
        }
        _ => panic!("Should be a number"),
    }

    let resource_1 = resolved_resources.get(1).unwrap();
    assert_eq!(resource_1.identifier, Some("vol".to_string()));
    match &resource_1.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &3.0);
        }
        _ => panic!("Should be a number"),
    }

    let resource_2 = resolved_resources.get(2).unwrap();
    assert_eq!(resource_2.identifier, Some("resist_phy".to_string()));
    match &resource_2.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &5.0);
        }
        _ => panic!("Should be a number"),
    }

    let resource_3 = resolved_resources.get(3).unwrap();
    assert_eq!(resource_3.identifier, Some("resist_ment".to_string()));
    match &resource_3.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &3.0);
        }
        _ => panic!("Should be a number"),
    }

    let resource_4 = resolved_resources.get(4).unwrap();
    assert_eq!(resource_4.identifier, Some("hit".to_string()));
    match &resource_4.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &10.0);
        }
        _ => panic!("Should be a number"),
    }

    let resource_5 = resolved_resources.get(5).unwrap();
    assert_eq!(resource_5.identifier, Some("heal".to_string()));
    match &resource_5.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &8.0);
        }
        _ => panic!("Should be a number"),
    }

    let resource_6 = resolved_resources.get(6).unwrap();
    assert_eq!(resource_6.identifier, Some("mag".to_string()));
    match &resource_6.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &4.0);
        }
        _ => panic!("Should be a number"),
    }

    let resource_7 = resolved_resources.get(7).unwrap();
    assert_eq!(resource_7.identifier, Some("initiation".to_string()));
    match &resource_7.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &1.0);
        }
        _ => panic!("Should be a number"),
    }

    let resource_8 = resolved_resources.get(8).unwrap();
    assert_eq!(resource_8.identifier, Some("resist_drain".to_string()));
    match &resource_8.value {
        ResolvedResourceValue::Number(x) => {
            assert_eq!(x, &1.0);
        }
        _ => panic!("Should be a number"),
    }
    let metas = &resource_8.metas;
    assert_eq!(metas.len(), 1);
    let first_meta = metas.get(0).unwrap();
    match &first_meta.value {
        ResolvedResourceValue::String(x) => {
            assert_eq!(x, "vol");
        }
        _ => panic!("Should be a string"),
    }
}
