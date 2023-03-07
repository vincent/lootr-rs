use crate::ROOT;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Drop {
    pub from: Option<&'static str>,
    pub luck: f32,
    pub modify: bool,
    pub depth: i16,
    pub stack: RangeInclusive<u32>,
}

impl Default for DropBuilder {
    fn default() -> Self {
        Self::new()
    }
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
