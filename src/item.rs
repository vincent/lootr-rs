use std::collections::HashMap;

use crate::Props;

#[derive(Clone)]
pub struct Item {
    pub name: &'static str,
    pub props: Option<Props>,
}
impl Item {
    pub fn a(name: &'static str) -> Self {
        Self { name, props: None }
    }

    pub fn an(name: &'static str) -> Self {
        Item::a(name)
    }

    pub fn named(name: &'static str) -> Self {
        Item::a(name)
    }

    pub fn from(name: &'static str, props: Props) -> Self {
        Item {
            name,
            props: Some(props),
        }
    }

    pub fn has_prop(&self, key: &'static str) -> bool {
        match &self.props {
            None => false,
            Some(props) => props.contains_key(key),
        }
    }

    pub fn get_prop(&self, key: &'static str) -> Option<&str> {
        match &self.props {
            None => None,
            Some(props) => props.get(key).copied(),
        }
    }

    pub fn add_prop(&mut self, name: &'static str, value: &'static str) -> &mut Self {
        let props = self.props.clone().unwrap_or_default();
        let mut new_props: HashMap<&str, &str> = HashMap::new();
        new_props.extend(props.iter());
        new_props.insert(name, value);
        self.props = Some(new_props);
        self
    }

    pub fn extend(&self, name: &'static str, ext_props: &Props) -> Self {
        let props = self.props.clone().unwrap_or_default();
        let mut new_props: HashMap<&str, &str> = HashMap::new();
        new_props.extend(props.iter());
        new_props.extend(ext_props.iter());
        Item {
            name,
            props: Some(new_props),
        }
    }
}
