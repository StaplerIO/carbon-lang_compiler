mod tests {
    pub use crate::apa::modulo::modulo;

    #[test]
    fn simple_modulo() {
        assert_eq!(modulo("56".to_string(), "3".to_string()), "2".to_string());
        assert_eq!(modulo("12345".to_string(), "2".to_string()), "1".to_string());
        assert_eq!(modulo("1234590234567".to_string(), "2021".to_string()), "339".to_string());
    }
}
