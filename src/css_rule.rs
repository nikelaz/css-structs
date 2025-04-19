use crate::css_prop_list::CSSPropList;
use crate::css_rule::CSSRule;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct CSSRule {
  pub selector: String,
  pub props: CSSPropList, 
}

impl CSSRule {
  pub fn new(selector: &str, props: &CSSPropList) {
    CSSRule { selector, props }
  }
}
