mod tests;

use ascii_tree::{
    write_tree,
    Tree::{Leaf, Node},
};
use rand::{seq::SliceRandom, Rng};
use std::{collections::HashMap, fmt, ops::RangeInclusive};

const ROOT: Option<&str> = None;
const SEPARATOR: char = '/';

#[derive(Default)]
pub struct Lootr {
    items: Vec<Item>,
    branchs: HashMap<&'static str, Lootr>,
    modifiers: Vec<Modifier>,
}

impl Lootr {
    /// Create a new lootbag
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new lootbag from given items
    ///
    /// * `items` A Vec of Items
    ///
    pub fn from(items: Vec<Item>) -> Self {
        Self {
            items,
            branchs: HashMap::new(),
            modifiers: vec![],
        }
    }

    /// Return this lootbag branchs
    ///
    pub fn branchs(&self) -> &HashMap<&str, Lootr> {
        &self.branchs
    }

    /// Return this lootbag items (at this level)
    ///
    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    /// Return this lootbag items count (at this level)
    ///
    pub fn self_count(&self) -> usize {
        self.items.len()
    }

    /// Return this lootbag items count (including any sublevel)
    ///
    pub fn all_count(&self) -> usize {
        self.all_items().len()
    }

    /// Add an item at this level
    ///
    /// * `item` Item
    ///
    /// Returns the current lootbag
    ///
    pub fn add(&mut self, item: Item) -> &mut Self {
        self.items.push(item);

        self
    }

    /// Add an item in the given branch
    ///
    /// * `item` Item
    /// * `path` Path to the destination branch
    ///
    /// Returns the current lootbag
    ///
    pub fn add_in(&mut self, item: Item, path: &'static str) -> &mut Self {
        match self.branch_mut(path) {
            None => panic!("this path does not exist"),
            Some(branch) => branch.add(item),
        };

        self
    }

    /// Returns the branch at the given path.
    ///
    /// * `path` Branch path
    ///
    pub fn branch_mut(&mut self, path: &'static str) -> Option<&mut Lootr> {
        let cname = Self::clean(path);

        // simple case
        if self.branchs.contains_key(&cname) {
            return self.branchs.get_mut(&cname);
        }

        if !cname.contains(SEPARATOR) {
            return None;
        }

        // segmented path
        let leaf = path
            .trim_matches(SEPARATOR)
            .split(SEPARATOR)
            .fold(self, |acc, s| acc.branch_mut(s).unwrap());

        Some(leaf)
    }

    /// Returns the branch at the given path.
    /// If the branch does not exit yet, `None` is returned
    ///
    /// * `path` Branch path
    ///
    pub fn branch(&self, path: &'static str) -> Option<&Lootr> {
        let cname = Self::clean(path);

        // simple case
        if self.branchs.contains_key(&cname) {
            return self.branchs.get(&cname);
        }

        if !cname.contains(SEPARATOR) {
            return None;
        }

        // segmented path
        let leaf = path
            .trim_matches(SEPARATOR)
            .split(SEPARATOR)
            .fold(self, |acc, s| match acc.branch(s) {
                Some(branch) => branch,
                _ => panic!("this branch does not exist: {s}"),
            });

        Some(leaf)
    }

    /// Add a branch, return self (the owner)
    ///
    /// * `path` Branch path
    ///
    pub fn add_branch(&mut self, path: &'static str, branch: Lootr) -> &mut Self {
        self.branchs.insert(path, branch);
        self
    }

    /// Return all items in the current and nested branchs
    ///
    pub fn all_items(&self) -> Vec<Item> {
        let mut bag = vec![];

        bag.append(&mut self.items.clone());

        for b in self.branchs.values() {
            bag.append(&mut b.all_items().to_vec());
        }

        bag
    }

    /// Pick a random item from the specified branch
    ///
    /// * `catalog_path` Branch to get an item from, or ROOT
    /// * `nesting` Depth limit
    /// * `threshold` Chances (0-1) to go deeper
    ///
    /// Returns `Some(Item)` or `None`
    ///
    pub fn roll(
        &self,
        catalog_path: Option<&'static str>,
        nesting: i16,
        threshold: f32,
    ) -> Option<&Item> {
        let branch = match catalog_path {
            None => self,
            Some(path) => self.branch(path).unwrap(),
        };

        branch.random_pick(nesting, threshold).to_owned()
    }

    /// Pick a random item anywhere in that branch
    ///
    /// Returns `Some(Item)` or `None`
    ///
    pub fn roll_any(&mut self) -> Option<&Item> {
        self.roll(ROOT, i16::MAX, 1.0)
    }

    /// Roll against a looting table
    ///
    /// * `drops` A Drops table
    ///
    /// Returns a vec of Item
    ///
    pub fn loot(&self, drops: &[Drop]) -> Vec<Item> {
        let mut rewards = vec![];

        for d in drops {
            let item = self.roll(d.from, d.depth, d.luck);

            if item.is_none() {
                continue;
            }

            let stack = rand::thread_rng().gen_range(d.stack.clone());

            (0..stack).for_each(|_s| {
                let mut citem = item.unwrap().clone();

                if !self.modifiers.is_empty() && d.modify {
                    citem = self.random_modifier()(&mut citem);
                }

                rewards.push(citem)
            });
        }

        rewards
    }

    fn random_pick(&self, nesting: i16, threshold: f32) -> Option<&Item> {
        let mut bag = vec![];
        let rng = &mut rand::thread_rng();

        if rng.gen::<f32>() < threshold && !self.items.is_empty() {
            if let Some(item) = self.items.choose(&mut rand::thread_rng()) {
                bag.push(item);
            }
        }

        if nesting > 0 {
            for b in self.branchs.values() {
                let decrease: f32 = rng.gen_range(0.0001..1.0);
                let new_threshold = (threshold * decrease).clamp(0.0, 1.0);
                let new_threshold = (new_threshold * 100.0).round() / 100.0;

                if let Some(item) = b.random_pick(nesting - 1, new_threshold) {
                    bag.push(item);
                }
            }
        }

        bag.choose(rng).copied()
    }

    /// Add a modifier
    ///
    pub fn add_modifier(&mut self, modifier: Modifier) -> &mut Self {
        self.modifiers.push(modifier);
        self
    }

    fn fmt_node(&self, name: &str) -> ascii_tree::Tree {
        let mut children: Vec<ascii_tree::Tree> = vec![];

        children.push(Leaf(
            self.items()
                .iter()
                .map(|item| String::from(item.name))
                .collect(),
        ));

        let mut branchs: Vec<ascii_tree::Tree> = self
            .branchs()
            .iter()
            .map(|(&name, branch)| branch.fmt_node(name))
            .collect();
        children.append(&mut branchs);

        Node(String::from(name), children)
    }

    fn random_modifier(&self) -> &Modifier {
        self.modifiers.choose(&mut rand::thread_rng()).unwrap()
    }

    fn clean(path: &'static str) -> &str {
        path.trim_matches(SEPARATOR)
    }
}

impl fmt::Display for Lootr {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_tree(f, &self.fmt_node("ROOT"))
    }
}

pub type Props = HashMap<&'static str, &'static str>;
pub type Modifier = fn(item: &mut Item) -> Item;

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

#[derive(Clone)]
pub struct Drop {
    pub from: Option<&'static str>,
    pub luck: f32,
    pub modify: bool,
    pub depth: i16,
    pub stack: RangeInclusive<u32>,
}

pub struct DropBuilder {
    pub from: Option<&'static str>,
    pub luck: f32,
    pub modify: bool,
    pub depth: i16,
    pub stack: RangeInclusive<u32>,
}

impl DropBuilder {
    pub fn new() -> DropBuilder {
        DropBuilder {
            from: ROOT,
            luck: f32::MAX,
            modify: false,
            depth: 1,
            stack: 1..=1,
        }
    }

    pub fn from(mut self, path: &'static str) -> DropBuilder {
        self.from = Some(path);
        self
    }

    pub fn luck(mut self, luck: f32) -> DropBuilder {
        self.luck = luck;
        self
    }

    pub fn anydepth(mut self) -> DropBuilder {
        self.depth = i16::MAX;
        self
    }

    pub fn depth(mut self, depth: i16) -> DropBuilder {
        self.depth = depth;
        self
    }

    pub fn build(&self) -> Drop {
        Drop {
            from: self.from,
            luck: self.luck,
            modify: self.modify,
            depth: self.depth,
            stack: self.stack.clone(),
        }
    }
}

impl Default for DropBuilder {
    fn default() -> Self {
        Self::new()
    }
}
