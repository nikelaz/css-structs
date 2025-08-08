//! CSS Parser Library
//!
//! A Rust library for parsing and manipulating CSS stylesheets, rules, and declarations.
//!
//! ## Main Components
//!
//! - [`Stylesheet`] - Complete CSS stylesheet parser
//! - [`css_rule::CSSRule`] - Individual CSS rule parser  
//! - [`css_declaration_list::CSSDeclarationList`] - CSS declaration list parser
//! - [`css_declaration::CSSDeclaration`] - Individual CSS declaration parser
//!
//! ## Quick Start
//!
//! ```rust
//! use css_parser::Stylesheet;
//!
//! let css = "body { margin: 0; padding: 0; }";
//! let stylesheet = Stylesheet::from_string(css).unwrap();
//! println!("{}", stylesheet);
//! ```

mod helpers;
pub mod css_declaration;
pub mod css_declaration_list;
pub mod css_rule;
pub mod stylesheet;

// Re-export main types at the crate root for convenience
pub use stylesheet::Stylesheet;
pub use css_rule::CSSRule;
pub use css_declaration_list::CSSDeclarationList;
pub use css_declaration::CSSDeclaration;
