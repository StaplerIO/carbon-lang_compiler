use crate::models::math_object::MathObject;

#[test]
fn self_test_a() {
    let model = MathObject::from_str("-1.23356");
    assert_eq!(model.to_string(), "-1.23356".to_string());
}

#[test]
fn self_test_b() {
    let model = MathObject::from_str("12278");
    assert_eq!(model.to_string(), "12278".to_string());
}
