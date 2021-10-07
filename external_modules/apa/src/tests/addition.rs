mod tests {
    pub use crate::apa::addition::add;

    #[test]
    fn simple_addition() {
        assert_eq!(add("123".to_string(), "456".to_string()), "579".to_string());
        assert_eq!(add("12349022707077710234050034561786".to_string(), "45622".to_string()), "12349022707077710234050034607408".to_string());
        assert_eq!(add("999".to_string(), "999".to_string()), "1998".to_string());
    }
}
