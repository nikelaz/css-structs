mod helpers;
pub mod css_declaration;
pub mod css_declaration_list;
pub mod css_rule;
pub mod stylesheet;

use crate::stylesheet::Stylesheet;

fn main() {
  println!("Parsing demo");
  
  let css = r#"
    body {
      padding: 0;
    }

    .-c-red {
      color: #ff0000 !important;
    }

    .block {
      display: block;
    }
  "#;

  let parsed_stylesheet = Stylesheet::from_string(css);

  println!("{:?}", parsed_stylesheet);
}
