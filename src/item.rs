//! Module containing item types used in Lootr.
//!
//! Items are the core data type used to hold your items data in Lootr.
//! It holds a `name` and some `props`.
//!
//! The easiest way to create an Item is to use [`Item::from`](crate::item::Item::from).
//!
//! Item [`Props`](crate::item::Props) can be queried directly with `has_prop()`, `get_prop()` and `set_prop()`
//!

use std::{
    collections::HashMap,
    fmt::{self, format, Display},
};

/// Holds the item properties in an `HashMap<&str, &str>`.
///
pub type Props<'a> = HashMap<&'a str, &'a str>;

/// Holds a modifier helper function.
///
pub type Modifier = fn(item: Item) -> Item;

/// Holds a Lootr Item.
///
/// Items are the core data type used to hold your items data in Lootr.
/// It holds a `name` and some `props`.
///
/// The easiest way to create an Item is to use [`Item::from`](crate::item::Item::from).
///
#[derive(Debug, Clone)]
pub struct Item<'a> {
    /// Holds the item name.
    ///
    pub name: &'a str,

    /// Holds the item properties.
    ///
    pub props: Option<Props<'a>>,
}

impl<'a> Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let props = self.props.clone().unwrap_or_default();
        let props: Vec<String> = props
            .iter()
            .map(|(key, value)| format(format_args!("{}={}", key, value)))
            .collect::<_>();
        write!(f, "{}{{{}}}", self.name, props.join(","))
    }
}

impl<'a> Item<'a> {
    /// Create an Item with just a name.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::item::Item;
    ///
    /// let hat = Item::a("hat");
    /// ```
    pub fn a(name: &'a str) -> Self {
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
    pub fn an(name: &'a str) -> Self {
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
    pub fn named(name: &'a str) -> Self {
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
    pub fn from(name: &'a str, props: Props<'a>) -> Self {
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
    /// let cap = hat.extend("cap", Props::from([
    ///     ("size", "small"),
    /// ]));
    ///
    /// assert_eq!(cap.get_prop("color"), Some("black"));
    /// assert_eq!(cap.get_prop("size"), Some("small"));
    /// ```
    pub fn extend(&self, name: &'a str, ext_props: Props<'a>) -> Self {
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
    pub fn has_prop(&self, key: &str) -> bool {
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
    pub fn get_prop(&self, key: &str) -> Option<&str> {
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
    pub fn set_prop<'b: 'a>(&mut self, key: &'b str, value: &'b str) -> &mut Self {
        let mut new_props: HashMap<&str, &str> = HashMap::new();
        new_props.extend(self.props.clone().unwrap_or_default().iter());
        new_props.insert(key, value);
        self.props = Some(new_props);

        self
    }
}
