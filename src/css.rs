use regex::Regex;
use crate::css_prop_list::CSSPropList;
use crate::css_rule::CSSRule;

pub struct CSS {
  pub rules: Vec<CSSRule>,
}

impl CSS {
  pub fn from_string(css_block: &str) -> Result<Self, String> {
    let re = Regex::new(r"(?s)([^{}]+)\s*\{([^}]*)\}").map_err(|e| e.to_string())?;
    let mut rules = Vec::new();

    for cap in re.captures_iter(css_block) {
      let selector = cap[1].trim().to_string();
      let body = cap[2].trim().to_string();
      let css_rule = CSSRule::new(selector.as_str(), CSSPropList::from_string(body.as_str()));
      rules.push(css_rule);
    }

    Ok(CSS { rules })
  }  
}

