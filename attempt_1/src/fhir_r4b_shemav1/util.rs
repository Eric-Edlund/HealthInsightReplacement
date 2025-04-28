
pub fn join_name<T>(name: &T) -> String
where
    T: AsRef<Vec<Option<String>>>,
{
    let parts: Vec<String> = name
        .as_ref()
        .iter()
        .filter_map::<String, _>(|part| part.clone())
        .collect();

    parts.join(" ")
}


pub fn double_unwrap<T>(it: &[Option<T>]) -> Vec<T>
where
    T: Clone,
{
    it.iter().filter_map(|it| it.clone()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_names() {
        assert_eq!(
            join_name(&vec![None, Some("One".into()), None, Some("Two".into())]),
            "One Two".to_string(),
        )
    }
}
