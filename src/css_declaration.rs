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
}

impl CSSDeclaration {
  pub fn new(name: &str, value: &str) -> Self {
    CSSDeclaration {
      name: name.to_string(),
      value: value.to_string(),
    }
  }

  fn parse_identifier(input: &str) -> IResult<&str, String> {
    take_while1(|c: char| c.is_alphanumeric() || c == '-')
      .map(|s: &str| s.to_string())
      .parse(input)
  }

  fn parse_value(input: &str) -> IResult<&str, String> {
    is_not(";{}")
      .map(|s: &str| s.trim().to_string())
      .parse(input)
  }

  pub fn from_string(input: &str) -> IResult<&str, CSSDeclaration> {
    let (input, (name, value)) = separated_pair(
      preceded(multispace0, Self::parse_identifier),
      delimited(multispace0, char(':'), multispace0),
      Self::parse_value,
    ).parse(input)?;

    let (input, _) = opt(char(';')).parse(input)?;

    Ok((input, CSSDeclaration { name, value }))
  }

  pub fn to_string(&self) -> String {
    format!("{}: {};", self.name, self.value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let decl = CSSDeclaration::new("x", "y");
    assert_eq!(decl.name, "x");
    assert_eq!(decl.value, "y");
  }

  #[test]
  fn test_from_string_simple() {
    let (_, decl) = CSSDeclaration::from_string("color: red;").unwrap();
    assert_eq!(decl.name, "color");
    assert_eq!(decl.value, "red");
  }

  #[test]
  fn test_from_string_values_with_whitespace() {
    let (_, decl) = CSSDeclaration::from_string("border: 1px solid red;").unwrap();
    assert_eq!(decl.name, "border");
    assert_eq!(decl.value, "1px solid red");
  }

  #[test]
  fn test_from_string_no_semi() {
    let (_, decl) = CSSDeclaration::from_string("color: red").unwrap();
    assert_eq!(decl.name, "color");
    assert_eq!(decl.value, "red");
  }

  #[test]
  fn test_from_string_numeric_val() {
    let (_, decl) = CSSDeclaration::from_string("padding: 10px").unwrap();
    assert_eq!(decl.name, "padding");
    assert_eq!(decl.value, "10px");
  }


  #[test]
  fn test_from_string_prefix() {
    let (_, decl) = CSSDeclaration::from_string("-webkit-transition: .2s all").unwrap();
    assert_eq!(decl.name, "-webkit-transition");
    assert_eq!(decl.value, ".2s all");
  }

  #[test]
  fn test_to_string() {
    let (_, decl) = CSSDeclaration::from_string("color: red;").unwrap();
    let decl_str = decl.to_string();
    assert_eq!(decl_str, "color: red;");
  }
}
