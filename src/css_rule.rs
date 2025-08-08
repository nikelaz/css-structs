//! CSS Rule Parser
//!
//! This module provides parsing and representation for complete CSS rules
//! (selector-declaration block pairs like `div { color: red; margin: 10px }` or 
//! `h1.title, h2.subtitle { font-weight: bold; padding: 1em }`).
//!
//! ## Main API
//! 
//! - `CSSRule::from_string()` - Parse a CSS rule from a string
//! - `CSSRule::new()` - Create a new rule programmatically  
//! - `Display` trait implementation for converting back to CSS string
//!
//! ## Examples
//!
//! ```rust
//! use css_structs::{CSSRule, CSSDeclarationList, CSSDeclaration};
//! 
//! // Parse from string
//! let rule = CSSRule::from_string("div.container { color: red; margin: 10px }").unwrap();
//! assert_eq!(rule.selector, "div.container");
//! assert_eq!(rule.declarations.declarations.len(), 2);
//!
//! // Create programmatically  
//! let mut declarations = CSSDeclarationList::from_string("padding: 1em").unwrap();
//! let rule = CSSRule::new("h1", &declarations);
//! println!("{}", rule); // "h1 { padding: 1em }"
//! ```


use std::fmt;
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
  fn parse_selector(input: &str) -> IResult<&str, String> {
    let (input, selector) = terminated(take_until("{"), char('{')).parse(input)?;

    Ok((input, selector.trim().to_string()))
  }

  fn parse_declarations_block(input: &str) -> IResult<&str, CSSDeclarationList> {
    let (input, declarations) = terminated(
      delimited(
        multispace0,
        CSSDeclarationList::parse,
        multispace0
      ),
      char('}')
    ).parse(input)?;

    Ok((input, declarations))
  }

  pub(crate) fn parse(input: &str) -> IResult<&str, CSSRule> {
    let (input, selector) = Self::parse_selector(input)?;
    let (input, declarations) = Self::parse_declarations_block(input)?; 

    Ok((
      input,
      CSSRule {
        selector,
        declarations,
      },
    ))
  }

  pub fn from_string(input: &str) -> Result<CSSRule, String> {
    let (_, css_rule) = Self::parse(input)
      .map_err(|_| "Failed to parse CSS rule".to_string())?;

    Ok(css_rule)
  }

  pub fn new(selector: &str, declarations: &CSSDeclarationList) -> Self {
    CSSRule {
      selector: selector.to_string(),
      declarations: declarations.clone(),
    }
  }
}

impl fmt::Display for CSSRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {{ {} }}", self.selector, self.declarations.to_string())
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::css_declaration::CSSDeclaration;


  #[test]
  fn test_parse_selector_simple_element() {
    let input = "div{color: red}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "color: red}");
    assert_eq!(selector, "div");
  }

  #[test]
  fn test_parse_selector_class() {
    let input = ".container{margin: 0}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "margin: 0}");
    assert_eq!(selector, ".container");
  }

  #[test]
  fn test_parse_selector_id() {
    let input = "#header{background: blue}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "background: blue}");
    assert_eq!(selector, "#header");
  }

  #[test]
  fn test_parse_selector_complex() {
    let input = "div.container > p:first-child{font-size: 16px}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "font-size: 16px}");
    assert_eq!(selector, "div.container > p:first-child");
  }

  #[test]
  fn test_parse_selector_with_spaces() {
    let input = "  div  {padding: 10px}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "padding: 10px}");
    assert_eq!(selector, "div");
  }

  #[test]
  fn test_parse_selector_attribute() {
    let input = "input[type=\"text\"]{border: 1px solid gray}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "border: 1px solid gray}");
    assert_eq!(selector, "input[type=\"text\"]");
  }

  #[test]
  fn test_parse_selector_pseudo_class() {
    let input = "a:hover{color: red}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "color: red}");
    assert_eq!(selector, "a:hover");
  }

  #[test]
  fn test_parse_selector_child_combinator() {
    let input = "div > p{margin-bottom: 1em}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "margin-bottom: 1em}");
    assert_eq!(selector, "div > p");
  }

  #[test]
  fn test_parse_selector_adjacent_sibling() {
    let input = "h1 + p{font-weight: bold}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "font-weight: bold}");
    assert_eq!(selector, "h1 + p");
  }

  #[test]
  fn test_parse_selector_general_sibling() {
    let input = "h1 ~ p{color: gray}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "color: gray}");
    assert_eq!(selector, "h1 ~ p");
  }

  #[test]
  fn test_parse_selector_empty_block() {
    let input = "div{}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "}");
    assert_eq!(selector, "div");
  }

  #[test]
  fn test_parse_selector_with_newlines() {
    let input = "\n  .container\n  {display: flex}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "display: flex}");
    assert_eq!(selector, ".container");
  }

  #[test]
  fn test_parse_selector_universal() {
    let input = "*{box-sizing: border-box}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "box-sizing: border-box}");
    assert_eq!(selector, "*");
  }

  #[test]
  fn test_parse_selector_fails_no_opening_brace() {
    let input = "div color: red";
    let result = CSSRule::parse_selector(input);

    assert!(result.is_err());
  }

  #[test]
  fn test_parse_selector_fails_empty_input() {
    let input = "";
    let result = CSSRule::parse_selector(input);

    assert!(result.is_err());
  }

  #[test]
  fn test_parse_selector_with_tabs() {
    let input = "\t.nav\t{position: fixed}";
    let (remaining, selector) = CSSRule::parse_selector(input).unwrap();

    assert_eq!(remaining, "position: fixed}");
    assert_eq!(selector, ".nav");
  }

  #[test]
  fn test_basic_rule() {
    let input = "h1 { color: red; padding: 10px; }";
    let (_, rule) = CSSRule::parse(input).unwrap();
    assert_eq!(rule.selector, "h1");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("padding", "10px", None));
  }

  #[test]
  fn test_rule_with_whitespace() {
    let input = "  div.my-class   {  margin : 0 auto ;  padding : 1em ; }";
    let (_, rule) = CSSRule::parse(input).unwrap();
    assert_eq!(rule.selector, "div.my-class");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("margin", "0 auto", None));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("padding", "1em", None));
  }

  #[test]
  fn test_rule_no_trailing_semicolon() {
    let input = "p { font-size: 16px; line-height: 1.5 }";
    let (_, rule) = CSSRule::parse(input).unwrap();
    assert_eq!(rule.selector, "p");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("font-size", "16px", None));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("line-height", "1.5", None));
  }

  #[test]
  fn test_empty_declarations() {
    let input = ".empty { }";
    let (_, rule) = CSSRule::parse(input).unwrap();
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
    let (_, rule) = CSSRule::parse(input).unwrap();
    assert_eq!(rule.selector.trim(), ".box");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("border", "1px solid black", None));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("background", "white", None));
  }

  #[test]
  fn test_rule_with_multiple_selectors() {
    let input = "h1, h2, h3 { font-weight: bold; }";
    let (_, rule) = CSSRule::parse(input).unwrap();
    assert_eq!(rule.selector, "h1, h2, h3");
    assert_eq!(rule.declarations.declarations.len(), 1);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("font-weight", "bold", None));
  }
}
