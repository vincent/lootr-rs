//! Module containing item types used in Lootr.
//!
//! Items are the core data type used to hold your items data in Lootr.
//! It holds a `name` and some `props`.
//!
//! The easiest way to create an Item is to use [`Item::from`](crate::item::Item::from).
//!
//! Item [`Props`](crate::item::Props) can be queried directly with `has_prop()`, `get_prop()` and `set_prop()`
//!

use std::collections::HashMap;

/// Holds the item properties in an `HashMap<&str, &str>`.
///
pub type Props = HashMap<&'static str, &'static str>;

/// Describe a modifier helper function.
///
pub type Modifier = fn(item: &mut Item) -> Item;

/// Holds a Lootr Item.
///
/// Items are the core data type used to hold your items data in Lootr.
/// It holds a `name` and some `props`.
///
/// The easiest way to create an Item is to use [`Item::from`](crate::item::Item::from).
///
#[derive(Debug, Clone)]
pub struct Item {
    /// Holds the item name.
    ///
    pub name: &'static str,

    /// Holds the item properties.
    ///
    pub props: Option<Props>,
}
impl Item {
    /// Create an Item with just a name.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::Item;
    ///
    /// let hat = Item::a("hat");
    /// ```
    pub fn a(name: &'static str) -> Self {
        Self { name, props: None }
    }

    /// Create an Item with just a name.
    ///
    /// * `name: &str` Item name
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::Item;
    ///
    /// let hat = Item::an("ascot");
    /// ```
    pub fn an(name: &'static str) -> Self {
        Item::a(name)
    }

    /// Create an Item with just a name.
    ///
    /// * `name: &str` Item name
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::Item;
    ///
    /// let hat = Item::named("greg");
    /// ```
    pub fn named(name: &'static str) -> Self {
        Item::a(name)
    }

    /// Create an Item with a name and some properties.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::{Item, Props};
    ///
    /// let hat = Item::from("hat", Props::from([
    ///     ("color", "black"),
    ///     ("size", "small"),
    /// ]));
    /// ```
    pub fn from(name: &'static str, props: Props) -> Self {
        Item {
            name,
            props: Some(props),
        }
    }

    /// Create an Item by extending a previous one, with new name and properties.
    /// The given properties will overload the given item ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::{Item, Props};
    ///
    /// let hat = Item::from("hat", Props::from([
    ///     ("color", "black"),
    ///     ("size", "large"),
    /// ]));
    ///
    /// let cap = hat.extend("cap", &Props::from([
    ///     ("size", "small"),
    /// ]));
    ///
    /// assert_eq!(cap.get_prop("color"), Some("black"));
    /// assert_eq!(cap.get_prop("size"), Some("small"));
    /// ```
    pub fn extend(&self, name: &'static str, ext_props: &Props) -> Self {
        let mut new_props: HashMap<&str, &str> = HashMap::new();
        new_props.extend(self.props.clone().unwrap_or_default().iter());
        new_props.extend(ext_props.iter());

        Item {
            name,
            props: Some(new_props),
        }
    }

    /// Check the existence of an item property.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::{Item, Props};
    ///
    /// let hat = Item::from("hat", Props::from([
    ///     ("color", "black"),
    ///     ("size", "small"),
    /// ]));
    ///
    /// assert_eq!(hat.has_prop("size"), true)
    /// ```
    pub fn has_prop(&self, key: &'static str) -> bool {
        match &self.props {
            None => false,
            Some(props) => props.contains_key(key),
        }
    }

    /// Return an item property.
    /// If this prop does not exist, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::{Item, Props};
    ///
    /// let hat = Item::from("hat", Props::from([
    ///     ("color", "black"),
    ///     ("size", "small"),
    /// ]));
    ///
    /// assert_eq!(hat.get_prop("size"), Some("small"))
    /// ```
    pub fn get_prop(&self, key: &'static str) -> Option<&str> {
        match &self.props {
            None => None,
            Some(props) => props.get(key).copied(),
        }
    }

    /// Set an item property.
    /// If this prop already exist, the value is replaced.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::{Item, Props};
    ///
    /// let mut hat = Item::from("hat", Props::from([
    ///     ("color", "black"),
    ///     ("size", "small"),
    /// ]));
    ///
    /// hat.set_prop("fancy", "yes");
    /// hat.set_prop("size", "large");
    ///
    /// assert_eq!(hat.get_prop("fancy"), Some("yes"));
    /// assert_eq!(hat.get_prop("size"), Some("large"));
    /// ```
    pub fn set_prop(&mut self, key: &'static str, value: &'static str) -> &mut Self {
        let mut new_props: HashMap<&str, &str> = HashMap::new();
        new_props.extend(self.props.clone().unwrap_or_default().iter());
        new_props.insert(key, value);
        self.props = Some(new_props);

        self
    }
}
