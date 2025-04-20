mod css_declaration;
mod css_declaration_list;
mod css_rule;
mod stylesheet;

use crate::stylesheet::Stylesheet;

fn main() {
  println!("Hello, world!");
  
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
