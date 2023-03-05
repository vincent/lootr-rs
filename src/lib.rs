mod tests;

use std::{collections::HashMap, ops::RangeInclusive};
use rand::{seq::SliceRandom, Rng};

const ROOT: Option<&'static str> = None;
const DEFAULT_MODIFIER: Modifier = |i| i;

pub struct Lootr {
    items: Vec<Item>,
    branchs: HashMap<String, Lootr>,
    modifiers: Vec<Modifier>,
}

impl Lootr {

    /// Create a new lootbag
    ///
    pub fn new() -> Self {
        Self {
            items: vec![],
            branchs: HashMap::new(),
            modifiers: vec![DEFAULT_MODIFIER],
        }
    }

    /// Create a new lootbag from given items
    ///
    /// * `items` A Vec of Items
    ///
    pub fn from_items(items: Vec<Item>) -> Self {
        Self {
            items: items,
            branchs: HashMap::new(),
            modifiers: vec![DEFAULT_MODIFIER],
        }
    }

    /// Return this lootbag branchs
    ///
    pub fn branchs(&self) -> &HashMap<String, Lootr> {
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
       self.items.len().clone()
    }

    /// Return this lootbag items count (including any sublevel)
    ///
    pub fn all_count(&self) -> usize {
       self.all_items().len().clone()
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
        match self.branch(path) {
            None => panic!("this path does not exist"),
            Some(branch) => branch.add(item),
        };

        self
    }

    /// Returns the branch at the given path.
    /// If the branch does not exit yet, it will be created.
    ///
    /// * `path` Branch path
    /// 
    pub fn branch(&mut self, path: &'static str) -> Option<&mut Lootr> {
        let cname = Self::clean(path);

        // simple case
        if self.branchs.contains_key(&cname) {
            return self.branchs.get_mut(&cname);
        }

        if !cname.contains("/") {
            return None
        }

        // segmented path
        let leaf = path
            .trim_matches('/')
            .split("/")
            .fold(self, |acc, s| acc.branch(s).unwrap());

        Some(leaf)
    }

    /// Returns the branch at the given path.
    /// If the branch does not exit yet, `None` is returned
    ///
    /// * `path` Branch path
    /// 
    pub fn branch_immutable(&self, path: &'static str) -> Option<&Lootr> {
        let cname = Self::clean(path);

        // simple case
        if self.branchs.contains_key(&cname) {
            return self.branchs.get(&cname);
        }

        if !cname.contains("/") {
            return None
        }

        // segmented path
        let leaf = path
            .trim_matches('/')
            .split("/")
            .fold(self, |acc, s| acc.branch_immutable(s).unwrap());

        Some(leaf)
    }

    pub fn add_branch(&mut self, path: &'static str, branch: Lootr) -> &mut Self {
        self.branchs.insert(String::from(path), branch);
        self.branchs.get_mut(path);

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
    pub fn roll(&self, catalog_path: Option<&'static str>, nesting: i16, threshold: f32) -> Option<&Item> {
        let branch = match catalog_path {
            None => self,
            Some(path) => self.branch_immutable(path).unwrap()
        };

        branch
            .random_pick(nesting, threshold)
            .to_owned()
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

            (0..stack)
                .for_each(|_s| {
                    let mut citem = item.unwrap().clone();

                    if d.modify {
                        self.random_modifier()(&mut citem);
                    }

                    rewards.push(citem)
                });
        }

        rewards
    }

    fn random_pick(&self, nesting: i16, threshold: f32) -> Option<&Item> {
        let mut bag = vec![];
        let rng = &mut rand::thread_rng();

        if rng.gen::<f32>() < threshold && self.items.len() > 0 {
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

    fn random_modifier(&self) -> &Modifier {
        self.modifiers.choose(&mut rand::thread_rng()).unwrap()
    }

    fn clean(path: &'static str) -> String {
        String::from(path.trim_matches('/'))
    }    
}


pub type Props = Option<HashMap<&'static str, &'static str>>;
pub type Modifier = fn(item: &mut Item) -> &Item;

#[derive(Clone)]
pub struct Item {
    pub name: &'static str,
    pub props: Option<Props>,
}
impl Item {
    pub fn a(name: &'static str) -> Self {
        Self {
            name: name,
            props: None,
        }
    }

    pub fn an(name: &'static str) -> Self {
        Item::a(name)
    }
    
    pub fn named(name: &'static str) -> Self {
        Item::a(name)
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
impl Drop {
    pub fn from(path: &'static str) -> Self {
        Self {
            from: Some(path),
            luck: 1.0,
            modify: false,
            depth: 1,
            stack: 1..=1
        }
    }

    pub fn any() -> Self {
        Self {
            from: ROOT,
            luck: f32::MAX,
            modify: false,
            depth: i16::MAX,
            stack: 1..=1
        }
    }

    pub fn modify(mut self) -> Self {
        self.modify = true;
        self
    }

    pub fn stack_of(mut self, stack: RangeInclusive<u32>) -> Self {
        self.stack = stack;
        self
    }

    pub fn luck_of(mut self, luck: f32) -> Self {
        self.luck = luck;
        self
    }

    pub fn depth(mut self, depth: i16) -> Self {
        self.depth = depth;
        self
    }
}

struct DropBuilder {
    pub from: Option<&'static str>,
    pub luck: f32,
    pub modify: bool,
    pub depth: i16,
    pub stack: RangeInclusive<u32>,
}

impl DropBuilder {
    fn new() -> DropBuilder {
        DropBuilder {
            from: ROOT,
            luck: f32::MAX,
            modify: false,
            depth: 1,
            stack: 1..=1
        }
    }
    fn from(mut self, path: &'static str) -> DropBuilder {
        self.from = Some(path);
        self
    }
    fn luck(mut self, luck: f32) -> DropBuilder {
        self.luck = luck;
        self
    }
    fn anydepth(mut self) -> DropBuilder {
        self.depth = i16::MAX;
        self
    }
    fn depth(mut self, depth: i16) -> DropBuilder {
        self.depth = depth;
        self
    }
    fn build(&self) -> Drop {
        Drop {
            from: self.from,
            luck: self.luck,
            modify: self.modify,
            depth: self.depth,
            stack: self.stack.clone(),
        }
    }
}
