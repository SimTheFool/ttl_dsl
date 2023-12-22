#[macro_use]
mod utils;
use crate::utils::*;
use custom_dsl::domain::resolution::ResolvedResourceValue;
use regex::Regex;

#[test]
fn it_should_create_resources_only() {
    let (app, _, _) = MockedApp::new();
    let index = r#"{
        var05: hello, I'm some text !!
        var06: {
            var07: 07
            var08: 08
        }
    }"#;

    let values = app.assemble_from_str(index);
    let values = print_unwrap!(values);

    assert_eq!(values.len(), 3);

    let first_ressource = values.get(0).unwrap();
    assert_resource!(first_ressource: "var05", String "hello, I'm some text !!");

    let second_ressource = values.get(1).unwrap();
    assert_resource!(second_ressource: "var06.var07", Number 7.0);

    let third_ressource = values.get(2).unwrap();
    assert_resource!(third_ressource: "var06.var08", Number 8.0);
}

#[test]
fn it_should_create_resources_with_import() {
    let (app, resolver, _) = MockedApp::new();
    let index = r#"{
            <? ./stats
                with var01 : 001
                with var02 : "002" >
            var03: 003
        }"#;

    resolver.borrow_mut().mock_file(
        "./stats",
        r#"{
            somevar01: $var01
            somevar02: $var02
            [$var01 $var02]
            someothervar: "statistics"
        }"#,
    );
    let values = app.assemble_from_str(index);
    let values = print_unwrap!(values);

    assert_eq!(values.len(), 4);

    let first_ressource = values.get(0).unwrap();
    assert_resource!(first_ressource: "somevar01", Number 1.0);

    let second_ressource = values.get(1).unwrap();
    assert_resource!(second_ressource: "somevar02", String "002");

    let third_ressource = values.get(2).unwrap();
    assert_resource!(third_ressource: "someothervar", String "statistics");

    let fourth_ressource = values.get(3).unwrap();
    assert_resource!(fourth_ressource: "var03", Number 3.0);

    let third_resources_metas = &third_ressource.metas;
    assert_eq!(third_resources_metas.len(), 2);

    let first_meta = third_resources_metas.get(0).unwrap();
    assert_eq!(first_meta.identifier, None);
    let meta_value = as_variant!(&first_meta.value, ResolvedResourceValue::Number);
    assert_eq!(meta_value, &1.0);

    let second_meta = third_resources_metas.get(1).unwrap();
    assert_eq!(second_meta.identifier, None);
    let meta_value = as_variant!(&second_meta.value, ResolvedResourceValue::String);
    assert_eq!(meta_value, "002");
}

#[test]
fn it_should_create_resources_with_transforms() {
    let (app, _, config) = MockedApp::new();
    let index = r#"
        {
            x: 5
        }

        @TRANSFORM SECOND_LAYER
        > x += 3
        > x *= 3

        @TRANSFORM FIRST_LAYER
        > x *= 2
        > x += 2

        "#;

    config.borrow_mut().add_layer("FIRST_LAYER");
    config.borrow_mut().add_layer("SECOND_LAYER");

    let values = app.assemble_from_str(index);
    let values = print_unwrap!(values);

    assert_eq!(values.len(), 1);

    let first_ressource = values.get(0).unwrap();
    assert_resource!(first_ressource: "x", Number 45.0);
}

#[test]
fn it_should_assemble_nested_import() {
    let (app, resolver, _) = MockedApp::new();
    let index = r#"{
        <? ./stats
            with <? ./mag >
        >
    }"#;

    resolver.borrow_mut().mock_file(
        "./stats",
        r#"{
        con: 1
        vol: 2
    }"#,
    );
    resolver.borrow_mut().mock_file(
        "./mag",
        r#"{
        mag: 3
    }"#,
    );

    let values = app.assemble_from_str(index);
    let values = print_unwrap!(values);

    assert_eq!(values.len(), 3);

    let first_ressource = values.get(0).unwrap();
    assert_resource!(first_ressource: "con", Number 1.0);

    let second_ressource = values.get(1).unwrap();
    assert_resource!(second_ressource: "vol", Number 2.0);

    let third_ressource = values.get(2).unwrap();
    assert_resource!(third_ressource: "mag", Number 3.0);
}

#[test]
fn it_should_assemble_named_imports() {
    let (app, resolver, _) = MockedApp::new();
    let index = r#"{
        <@ ./stats >
    }"#;

    resolver.borrow_mut().mock_file(
        "./stats",
        r#"
    @NAME stats
    {
        con: 1
        vol: 2
    }"#,
    );

    let values = app.assemble_from_str(index);
    let values = print_unwrap!(values);

    assert_eq!(values.len(), 2);

    let first_ressource = values.get(0).unwrap();
    assert_resource!(first_ressource: "stats.con", Number 1.0);

    let second_ressource = values.get(1).unwrap();
    assert_resource!(second_ressource: "stats.vol", Number 2.0);
}

#[test]
fn it_should_assemble_uniq_imports() {
    let (app, resolver, _) = MockedApp::new();
    let index = r#"{
        <! ./stats >
        <! ./stats >
    }"#;

    resolver.borrow_mut().mock_file(
        "./stats",
        r#"
    @NAME stats
    {
        con: 1
    }"#,
    );

    let values = app.assemble_from_str(index);
    let values = print_unwrap!(values);

    assert_eq!(values.len(), 2);
    let key_regex = Regex::new(r"^([a-zA-Z0-9]+)\.con$").unwrap();

    let first_ressource = values.get(0).unwrap();
    let match_group_first = key_regex
        .captures(first_ressource.identifier.as_ref().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let value = as_variant!(&first_ressource.value, ResolvedResourceValue::Number);
    assert_eq!(value, &1.0);

    let second_ressource = values.get(1).unwrap();
    let match_group_second = key_regex
        .captures(second_ressource.identifier.as_ref().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let value = as_variant!(&second_ressource.value, ResolvedResourceValue::Number);
    assert_eq!(value, &1.0);

    assert_ne!(match_group_first, match_group_second);
}

#[test]
fn it_should_handle_uniq_declaration() {
    let (app, _, _) = MockedApp::new();
    let index = r#"{
        con! 1
        con! 2
        con:
        {
            var10: I'm a subvalue
        }
    }"#;
    let values = app.assemble_from_str(index);
    let values = print_unwrap!(values);

    assert_eq!(values.len(), 3);

    let key_regex = Regex::new(r"^con\.([a-zA-Z0-9]+)$").unwrap();

    let first_ressource = values.get(0).unwrap();
    let match_group_first = key_regex
        .captures(first_ressource.identifier.as_ref().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let value = as_variant!(&first_ressource.value, ResolvedResourceValue::Number);
    assert_eq!(value, &1.0);

    let second_ressource = values.get(1).unwrap();
    let match_group_second = key_regex
        .captures(second_ressource.identifier.as_ref().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let value = as_variant!(&second_ressource.value, ResolvedResourceValue::Number);
    assert_eq!(value, &2.0);

    assert_ne!(match_group_first, match_group_second);

    let third_ressource = values.get(2).unwrap();
    assert_resource!(third_ressource: "con.var10", String "I'm a subvalue");
}
