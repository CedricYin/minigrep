pub(in super) fn search_is_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub(in super) fn search_not_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| { line.to_lowercase().contains(&query.to_lowercase()) })
        .collect()
}

#[cfg(test)]
mod test_search {
    use super::*;
    #[test]
    fn test_search_is_case() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct haha";

        assert_eq!(vec!["safe, fast, productive."], search_is_case(query, contents));
    }

    #[test]
    fn test_search_not_case() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct haha";

        assert_eq!(vec!["safe, fast, productive.", "Duct haha"], search_not_case(query, contents));
    }
}