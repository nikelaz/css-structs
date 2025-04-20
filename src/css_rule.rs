use nom::{
  IResult,
  bytes::complete::take_until,
  character::complete::{char, multispace0},
  sequence::{terminated, delimited},
  Parser,
};

use crate::css_declaration_list::CSSDeclarationList;

#[derive(Debug, Clone, PartialEq)]
pub struct CSSRule {
  pub selector: String,
  pub declarations: CSSDeclarationList,
}

impl CSSRule {
  pub fn new(selector: &str, declarations: &CSSDeclarationList) -> Self {
    CSSRule {
      selector: selector.to_string(),
      declarations: declarations.clone(),
    }
  }

  pub fn from_string(input: &str) -> IResult<&str, CSSRule> {
    let (input, selector) = terminated(take_until("{"), char('{')).parse(input)?;

    let (input, declarations) = terminated(
      delimited(
        multispace0,
        CSSDeclarationList::from_str_parser,
        multispace0
      ),
      char('}')
    ).parse(input)?;

    Ok((
      input,
      CSSRule {
        selector: selector.trim().to_string(),
        declarations,
      },
    ))
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::css_declaration::CSSDeclaration;
  use crate::css_declaration_list::CSSDeclarationList;

  #[test]
  fn test_basic_rule() {
    let input = "h1 { color: red; padding: 10px; }";
    let (_, rule) = CSSRule::from_string(input).unwrap();
    assert_eq!(rule.selector, "h1");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("color", "red"));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("padding", "10px"));
  }

  #[test]
  fn test_rule_with_whitespace() {
    let input = "  div.my-class   {  margin : 0 auto ;  padding : 1em ; }";
    let (_, rule) = CSSRule::from_string(input).unwrap();
    assert_eq!(rule.selector, "div.my-class");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("margin", "0 auto"));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("padding", "1em"));
  }

  #[test]
  fn test_rule_no_trailing_semicolon() {
    let input = "p { font-size: 16px; line-height: 1.5 }";
    let (_, rule) = CSSRule::from_string(input).unwrap();
    assert_eq!(rule.selector, "p");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("font-size", "16px"));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("line-height", "1.5"));
  }

  #[test]
  fn test_empty_declarations() {
    let input = ".empty { }";
    let (_, rule) = CSSRule::from_string(input).unwrap();
    assert_eq!(rule.selector, ".empty");
    assert!(rule.declarations.declarations.is_empty());
  }

  #[test]
  fn test_rule_with_newlines() {
    let input = r#"
            .box {
                border: 1px solid black;
                background: white;
            }
        "#;
    let (_, rule) = CSSRule::from_string(input).unwrap();
    assert_eq!(rule.selector.trim(), ".box");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("border", "1px solid black"));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("background", "white"));
  }

  #[test]
  fn test_rule_with_multiple_selectors() {
    let input = "h1, h2, h3 { font-weight: bold; }";
    let (_, rule) = CSSRule::from_string(input).unwrap();
    assert_eq!(rule.selector, "h1, h2, h3");
    assert_eq!(rule.declarations.declarations.len(), 1);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("font-weight", "bold"));
  }
}
