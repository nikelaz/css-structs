use crate::css_rule::CSSRule;
use crate::css_declaration_list::CSSDeclarationList;
use nom::{
  IResult,
  bytes::complete::{is_not, tag},
  sequence::{delimited, preceded, terminated},
  multi::many0,
  combinator::map,
  character::complete::{char, multispace0},
  Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Stylesheet {
  pub rules: Vec<CSSRule>,
}

fn rules_parser(input: &str) -> IResult<&str, Vec<CSSRule>> {
  many0(CSSRule::from_string).parse(input)
}

impl Stylesheet {
  pub fn from_string(css_block: &str) -> Result<Self, &str> {
    let (_, rules) = rules_parser.parse(css_block).unwrap();
    Ok(Stylesheet { rules })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::css_declaration::CSSDeclaration;
  use crate::css_declaration_list::CSSDeclarationList;

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
    assert_eq!(rule.declarations.declarations[0], CSSDeclaration::new("margin", "0"));
    assert_eq!(rule.declarations.declarations[1], CSSDeclaration::new("padding", "0"));
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
    assert_eq!(rule1.declarations.declarations[0], CSSDeclaration::new("color", "red"));

    let rule2 = &result.rules[1];
    assert_eq!(rule2.selector, "p");
    assert_eq!(rule2.declarations.declarations[0], CSSDeclaration::new("font-size", "16px"));

    let rule3 = &result.rules[2];
    assert_eq!(rule3.selector, ".box");
    assert_eq!(rule3.declarations.declarations.len(), 2);
    assert_eq!(rule3.declarations.declarations[0], CSSDeclaration::new("border", "1px solid black"));
    assert_eq!(rule3.declarations.declarations[1], CSSDeclaration::new("background", "white"));
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
    assert_eq!(title_rule.declarations.declarations[0], CSSDeclaration::new("font-weight", "bold"));
    assert_eq!(title_rule.declarations.declarations[1], CSSDeclaration::new("font-size", "24px"));

    let subtitle_rule = &result.rules[1];
    assert_eq!(subtitle_rule.selector, ".subtitle");
    assert_eq!(subtitle_rule.declarations.declarations[0], CSSDeclaration::new("font-weight", "normal"));
    assert_eq!(subtitle_rule.declarations.declarations[1], CSSDeclaration::new("font-size", "18px"));
  }

  #[test]
  #[should_panic]
  fn test_malformed_css_returns_error() {
    let input = "div { color: blue; padding: 10px ";
    let result = std::panic::catch_unwind(|| Stylesheet::from_string(input));
    assert!(result.is_err(), "Should panic due to missing closing brace");
  }
}
