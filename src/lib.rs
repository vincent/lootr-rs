#![doc = include_str!("../README.md")]

pub mod drops;
pub mod item;
mod tests;

use ascii_tree::{
    write_tree,
    Tree::{Leaf, Node},
};
use rand::{seq::SliceRandom, Rng, RngCore, SeedableRng};
use std::{collections::HashMap, fmt};
use rand_chacha::ChaCha20Rng;

use crate::{
    drops::Drop,
    item::{Item, Modifier},
};

pub const ROOT: Option<&str> = None;
const SEPARATOR: char = '/';

pub struct Lootr {
    items: Vec<Item>,
    branchs: HashMap<&'static str, Lootr>,
    modifiers: Vec<Modifier>,
    rng: Box<ChaCha20Rng>,
}

impl fmt::Display for Lootr {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_tree(f, &self.fmt_node("ROOT"))
    }
}

impl Lootr {
    /// Create a new lootbag
    ///
    pub fn new() -> Self {
        Self::from(vec![])
    }

    /// Create a new lootbag from given items
    ///
    pub fn from(items: Vec<Item>) -> Self {
        Self {
            items,
            branchs: HashMap::new(),
            modifiers: vec![],
            rng: Box::new(ChaCha20Rng::from_entropy()),
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
    /// Returns the current lootbag
    ///
    pub fn add(&mut self, item: Item) -> &mut Self {
        self.items.push(item);

        self
    }

    /// Add an item in the given branch
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
    /// Returns `Some(Item)` or `None`
    ///
    pub fn roll(
        &mut self,
        catalog_path: Option<&'static str>,
        nesting: i16,
        threshold: f32,
    ) -> Option<&Item> {
        let branch = match catalog_path {
            None => self,
            Some(path) => self.branch_mut(path).unwrap(),
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
    /// Returns a vec of Item
    ///
    pub fn loot(&mut self, drops: &[Drop]) -> Vec<Item> {
        let mut rewards = vec![];
        let mut rng = self.rng.to_owned();
        let modifiers = self.modifiers.clone();

        for d in drops {
            let item = self.roll(d.path, d.depth, d.luck);

            if item.is_none() {
                continue;
            }

            let stack = rng.gen_range(d.stack.clone());

            (0..stack).for_each(|_s| {
                let mut citem = item.unwrap().clone();

                if !modifiers.is_empty() && d.modify {
                    let modifier = modifiers.choose(&mut rng).unwrap();
                    citem = modifier(&mut citem);
                }

                rewards.push(citem)
            });
        }

        rewards
    }

    fn random_pick(&mut self, nesting: i16, threshold: f32) -> Option<&Item> {
        let mut bag = vec![];

        if self.rng.gen::<f32>() < threshold && !self.items.is_empty() {
            if let Some(item) = self.items.choose(&mut self.rng) {
                bag.push(item);
            }
        }

        if nesting > 0 {
            for b in self.branchs.values_mut() {
                let decrease: f32 = self.rng.gen_range(0.0001..1.0);
                let new_threshold = (threshold * decrease).clamp(0.0, 1.0);
                let new_threshold = (new_threshold * 100.0).round() / 100.0;

                if let Some(item) = b.random_pick(nesting - 1, new_threshold) {
                    bag.push(item);
                }
            }
        }

        bag.choose(&mut self.rng).copied()
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

    fn clean(path: &'static str) -> &str {
        path.trim_matches(SEPARATOR)
    }
}
