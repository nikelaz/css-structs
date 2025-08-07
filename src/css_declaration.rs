use nom::{
  bytes::complete::{is_not, take_while1},
  character::complete::{char, multispace0},
  combinator::{opt},
  sequence::{delimited, preceded, separated_pair},
  IResult,
  Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CSSDeclaration {
  pub name: String,
  pub value: String,
  pub important: bool,
}

impl CSSDeclaration {
  pub fn new(name: &str, value: &str, important: bool) -> Self {
    CSSDeclaration {
      name: name.to_string(),
      value: value.to_string(),
      important,
    }
  }

  fn parse_identifier(input: &str) -> IResult<&str, String> {
    take_while1(|c: char| c.is_alphanumeric() || c == '-')
      .map(|s: &str| s.to_string())
      .parse(input)
  }

  fn parse_value(input: &str) -> IResult<&str, (String, bool)> {
    let (input, raw_value) = is_not(";{}")
      .map(|s: &str| s.trim().to_string())
      .parse(input)?;
    
    // Check if the value ends with !important
    if raw_value.ends_with("!important") {
      let value = raw_value[..raw_value.len() - "!important".len()].trim().to_string();
      Ok((input, (value, true)))
    } else {
      Ok((input, (raw_value, false)))
    }
  }

  pub fn from_string(input: &str) -> IResult<&str, CSSDeclaration> {
    let (input, (name, (value, important))) = separated_pair(
      preceded(multispace0, Self::parse_identifier),
      delimited(multispace0, char(':'), multispace0),
      Self::parse_value,
    ).parse(input)?;

    let (input, _) = opt(char(';')).parse(input)?;

    Ok((input, CSSDeclaration { name, value, important }))
  }

  pub fn to_string(&self) -> String {
    if self.important {
      format!("{}: {} !important;", self.name, self.value)
    } else {
      format!("{}: {};", self.name, self.value)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let decl = CSSDeclaration::new("x", "y", false);
    assert_eq!(decl.name, "x");
    assert_eq!(decl.value, "y");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_from_string_simple() {
    let (_, decl) = CSSDeclaration::from_string("color: red;").unwrap();
    assert_eq!(decl.name, "color");
    assert_eq!(decl.value, "red");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_from_string_values_with_whitespace() {
    let (_, decl) = CSSDeclaration::from_string("border: 1px solid red;").unwrap();
    assert_eq!(decl.name, "border");
    assert_eq!(decl.value, "1px solid red");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_from_string_no_semi() {
    let (_, decl) = CSSDeclaration::from_string("color: red").unwrap();
    assert_eq!(decl.name, "color");
    assert_eq!(decl.value, "red");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_from_string_numeric_val() {
    let (_, decl) = CSSDeclaration::from_string("padding: 10px").unwrap();
    assert_eq!(decl.name, "padding");
    assert_eq!(decl.value, "10px");
    assert_eq!(decl.important, false);
  }


  #[test]
  fn test_from_string_prefix() {
    let (_, decl) = CSSDeclaration::from_string("-webkit-transition: .2s all").unwrap();
    assert_eq!(decl.name, "-webkit-transition");
    assert_eq!(decl.value, ".2s all");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_to_string() {
    let (_, decl) = CSSDeclaration::from_string("color: red;").unwrap();
    let decl_str = decl.to_string();
    assert_eq!(decl_str, "color: red;");
  }

  #[test]
  fn test_from_string_with_important() {
    let (_, decl) = CSSDeclaration::from_string("color: red !important;").unwrap();
    assert_eq!(decl.name, "color");
    assert_eq!(decl.value, "red");
    assert_eq!(decl.important, true);
  }

  #[test]
  fn test_from_string_with_important_no_semi() {
    let (_, decl) = CSSDeclaration::from_string("color: red !important").unwrap();
    assert_eq!(decl.name, "color");
    assert_eq!(decl.value, "red");
    assert_eq!(decl.important, true);
  }

  #[test]
  fn test_from_string_important_with_whitespace() {
    let (_, decl) = CSSDeclaration::from_string("border: 1px solid red !important;").unwrap();
    assert_eq!(decl.name, "border");
    assert_eq!(decl.value, "1px solid red");
    assert_eq!(decl.important, true);
  }

  #[test]
  fn test_to_string_with_important() {
    let decl = CSSDeclaration::new("color", "red", true);
    let decl_str = decl.to_string();
    assert_eq!(decl_str, "color: red !important;");
  }

  #[test]
  fn test_new_with_important() {
    let decl = CSSDeclaration::new("font-size", "14px", true);
    assert_eq!(decl.name, "font-size");
    assert_eq!(decl.value, "14px");
    assert_eq!(decl.important, true);
  }
}
