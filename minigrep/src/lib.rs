pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>  {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "dist";
        let contents = "\
Artemis just went to the moon.
Beat apollo 13's distance travelled from earth.
Artemis is cool!";
        assert_eq!(vec!["Beat apollo 13's distance travelled from earth."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "CATS";
        let contents = "\
CATS ARE REALLY NICE!
but they're mean sometimes, these cats.
I love them though!
        ";
        assert_eq!(vec!["CATS ARE REALLY NICE!"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "CATS";
        let contents = "\
CATS ARE REALLY NICE!
but they're mean sometimes, these cats.
I love them though!
        ";
        assert_eq!(vec!["CATS ARE REALLY NICE!", "but they're mean sometimes, these cats."], search_case_insensitive(query, contents));
    }
}