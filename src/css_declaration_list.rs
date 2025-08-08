//! CSS Declaration List Parser
//!
//! This module provides parsing and representation for CSS declaration lists
//! (collections of property-value pairs typically found inside CSS rule blocks
//! like `{ color: red; margin: 10px; padding: 5px !important }`).
//!
//! ## Main API
//! 
//! - `CSSDeclarationList::from_string()` - Parse a CSS declaration list from a string
//! - `CSSDeclarationList::new()` - Create a new declaration list programmatically  
//! - `remove_declaration()` - Remove declarations by property name
//! - `Display` trait implementation for converting back to CSS string
//!
//! ## Examples
//!
//! ```rust
//! use css_structs::CSSDeclarationList;
//! 
//! // Parse from string
//! let list = CSSDeclarationList::from_string("color: red; margin: 10px; padding: 5px").unwrap();
//! assert_eq!(list.declarations.len(), 3);
//!
//! // Create and modify programmatically  
//! let mut list = CSSDeclarationList::from_string("color: red; margin: 10px").unwrap();
//! list.remove_declaration("color");
//! println!("{}", list); // "margin: 10px;"
//!
//! // Create a new empty declaration list
//! let empty_list = CSSDeclarationList::new();
//! assert!(empty_list.declarations.is_empty());
//! ```


use std::fmt;
use nom::{
  character::complete::{char, multispace0},
  combinator::opt,
  multi::many0,
  sequence::{delimited, preceded},
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
      preceded(
        many0(delimited(multispace0, char(';'), multispace0)),
        delimited(
          multispace0,
          CSSDeclaration::parse,
          opt(char(';')),
        )
      )
    ).parse(input)
  }

  pub(crate) fn parse(input: &str) -> IResult<&str, CSSDeclarationList> {
    let (input, declarations) = Self::parse_declarations(input)?;

    Ok((input, CSSDeclarationList { declarations }))
  }

  pub fn from_string(css_block: &str) -> Result<Self, String> {
    let (_, declaration_list) = Self::parse(css_block)
      .map_err(|_| "Failed to parse CSS declarations list".to_string())?;

    Ok(declaration_list)
  }

  pub fn remove_declaration(&mut self, decl_name: &str) {
    self.declarations.retain(|decl| decl.name != decl_name);
  }

  pub fn new() -> Self {
    CSSDeclarationList {
      declarations: Vec::new(),
    }
  }
}

impl fmt::Display for CSSDeclarationList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let list_str = self.declarations
      .iter()
      .map(|decl| decl.to_string())
      .collect::<Vec<_>>()
      .join(" ");

    write!(f, "{}", list_str)
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::css_declaration::CSSDeclaration;

  #[test]
  fn test_parse_declarations_single_declaration() {
    let input = "color: red;";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "");
    assert_eq!(declarations.len(), 1);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
  }

  #[test]
  fn test_parse_declarations_multiple_declarations() {
    let input = "color: red; background: blue; margin: 10px;";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "");
    assert_eq!(declarations.len(), 3);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(declarations[1], CSSDeclaration::new("background", "blue", None));
    assert_eq!(declarations[2], CSSDeclaration::new("margin", "10px", None));
  }

  #[test]
  fn test_parse_declarations_no_trailing_semicolon() {
    let input = "font-size: 16px";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "");
    assert_eq!(declarations.len(), 1);
    assert_eq!(declarations[0], CSSDeclaration::new("font-size", "16px", None));
  }

  #[test]
  fn test_parse_declarations_mixed_semicolons() {
    let input = "color: red; background: blue; padding: 5px";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "");
    assert_eq!(declarations.len(), 3);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(declarations[1], CSSDeclaration::new("background", "blue", None));
    assert_eq!(declarations[2], CSSDeclaration::new("padding", "5px", None));
  }

  #[test]
  fn test_parse_declarations_leading_whitespace() {
    let input = "   color: red; background: blue;";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "");
    assert_eq!(declarations.len(), 2);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(declarations[1], CSSDeclaration::new("background", "blue", None));
  }

  #[test]
  fn test_parse_declarations_whitespace_between() {
    let input = "color: red;   background: blue;   padding: 10px;";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "");
    assert_eq!(declarations.len(), 3);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(declarations[1], CSSDeclaration::new("background", "blue", None));
    assert_eq!(declarations[2], CSSDeclaration::new("padding", "10px", None));
  }

  #[test]
  fn test_parse_declarations_trailing_whitespace() {
    let input = "color: red; background: blue;   ";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "   ");
    assert_eq!(declarations.len(), 2);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(declarations[1], CSSDeclaration::new("background", "blue", None));
  }

  #[test]
  fn test_parse_declarations_empty_input() {
    let input = "";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "");
    assert_eq!(declarations.len(), 0);
  }

  #[test]
  fn test_parse_declarations_whitespace_only() {
    let input = "   \n  \t  ";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(remaining, "   \n  \t  ");
    assert_eq!(declarations.len(), 0);
  }

  #[test]
  fn test_parse_declarations_extra_semicolons() {
    let input = "color: red;; background: blue;;";
    let (_, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    println!("{:?}", declarations);
    assert_eq!(declarations.len(), 2);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(declarations[1], CSSDeclaration::new("background", "blue", None));
  }

  #[test]
  fn test_parse_declarations_newlines_and_tabs() {
    let input = "\n\tcolor: red;\n\tbackground: blue;\n\t";
    let (_, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(declarations.len(), 2);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(declarations[1], CSSDeclaration::new("background", "blue", None));
  }

  #[test]
  fn test_parse_declarations_partial_parse() {
    let input = "color: red; background: blue; } extra content";
    let (remaining, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    // Should parse valid declarations and leave remaining input
    assert_eq!(remaining.trim(), "} extra content");
    assert_eq!(declarations.len(), 2);
    assert_eq!(declarations[0], CSSDeclaration::new("color", "red", None));
    assert_eq!(declarations[1], CSSDeclaration::new("background", "blue", None));
  }

  #[test]
  fn test_parse_declarations_single_semicolon() {
    let input = ";";
    let (_, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    // Should handle a single semicolon gracefully
    assert_eq!(declarations.len(), 0);
  }

  #[test]
  fn test_parse_declarations_semicolon_with_whitespace() {
    let input = "  ;  ";
    let (_, declarations) = CSSDeclarationList::parse_declarations(input).unwrap();

    assert_eq!(declarations.len(), 0);
  }

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
