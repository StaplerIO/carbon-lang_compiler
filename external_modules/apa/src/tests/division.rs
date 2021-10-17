mod tests {
    pub use crate::apa::division::divide;

    #[test]
    fn simple_division() {
        assert_eq!(divide("56".to_string(), "8".to_string(), 0), "7".to_string());
        assert_eq!(divide("38888888".to_string(), "2".to_string(), 0), "19444444".to_string());
    }
}
