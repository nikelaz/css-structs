//! CSS Stylesheet Parser
//!
//! This module provides parsing and representation for complete CSS stylesheets
//! containing multiple CSS rules. A stylesheet represents the top-level structure
//! that holds all CSS rules like `body { margin: 0; } .title { color: red; }`.
//!
//! ## Main API
//! 
//! - `Stylesheet::from_string()` - Parse a complete stylesheet from a CSS string
//! - `Stylesheet::new()` - Create a new stylesheet programmatically with optional rules
//! - `Display` trait implementation for converting back to CSS string format
//!
//! ## Examples
//!
//! ```rust
//! use css_structs::Stylesheet;
//! 
//! // Parse from string
//! let css = "body { margin: 0; padding: 0; } h1 { color: red; }";
//! let stylesheet = Stylesheet::from_string(css).unwrap();
//! assert_eq!(stylesheet.rules.len(), 2);
//!
//! // Create with existing rules
//! let stylesheet = Stylesheet::new(Some(vec![rule1, rule2]));
//! println!("{}", stylesheet); // Outputs formatted CSS
//!
//! // Create empty stylesheet
//! let empty = Stylesheet::new(None);
//! assert!(empty.rules.is_empty());
//! ```


use std::fmt;
use crate::css_rule::CSSRule;
use nom::{
  IResult,
  multi::many0,
  Parser,
};


#[derive(Debug, Clone, PartialEq)]
pub struct Stylesheet {
  pub rules: Vec<CSSRule>,
}

impl Stylesheet {  
  fn parse(input: &str) -> IResult<&str, Vec<CSSRule>> {
    many0(CSSRule::parse).parse(input)
  }

  pub fn from_string(input: &str) -> Result<Self, String> {
    let (_, rules) = Self::parse(input)
      .map_err(|_| "Failed to parse CSS".to_string())?;

    Ok(Self { rules })
  }

  pub fn new(rules: Option<Vec<CSSRule>>) -> Self {
    if let Some(rules) = rules {
      Self { rules }
    } else {
      Self { rules: Vec::new() }
    }
  }
}

impl fmt::Display for Stylesheet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let stylesheet = self.rules
      .iter()
      .map(|decl| decl.to_string())
      .collect::<Vec<_>>()
      .join(" ");

    write!(f, "{}", stylesheet)
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::css_declaration::CSSDeclaration;

  #[test]
  fn test_empty_stylesheet() {
    let input = "";
    let result = Stylesheet::from_string(input).unwrap();
    assert!(result.rules.is_empty());
  }

  #[test]
  fn test_single_rule() {
    let input = "body { margin: 0; padding: 0; }";
    let result = Stylesheet::from_string(input).unwrap();
    assert_eq!(result.rules.len(), 1);
    let rule = &result.rules[0];
    assert_eq!(rule.selector, "body");
    assert_eq!(rule.declarations.declarations.len(), 2);
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("margin", "0", None));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("padding", "0", None));
  }

  #[test]
  fn test_multiple_rules() {
    let input = r#"
            h1 { color: red; }
            p { font-size: 16px; }
            .box { border: 1px solid black; background: white; }
        "#;

    let result = Stylesheet::from_string(input).unwrap();
    assert_eq!(result.rules.len(), 3);

    let rule1 = &result.rules[0];
    assert_eq!(rule1.selector, "h1");
    assert_eq!(rule1.declarations.declarations[0], CSSDeclaration::new("color", "red", None));

    let rule2 = &result.rules[1];
    assert_eq!(rule2.selector, "p");
    assert_eq!(rule2.declarations.declarations[0], CSSDeclaration::new("font-size", "16px", None));

    let rule3 = &result.rules[2];
    assert_eq!(rule3.selector, ".box");
    assert_eq!(rule3.declarations.declarations.len(), 2);
    assert_eq!(rule3.declarations.declarations[0], CSSDeclaration::new("border", "1px solid black", None));
    assert_eq!(rule3.declarations.declarations[1], CSSDeclaration::new("background", "white", None));
  }

  #[test]
  fn test_whitespace_and_newlines() {
    let input = r#"
            .title {
                font-weight: bold;
                font-size: 24px;
            }

            .subtitle {
                font-weight: normal;
                font-size: 18px;
            }
        "#;

    let result = Stylesheet::from_string(input).unwrap();
    assert_eq!(result.rules.len(), 2);

    let title_rule = &result.rules[0];
    assert_eq!(title_rule.selector, ".title");
    assert_eq!(title_rule.declarations.declarations[0], CSSDeclaration::new("font-weight", "bold", None));
    assert_eq!(title_rule.declarations.declarations[1], CSSDeclaration::new("font-size", "24px", None));

    let subtitle_rule = &result.rules[1];
    assert_eq!(subtitle_rule.selector, ".subtitle");
    assert_eq!(subtitle_rule.declarations.declarations[0], CSSDeclaration::new("font-weight", "normal", None));
    assert_eq!(subtitle_rule.declarations.declarations[1], CSSDeclaration::new("font-size", "18px", None));
  }

  #[test]
  #[should_panic]
  fn test_malformed_css_returns_error() {
    let input = "div { color: blue; padding: 10px ";
    let result = std::panic::catch_unwind(|| Stylesheet::from_string(input));
    assert!(result.is_err(), "Should panic due to missing closing brace");
  }
}
