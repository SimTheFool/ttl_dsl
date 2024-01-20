use crate::{
    domain::resolution::{ResolvedResource, ResolvedResourceValue},
    result::{AppError, AppResult},
};

pub type JSONValue = serde_json::Value;

#[derive(Debug)]
pub enum KeyType<'a> {
    Obj(&'a str),
    Null,
}

pub fn get_keytypes(identifier: Option<&str>) -> Vec<KeyType> {
    match identifier {
        None => vec![KeyType::Null],
        Some(identifier) => identifier
            .split('.')
            .map(KeyType::Obj)
            .collect::<Vec<KeyType>>(),
    }
}

pub fn get_json_entry_for_key_type<'a>(
    json: &'a mut JSONValue,
    key_type: KeyType,
) -> AppResult<&'a mut JSONValue> {
    let entry = match (&json, &key_type) {
        (JSONValue::Null, KeyType::Obj(key)) => {
            let mut new_map = serde_json::Map::new();
            new_map.insert(key.to_string(), JSONValue::Null);
            *json = JSONValue::Object(new_map);
            json.get_mut(key).unwrap()
        }
        (JSONValue::Object(_), KeyType::Obj(key)) => {
            json.as_object_mut()
                .unwrap()
                .entry(key.to_string())
                .or_insert_with(|| JSONValue::Null);
            json.get_mut(key).unwrap()
        }
        _ => Err(AppError::String(format!(
            "Cannot find a valid entry for {:?} in {:?}",
            key_type, json
        )))?,
    };

    Ok(entry)
}

pub fn get_json_entry_for_keys<'a>(
    json: &'a mut JSONValue,
    key_types: Vec<KeyType>,
) -> AppResult<&'a mut JSONValue> {
    key_types
        .into_iter()
        .try_fold(json, |json_entry, key_type| {
            let json_entry = get_json_entry_for_key_type(json_entry, key_type)?;
            Ok(json_entry)
        })
}

pub fn get_json_value(resource: ResolvedResource) -> AppResult<JSONValue> {
    let ResolvedResource { value, metas, .. } = resource;
    let value = match value {
        ResolvedResourceValue::Null => JSONValue::Null,
        ResolvedResourceValue::String(value) => JSONValue::String(value),
        ResolvedResourceValue::Number(value) => JSONValue::Number(
            serde_json::Number::from_f64(value)
                .ok_or_else(|| AppError::Str("Failed to convert number to JSON number"))?,
        ),
    };

    match &metas.len() {
        0 => AppResult::Ok(value),
        _ => {
            let metas: Vec<JSONValue> = metas
                .into_iter()
                .map(|ResolvedResource { value, .. }| JSONValue::String(value.to_string()))
                .collect();
            let json_resource = serde_json::json!({
                "value": value,
                "metas": metas
            });

            AppResult::Ok(json_resource)
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    #[test]
    fn it_should_get_mutable_entry_for_key_type() {
        let mut json = serde_json::json!({
            "a": 10,
            "b": "hello"
        });

        let entry =
            super::get_json_entry_for_key_type(&mut json, super::KeyType::Obj("a")).unwrap();

        let expected_number = serde_json::Number::from_str("10").unwrap();
        match entry {
            serde_json::Value::Number(x) if x == &expected_number => {}
            _ => panic!("Expected a number, got {:?}", entry),
        }

        let entry =
            super::get_json_entry_for_key_type(&mut json, super::KeyType::Obj("c")).unwrap();

        match entry {
            serde_json::Value::Null => {}
            _ => panic!("Expected null, got {:?}", entry),
        }
    }

    #[test]
    fn it_should_create_mutable_entries_if_null_entry() {
        let mut json = serde_json::json!({
            "a": null,
            "b": "hello"
        });

        let key_types = super::get_keytypes(Some("a.c.d"));

        let entry = super::get_json_entry_for_keys(&mut json, key_types).unwrap();
        let value_to_insert = serde_json::json!(20);
        *entry = value_to_insert;

        let expected_json = serde_json::json!({
            "a": {
                "c": {
                    "d": 20
                }
            },
            "b": "hello",
        });

        assert_eq!(json, expected_json);
    }
}
