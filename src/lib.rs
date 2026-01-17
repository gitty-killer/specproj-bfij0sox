pub fn count_words(input: &str) -> usize {
    input.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::count_words;

    #[test]
    fn counts() {
        assert_eq!(count_words("a b c"), 3);
    }
}
