use std::collections::HashMap;

pub fn is_false(b: &bool) -> bool {
    !b
}

pub fn get_query(key: &str, url: &url::Url) -> Option<String> {
    let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();

    hash_query.get(key).map(|s| String::from(s))
}
