mod tests {
    use crate::apa::multiplication::multiply;

    #[test]
    fn simple_multiplication() {
        assert_eq!(multiply("123".to_string(), "456".to_string()), "56088".to_string());
        assert_eq!(multiply("123456".to_string(), "789012".to_string()), "97408265472".to_string());
    }
}

