//! CSS Declaration Parser
//!
//! This module provides parsing and representation for individual CSS declarations
//! (property-value pairs like `color: red` or `margin: 10px !important`).
//!
//! ## Main API
//! 
//! - `CSSDeclaration::from_string()` - Parse a CSS declaration from a string
//! - `CSSDeclaration::new()` - Create a new declaration programmatically  
//! - `Display` trait implementation for converting back to CSS string
//!
//! ## Examples
//!
//! ```rust
//! use css_parser::CSSDeclaration;
//! 
//! // Parse from string
//! let decl = CSSDeclaration::from_string("color: red !important").unwrap();
//! assert_eq!(decl.name, "color");
//! assert_eq!(decl.value, "red");
//! assert_eq!(decl.important, true);
//!
//! // Create programmatically  
//! let decl = CSSDeclaration::new("margin", "10px", None);
//! println!("{}", decl); // "margin: 10px;"
//! ```


use std::fmt;
use crate::helpers::is_non_ascii;
use nom::{
  bytes::complete::{tag, is_not, take_while1, take_while},
  character::complete::{char, multispace0},
  combinator::{recognize, map, opt},
  sequence::{delimited, preceded, separated_pair, pair},
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
  fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(
      recognize(
        pair(
          // First character: letter, underscore, dash or non-ASCII
          take_while1(|c: char| c.is_alphabetic() || c == '_' || c == '-' || is_non_ascii(c)),

          // Rest: letters, digits, hyphens, underscores, or non-ASCII
          take_while(|c: char| c.is_alphanumeric() || c == '-' || c == '_' || is_non_ascii(c)),
        )
      ),
      |s: &str| s.to_string() 
    ).parse(input)
  }

  fn parse_value(input: &str) -> IResult<&str, (String, bool)> {
    map(
      pair(
        // Parse the main value (everything except !important)
        map(is_not(";{}!"), |s: &str| s.trim().to_string()),

        // Parse optional !important
        opt(preceded(
          multispace0,
          preceded(tag("!"), preceded(multispace0, tag("important")))
        ))
      ),
      |(value, important)| (value, important.is_some())
    ).parse(input)
  }

  fn parse_declaration(input: &str) -> IResult<&str, (String, (String, bool))> {
    separated_pair(
      preceded(multispace0, Self::parse_identifier),
      delimited(multispace0, char(':'), multispace0),
      Self::parse_value,
    ).parse(input)
  }

  pub(crate) fn parse(input: &str) -> IResult<&str, CSSDeclaration> {
    let (input, (name, (value, important))) = Self::parse_declaration(input)?;

    Ok((input, CSSDeclaration { name, value, important }))
  }

  pub fn new(name: &str, value: &str, important: Option<bool>) -> Self {
    CSSDeclaration {
      name: name.to_string(),
      value: value.to_string(),
      important: important.unwrap_or(false),
    }
  }

  pub fn from_string(input: &str) -> Result<CSSDeclaration, String> {
    let (_, decl) = Self::parse(input)
      .map_err(|_| "Failed to parse CSS declaration".to_string())?; 

    Ok(decl)
  }
}

impl fmt::Display for CSSDeclaration {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.important {
      write!(f, "{}: {} !important;", self.name, self.value)
    } else {
      write!(f, "{}: {};", self.name, self.value)
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_identifier_simple_letter_identifier() {
    let result = CSSDeclaration::parse_identifier("color");
    assert!(result.is_ok());
    let (remaining, identifier) = result.unwrap();
    assert_eq!(identifier, "color");
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_identifier_hyphenated_identifier() {
    let result = CSSDeclaration::parse_identifier("background-color");
    assert!(result.is_ok());
    let (remaining, identifier) = result.unwrap();
    assert_eq!(identifier, "background-color");
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_identifier_vendor_prefixed_identifier() {
    let result = CSSDeclaration::parse_identifier("-webkit-transform");
    assert!(result.is_ok());
    let (remaining, identifier) = result.unwrap();
    assert_eq!(identifier, "-webkit-transform");
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_identifier_stops_at_colon() {
    let result = CSSDeclaration::parse_identifier("color: red");
    assert!(result.is_ok());
    let (remaining, identifier) = result.unwrap();
    assert_eq!(identifier, "color");
    assert_eq!(remaining, ": red");
  }

  #[test]
  fn parse_identifier_stops_at_semicolon() {
    let result = CSSDeclaration::parse_identifier("color;");
    assert!(result.is_ok());
    let (remaining, identifier) = result.unwrap();
    assert_eq!(identifier, "color");
    assert_eq!(remaining, ";");
  }

  #[test]
  fn parse_identifier_fails_on_empty_input() {
    let result = CSSDeclaration::parse_identifier("");
    assert!(result.is_err());
  }

  #[test]
  fn parse_identifier_fails_starting_with_number() {
    let result = CSSDeclaration::parse_identifier("123invalid");
    assert!(result.is_err());
  }

  #[test]
  fn parse_identifier_fails_starting_with_special_char() {
    let result = CSSDeclaration::parse_identifier("@invalid");
    assert!(result.is_err());
  }

  #[test]
  fn parse_value_simple_value() {
    let result = CSSDeclaration::parse_value("red");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_with_whitespace() {
    let result = CSSDeclaration::parse_value("  red  ");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_multiple_words() {
    let result = CSSDeclaration::parse_value("1px solid red");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "1px solid red");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_with_important() {
    let result = CSSDeclaration::parse_value("red !important");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "red");
    assert_eq!(important, true);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_with_important_no_space() {
    let result = CSSDeclaration::parse_value("red!important");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "red");
    assert_eq!(important, true);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_with_important_extra_spaces() {
    let result = CSSDeclaration::parse_value("red  !  important");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "red");
    assert_eq!(important, true);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_complex_with_important() {
    let result = CSSDeclaration::parse_value("1px solid rgba(255, 0, 0, 0.5) !important");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "1px solid rgba(255, 0, 0, 0.5)");
    assert_eq!(important, true);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_stops_at_semicolon() {
    let result = CSSDeclaration::parse_value("red; color: blue");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "; color: blue");
  }

  #[test]
  fn parse_value_stops_at_closing_brace() {
    let result = CSSDeclaration::parse_value("red}");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "}");
  }

  #[test]
  fn parse_value_stops_at_opening_brace() {
    let result = CSSDeclaration::parse_value("red{");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "{");
  }

  #[test]
  fn parse_value_numeric_value() {
    let result = CSSDeclaration::parse_value("10px");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "10px");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_hex_color() {
    let result = CSSDeclaration::parse_value("#ff0000");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "#ff0000");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_url() {
    let result = CSSDeclaration::parse_value("url('image.png')");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "url('image.png')");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_calc_expression() {
    let result = CSSDeclaration::parse_value("calc(100% - 20px)");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "calc(100% - 20px)");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_value_fails_on_empty_input() {
    let result = CSSDeclaration::parse_value("");
    assert!(result.is_err());
  }

  #[test]
  fn parse_value_whitespace_with_important() {
    let result = CSSDeclaration::parse_value("  1px solid red  !important  ");
    assert!(result.is_ok());
    let (remaining, (value, important)) = result.unwrap();
    assert_eq!(value, "1px solid red");
    assert_eq!(important, true);
    assert_eq!(remaining, "  ");
  }

  #[test]
  fn parse_declaration_simple() {
    let result = CSSDeclaration::parse_declaration("color: red");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "color");
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_with_whitespace() {
    let result = CSSDeclaration::parse_declaration("  color  :  red  ");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "color");
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_hyphenated_property() {
    let result = CSSDeclaration::parse_declaration("background-color: blue");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "background-color");
    assert_eq!(value, "blue");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_vendor_prefix() {
    let result = CSSDeclaration::parse_declaration("-webkit-transform: rotate(45deg)");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "-webkit-transform");
    assert_eq!(value, "rotate(45deg)");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_with_important() {
    let result = CSSDeclaration::parse_declaration("color: red !important");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "color");
    assert_eq!(value, "red");
    assert_eq!(important, true);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_complex_value() {
    let result = CSSDeclaration::parse_declaration("border: 1px solid rgba(255, 0, 0, 0.5)");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "border");
    assert_eq!(value, "1px solid rgba(255, 0, 0, 0.5)");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_complex_with_important() {
    let result = CSSDeclaration::parse_declaration("margin: 10px 20px 30px 40px !important");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "margin");
    assert_eq!(value, "10px 20px 30px 40px");
    assert_eq!(important, true);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_no_space_around_colon() {
    let result = CSSDeclaration::parse_declaration("color:red");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "color");
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_stops_at_semicolon() {
    let result = CSSDeclaration::parse_declaration("color: red; margin: 10px");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "color");
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "; margin: 10px");
  }

  #[test]
  fn parse_declaration_stops_at_closing_brace() {
    let result = CSSDeclaration::parse_declaration("color: red}");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "color");
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "}");
  }

  #[test]
  fn parse_declaration_underscore_property() {
    let result = CSSDeclaration::parse_declaration("_private: value");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "_private");
    assert_eq!(value, "value");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_non_ascii_property() {
    let result = CSSDeclaration::parse_declaration("café: brown");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "café");
    assert_eq!(value, "brown");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_numeric_value() {
    let result = CSSDeclaration::parse_declaration("z-index: 999");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "z-index");
    assert_eq!(value, "999");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_leading_whitespace() {
    let result = CSSDeclaration::parse_declaration("   color: red");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "color");
    assert_eq!(value, "red");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn parse_declaration_fails_missing_colon() {
    let result = CSSDeclaration::parse_declaration("color red");
    assert!(result.is_err());
  }

  #[test]
  fn parse_declaration_fails_empty_input() {
    let result = CSSDeclaration::parse_declaration("");
    assert!(result.is_err());
  }

  #[test]
  fn parse_declaration_fails_no_property() {
    let result = CSSDeclaration::parse_declaration(": red");
    assert!(result.is_err());
  }

  #[test]
  fn parse_declaration_url_value() {
    let result = CSSDeclaration::parse_declaration("background-image: url('test.jpg')");
    assert!(result.is_ok());
    let (remaining, (name, (value, important))) = result.unwrap();
    assert_eq!(name, "background-image");
    assert_eq!(value, "url('test.jpg')");
    assert_eq!(important, false);
    assert_eq!(remaining, "");
  }

  #[test]
  fn test_new() {
    let decl = CSSDeclaration::new("x", "y", None);
    assert_eq!(decl.name, "x");
    assert_eq!(decl.value, "y");
    assert_eq!(decl.important, false);

    let decl_important = CSSDeclaration::new("x", "y", Some(true));
    assert_eq!(decl_important.name, "x");
    assert_eq!(decl_important.value, "y");
    assert_eq!(decl_important.important, true);
  }

  #[test]
  fn test_from_string_simple() {
    let decl = CSSDeclaration::from_string("color: red;").unwrap();
    assert_eq!(decl.name, "color");
    assert_eq!(decl.value, "red");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_from_string_values_with_whitespace() {
    let decl = CSSDeclaration::from_string("border: 1px solid red;").unwrap();
    assert_eq!(decl.name, "border");
    assert_eq!(decl.value, "1px solid red");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_from_string_no_semi() {
    let decl = CSSDeclaration::from_string("color: red").unwrap();
    assert_eq!(decl.name, "color");
    assert_eq!(decl.value, "red");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_from_string_numeric_val() {
    let decl = CSSDeclaration::from_string("padding: 10px").unwrap();
    assert_eq!(decl.name, "padding");
    assert_eq!(decl.value, "10px");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_from_string_prefix() {
    let decl = CSSDeclaration::from_string("-webkit-transition: .2s all").unwrap();
    assert_eq!(decl.name, "-webkit-transition");
    assert_eq!(decl.value, ".2s all");
    assert_eq!(decl.important, false);
  }

  #[test]
  fn test_to_string() {
    let decl = CSSDeclaration::from_string("color: red;").unwrap();
    let decl_str = decl.to_string();
    assert_eq!(decl_str, "color: red;");
  }

  #[test]
  fn test_to_string_important() {
    let decl = CSSDeclaration::new("color", "red", Some(true));
    assert_eq!(decl.to_string(), "color: red !important;");
  }
}
