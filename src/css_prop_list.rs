use regex::Regex;
use crate::css_prop::CSSProp;
use crate::css_prop_list::CSSPropList;

pub struct CSSPropList {
  pub props: Vec<CSSProp>,
}

impl CSSPropList {
  pub fn from_string(css_block: &str) -> Result<Self, String> {
    let pattern = Regex::new(r"\s*([^:]+)\s*:\s*([^;]+)\s*;?").map_err(|e| e.to_string())?;
    let mut props = Vec::new();

    for cap in pattern.captures_iter(css_block) {
      let prop_name = cap[1].trim().to_string();
      let prop_value = cap[2].trim().to_string();
      let prop = CSSProp {
        name: prop_name,
        value: prop_value,
      };
      props.push(prop);
    }

    Ok(CSSPropList { props })
  }

  pub fn remove_prop(&mut self, prop_name: &str) {
    self.props.retain(|prop| prop.name != prop_name);
  }

  pub fn to_string(&self) -> String {
    self.props
      .iter()
      .map(|prop| prop.to_string())
      .collect::<Vec<String>>()
      .join(" ")
  }
}
