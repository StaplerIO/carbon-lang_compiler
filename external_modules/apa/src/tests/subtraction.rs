mod tests {
    pub use crate::apa::subtraction::subtract;

    #[test]
    fn simple_subtraction() {
        assert_eq!(
            subtract("456".to_string(), "123".to_string()),
            "333".to_string()
        );
    }
}
