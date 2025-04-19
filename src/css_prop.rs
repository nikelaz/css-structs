#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct CSSProp {
  pub name: String,
  pub value: String
}

impl CSSProp {
  pub fn to_string(&self) -> String {
    return format!("{}: {};", self.name, self.value);
  }
}
