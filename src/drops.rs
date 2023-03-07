//! Module containing Drop types used in Lootr.
//!
//! Drops describe what _should_ be looted from a bag in Lootr.
//! It mainly holds a `path`, `depth` and a `stack`.
//!
//! When used in [`Lootr::loot()`](crate::Lootr::loot), it should yield items as decribed by the Drop object.
//!
//! The easiest way to create a Drop is to use [`DropBuilder`](crate::drops::DropBuilder), the Lootr builder pattern for Drop.
//!

use crate::ROOT;
use std::ops::RangeInclusive;

/// Holds a Lootr Drop.
///
/// Drops describe what _should_ be looted from a Lootr bag.
/// It mainly holds a `path`, `depth` and a `stack`.
///
/// When used in [`Lootr::loot()`](crate::Lootr::loot), loot() should yield items as decribed by each Drop object.
///
/// The easiest way to create a Drop is to use [`DropBuilder`](crate::drops::DropBuilder), the Lootr builder pattern for Drop.
///
#[derive(Clone)]
pub struct Drop {
    /// Holds the root path to drop from.
    ///
    pub path: Option<&'static str>,

    /// Holds the drop starting depth.
    /// Will decrease at each visited sub-branch.
    ///
    pub depth: i16,

    /// Holds the drop starting luck.
    /// Will decrease at each visited sub-branch.
    ///
    pub luck: f32,

    /// Holds the drop stack range.
    ///
    pub stack: RangeInclusive<u32>,

    /// If true, will yield modified Items.
    /// See [Modifiers](crate::Modifier)
    ///
    pub modify: bool,
}

impl Default for Drop {
    fn default() -> Self {
        Self {
            path: ROOT,
            depth: 1,
            luck: 1.0,
            stack: 1..=1,
            modify: false,
        }
    }
}

/// The Lootr Drop factory.
///
/// DropBuilder creates [`Drop`](crate::drops::Drop) object in a functional programming oriented way.
///
pub struct DropBuilder {
    pub path: Option<&'static str>,
    pub depth: i16,
    pub luck: f32,
    pub stack: RangeInclusive<u32>,
    pub modify: bool,
}

impl Default for DropBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DropBuilder {
    pub fn new() -> DropBuilder {
        DropBuilder {
            path: ROOT,
            depth: 1,
            luck: f32::MAX,
            stack: 1..=1,
            modify: false,
        }
    }

    /// Set the `path` for the future [`Drop`](crate::drops::Drop) object.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::drops::DropBuilder;
    ///
    /// let drop = DropBuilder::new()
    ///     .path("fruits")
    ///     .build();
    ///
    /// assert_eq!(drop.path, Some("fruits"));
    /// ```
    pub fn path(mut self, path: &'static str) -> DropBuilder {
        self.path = Some(path);
        self
    }

    /// Set the `luck` for the future [`Drop`](crate::drops::Drop) object.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::drops::DropBuilder;
    ///
    /// let drop = DropBuilder::new()
    ///     .luck(0.9)
    ///     .build();
    ///
    /// assert_eq!(drop.luck, 0.9);
    /// ```
    pub fn luck(mut self, luck: f32) -> DropBuilder {
        self.luck = luck;
        self
    }

    /// Set the `depth` for the future [`Drop`](crate::drops::Drop) object.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::drops::DropBuilder;
    ///
    /// let drop = DropBuilder::new()
    ///     .depth(3)
    ///     .build();
    ///
    /// assert_eq!(drop.depth, 3);
    /// ```
    pub fn depth(mut self, depth: i16) -> DropBuilder {
        self.depth = depth;
        self
    }

    /// Use the max depth value for the future [`Drop`](crate::drops::Drop) object.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::drops::DropBuilder;
    ///
    /// let drop = DropBuilder::new()
    ///     .anydepth()
    ///     .build();
    ///
    /// assert_eq!(drop.depth, i16::MAX);
    /// ```
    pub fn anydepth(mut self) -> DropBuilder {
        self.depth = i16::MAX;
        self
    }

    /// Set the `stack` for the future [`Drop`](crate::drops::Drop) object.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::drops::DropBuilder;
    ///
    /// let drop = DropBuilder::new()
    ///     .stack(1..=3)
    ///     .build();
    /// ```
    pub fn stack(mut self, stack: RangeInclusive<u32>) -> DropBuilder {
        self.stack = stack;
        self
    }

    /// Finish a build sequence, and create a [`Drop`](crate::drops::Drop) object.
    ///
    /// # Examples
    ///
    /// ```
    /// use lootr::{item::{Item, Props}, drops::DropBuilder};
    ///
    /// let drop = DropBuilder::new()
    ///     .path("fruits")
    ///     .depth(3)
    ///     .luck(0.9)
    ///     .build();
    ///
    /// assert_eq!(drop.path, Some("fruits"));
    /// assert_eq!(drop.depth, 3);
    /// assert_eq!(drop.luck, 0.9);
    /// ```
    pub fn build(&self) -> Drop {
        Drop {
            path: self.path,
            depth: self.depth,
            luck: self.luck,
            stack: self.stack.clone(),
            modify: self.modify,
        }
    }
}
