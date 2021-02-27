#[cfg(test)]
mod tests {
    #[test]
    fn no_comment() {
        let result = crate::lexer::preprocess::remove_comments(String::from("func main() -> int { return 0; }"));

        assert_eq!(result, String::from("func main() -> int { return 0; }"));
    }

    #[test]
    fn full_comment() {
        let result = crate::lexer::preprocess::remove_comments(String::from("#asas22##t1#"));
        assert_eq!(result, String::from(""));
    }

    #[test]
    fn complex_comment() {
        let result = crate::lexer::preprocess::remove_comments(String::from("#asas22# \"car#bo#n\" #t1# func main(\'#\')"));
        assert_eq!(result, String::from(" \"car#bo#n\"  func main(\'#\')"));
    }
}
