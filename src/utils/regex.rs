use regex::{Captures, Regex};

pub fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

#[cfg(test)]
mod test {
    use crate::{
        regex::replace_all,
        result::{AppError, AppResult},
    };
    use regex::{Captures, Regex};
    use std::collections::HashMap;

    #[test]
    fn should_get_slices_from_regex() {
        let (regex1, regex2) = (
            Regex::new(r"\$\{(.+)\}").expect("Invalid regular expression"),
            Regex::new(r"\$(\p{L}+)").expect("Invalid regular expression"),
        );

        let mut variables = HashMap::<String, String>::new();
        variables.insert("world".to_string(), "everyone".to_string());
        variables.insert("something new".to_string(), "something old".to_string());

        let replacer = |captures: &Captures| -> AppResult<String> {
            let full = captures.get(0).map(|g| g.as_str()).unwrap();
            let group = captures.get(1).map(|g| g.as_str());

            let var_value = match group {
                None => Err(AppError::String(format!("No group found for {}", full)))?,
                Some(group) => {
                    let var_value = variables.get(&group.to_string()).ok_or_else(|| {
                        AppError::String(format!("Variable not found: {}", group.to_string()))
                    })?;

                    var_value.to_string()
                }
            };

            Ok(var_value)
        };

        let string = "hello $world, ${something new}";
        let replaced = replace_all(&regex1, string, replacer).unwrap();
        assert_eq!(replaced, "hello $world, something old");
        let replaced = replace_all(&regex2, &replaced, replacer);
        assert_eq!(replaced.unwrap(), "hello everyone, something old");
    }
}
