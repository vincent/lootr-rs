#![doc = include_str!("../README.md")]

pub mod drops;
pub mod item;
mod tests;

use ascii_tree::{
    write_tree,
    Tree::{Leaf, Node},
};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::{collections::BTreeMap, fmt};

use crate::{
    drops::Drop,
    item::{Item, Modifier},
};

pub const ROOT: Option<&str> = None;
const SEPARATOR: char = '/';

pub struct Lootr<'a> {
    items: Vec<Item<'a>>,
    branchs: BTreeMap<&'a str, Lootr<'a>>,
    modifiers: Vec<Modifier>,
}

impl<'a> fmt::Display for Lootr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_tree(f, &self.fmt_node("ROOT"))
    }
}

impl<'a> Lootr<'a> {
    /// Create a new lootbag
    ///
    pub fn new() -> Self {
        Self::from(vec![])
    }

    /// Create a new lootbag from given items
    ///
    pub fn from(items: Vec<Item<'a>>) -> Self {
        Self {
            items,
            branchs: BTreeMap::new(),
            modifiers: vec![],
        }
    }

    /// Return this lootbag branchs
    ///
    pub fn branchs(&self) -> &BTreeMap<&str, Lootr<'a>> {
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
    pub fn add(&mut self, item: Item<'a>) -> &mut Self {
        self.items.push(item);

        self
    }

    /// Add an item in the given branch
    ///
    /// Returns the current lootbag
    ///
    pub fn add_in(&mut self, item: Item<'a>, path: &'a str) -> &mut Self {
        match self.branch_mut(path) {
            None => panic!("this path does not exist"),
            Some(branch) => branch.add(item),
        };

        self
    }

    /// Returns the branch at the given path.
    ///
    pub fn branch_mut(&mut self, path: &'a str) -> Option<&mut Lootr<'a>> {
        let cname = path.trim_matches(SEPARATOR);

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
    pub fn branch(&self, path: &'a str) -> Option<&Lootr<'a>> {
        let cname = path.trim_matches(SEPARATOR);

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
    pub fn add_branch(&mut self, path: &'a str, branch: Lootr<'a>) -> &mut Self {
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

    /// Add a modifier
    ///
    pub fn add_modifier(&mut self, modifier: Modifier) -> &mut Self {
        self.modifiers.push(modifier);
        self
    }

    /// Pick a random item from the specified branch
    ///
    /// Returns `Some(Item)` or `None`
    ///
    pub fn roll(
        &self,
        catalog_path: Option<&'a str>,
        nesting: i16,
        threshold: f32,
    ) -> Option<&Item> {
        self.roll_seeded(
            catalog_path,
            nesting,
            threshold,
            &mut ChaCha20Rng::from_entropy(),
        )
    }

    /// Pick a random item from the specified branch, given a PRNG
    ///
    /// Returns `Some(Item)` or `None`
    ///
    pub fn roll_seeded<R>(
        &self,
        catalog_path: Option<&'a str>,
        nesting: i16,
        threshold: f32,
        rng: &mut R,
    ) -> Option<&Item<'a>>
    where
        R: Rng + ?Sized,
    {
        let branch = match catalog_path {
            None => self,
            Some(path) => self.branch(path).unwrap(),
        };

        branch.random_pick(nesting, threshold, rng)
    }

    /// Pick a random item anywhere in that branch
    ///
    /// Returns `Some(Item)` or `None`
    ///
    pub fn roll_any(&self) -> Option<&Item> {
        self.roll_seeded(ROOT, i16::MAX, 1.0, &mut ChaCha20Rng::from_entropy())
    }

    /// Roll against a looting table
    ///
    /// Returns a vec of Item
    ///
    pub fn loot(&self, drops: &[Drop]) -> Vec<Item> {
        self.loot_seeded(drops, &mut ChaCha20Rng::from_entropy())
    }

    /// Roll against a looting table, given a PRNG
    ///
    /// Returns a vec of Item
    ///
    pub fn loot_seeded<R>(&self, drops: &[Drop], rng: &mut R) -> Vec<Item>
    where
        R: Rng + ?Sized,
    {
        let mut rewards: Vec<Item> = vec![];

        for d in drops {
            let item = self.roll_seeded(d.path, d.depth, d.luck, rng);

            if item.is_none() {
                continue;
            }

            let citem: Item = item.unwrap().clone();
            let stack_max = rng.gen_range(d.stack.clone());

            rewards.append(
                &mut (0..stack_max)
                    .map(|_| {
                        if !self.modifiers.is_empty() && d.modify {
                            let modifier = self.modifiers.choose(rng).unwrap();
                            modifier(citem.clone())
                        } else {
                            citem.clone()
                        }
                    })
                    .collect::<Vec<Item>>(),
            );
        }

        rewards
    }

    fn random_pick<R>(&self, nesting: i16, threshold: f32, rng: &mut R) -> Option<&Item<'a>>
    where
        R: Rng + ?Sized,
    {
        let mut bag = vec![];

        if let Some(item) = self.items.choose(rng) {
            if rng.gen::<f32>() < threshold {
                bag.push(item);
            }
        }

        for b in self.branchs.values() {
            let decrease: f32 = rng.gen_range(0.0001..1.0);
            let new_threshold = (threshold * decrease).clamp(0.0, 1.0);
            let new_threshold = (new_threshold * 100.0).round() / 100.0;

            if nesting > 0 {
                if let Some(item) = b.random_pick(nesting - 1, new_threshold, rng) {
                    bag.push(item);
                }
            }
        }

        bag.choose(rng).copied()
    }

    fn fmt_node(&self, name: &str) -> ascii_tree::Tree {
        let mut children: Vec<ascii_tree::Tree> = vec![];

        children.push(Leaf(
            self.items()
                .iter()
                .map(|item| format!("{}", item))
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
}

#[macro_export]
macro_rules! a {
    ( $x:expr ) => {
        Item::name($x)
    }
}

#[macro_export]
macro_rules! bag {

    // ($(@ $b1:ident $($i1:ident $($a1:ident = $v1:expr) *;),* $(@$tail:meta |),* |)*) => {
    // ($(@ $branch:ident $($item:ident $($a1:ident = $v1:expr) *,);* |)*) => { // OK
    // ($(@ $branch:ident $($item:ident $($a1:ident = $v1:expr) *,);* $(@ $b2:ident $($i2:ident $($a2:ident = $v2:expr) *,);* |)* |)*) => { // OK
    ($
        (@ $b1:ident $($i1:ident $($a1:ident = $v1:expr) *,)* 
            $(@ $b2:ident $($i2:ident $($a2:ident = $v2:expr) *,)*
                $(@ $b3:ident $($i3:ident $($a3:ident = $v3:expr) *,)*
                .)*
            .)* 
        .)*
    ) => {
        {
            let mut loot = Lootr::new();
            loot.add(Item::named("test"));

            $( // for each $b1
                let mut b1 = Lootr::new();

                $( // for each $i1
                    let mut i1 = Item::named(stringify!($i1));
                    $( // for each $a1
                        i1.set_prop(stringify!($a1), stringify!($v1));
                    )*
                    b1.add(i1);
                )*

                $( // for each $b2
                    let mut b2 = Lootr::new();
    
                    $( // for each $i1
                        let mut i2 = Item::named(stringify!($i2));
                        $( // for each $a1
                            i2.set_prop(stringify!($a2), stringify!($v2));
                        )*
                        b2.add(i2);
                    )*

                    $( // for each $b3
                        let mut b3 = Lootr::new();
        
                        $( // for each $i3
        
                            let mut i3 = Item::named(stringify!($i3));
                            $( // for each $a3
                                i3.set_prop(stringify!($a3), stringify!($v3));
                            )*
                            b3.add(i3);
                        )*
        
                        b2.add_branch(stringify!($b3), b3);
                    )*
                    b1.add_branch(stringify!($b2), b2);
                )*
                loot.add_branch(stringify!($b1), b1);
            )*

            loot
        }
    };

    ($e:expr, $($es:expr),+) => {
        println("recursiooooooonnnn !!");
    };
}