use nom::{
  character::complete::{char, multispace0},
  combinator::opt,
  multi::many0,
  sequence::delimited,
  IResult,
  Parser, 
};

use crate::css_declaration::CSSDeclaration;

#[derive(Debug, Clone, PartialEq)]
pub struct CSSDeclarationList {
  pub declarations: Vec<CSSDeclaration>,
}

impl CSSDeclarationList {
  fn parse_declarations(input: &str) -> IResult<&str, Vec<CSSDeclaration>> {
    many0(
      delimited(
        multispace0,
        CSSDeclaration::parse,
        opt(char(';')),
      )
    )
      .parse(input)
  }

  pub fn from_string(css_block: &str) -> Result<Self, String> {
    let (_, declarations) = Self::parse_declarations(css_block)
      .map_err(|e| e.to_string())?;

    Ok(Self { declarations })
  }

  pub fn from_str_parser(input: &str) -> IResult<&str, CSSDeclarationList> {
    let (input, declarations) = Self::parse_declarations(input)?;

    Ok((input, CSSDeclarationList { declarations }))
  }

  pub fn remove_declaration(&mut self, decl_name: &str) {
    self.declarations.retain(|decl| decl.name != decl_name);
  }

  pub fn to_string(&self) -> String {
    self.declarations
      .iter()
      .map(|decl| decl.to_string())
      .collect::<Vec<_>>()
      .join(" ")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::css_declaration::CSSDeclaration;

  #[test]
  fn test_single() {
    let input = "color: red;";
    let list = CSSDeclarationList::from_string(input).unwrap();
    assert_eq!(list.declarations.len(), 1);
    assert_eq!(list.declarations[0], CSSDeclaration::new("color", "red", None));
  }

  #[test]
  fn test_multiple() {
    let input = "color: red; background-color: blue; padding: 10px;";
    let list = CSSDeclarationList::from_string(input).unwrap();
    assert_eq!(list.declarations.len(), 3);
    assert_eq!(list.declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(list.declarations[1], CSSDeclaration::new("background-color", "blue", None));
    assert_eq!(list.declarations[2], CSSDeclaration::new("padding", "10px", None));
  }

  #[test]
  fn test_extra_whitespace() {
    let input = "  margin :  0 auto  ;  padding :  1em ;  ";
    let list = CSSDeclarationList::from_string(input).unwrap();
    assert_eq!(list.declarations.len(), 2);
    assert_eq!(list.declarations[0], CSSDeclaration::new("margin", "0 auto", None));
    assert_eq!(list.declarations[1], CSSDeclaration::new("padding", "1em", None));
  }

  #[test]
  fn test_no_trailing_semicolon() {
    let input = "font-size: 16px; line-height: 1.5";
    let list = CSSDeclarationList::from_string(input).unwrap();
    assert_eq!(list.declarations.len(), 2);
    assert_eq!(list.declarations[0], CSSDeclaration::new("font-size", "16px", None));
    assert_eq!(list.declarations[1], CSSDeclaration::new("line-height", "1.5", None));
  }

  #[test]
  fn test_empty_input() {
    let input = "";
    let list = CSSDeclarationList::from_string(input).unwrap();
    assert_eq!(list.declarations.len(), 0);
  }

  #[test]
  fn test_to_string_output() {
    let input = "color: red; padding: 10px;";
    let list = CSSDeclarationList::from_string(input).unwrap();
    let output = list.to_string();
    assert_eq!(output, "color: red; padding: 10px;");
  }

  #[test]
  fn test_remove_declaration() {
    let input = "color: red; padding: 10px;";
    let mut list = CSSDeclarationList::from_string(input).unwrap();
    list.remove_declaration("color");
    assert_eq!(list.declarations.len(), 1);
    assert_eq!(list.declarations[0], CSSDeclaration::new("padding", "10px", None));
  }
}
