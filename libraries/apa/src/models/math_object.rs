pub struct MathObject {
  pub abs_no_dot: String,
  pub dot_pos: Option<usize>,
  pub is_negative: bool,
}

impl MathObject {
  pub fn new(abs_no_dot: String, dot_pos: Option<usize>, is_negative: bool) -> MathObject {
    MathObject {
      abs_no_dot,
      dot_pos,
      is_negative
    }
  }

  pub fn from_str(value: &str) -> MathObject {
    let mut result = MathObject::new("0".to_string(), Some(0), false);
    result.is_negative = value.starts_with("-");
    if result.is_negative {
      result.abs_no_dot = value[1..].parse().unwrap();
    } else {
      result.abs_no_dot = value[0..].parse().unwrap();
    }

    result.dot_pos = result.abs_no_dot.find(".");
    if result.dot_pos.is_some() {
      result.abs_no_dot.remove(result.dot_pos.unwrap());
    }

    return result;
  }

  pub fn to_string(&self) -> String {
    let mut result = String::new();
    if self.is_negative {
      result.push('-');
    }

    let mut abs = self.abs_no_dot.clone();
    if self.dot_pos.is_some() {
      abs.insert(self.dot_pos.unwrap(), '.');
    }

    result += &*abs;

    return result;
  }
}
