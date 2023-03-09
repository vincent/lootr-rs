Lootr
=====

**Lootr** \lutʁ\ is a simple RPG-like looting system.

Lootr provides a way to organize data commonly named loot in a gameplay context.
It helps you to manage which items can be found, in a generic and statisticaly controlled system.

It is heavily inspired from the work of Bob Nystrom <http://journal.stuffwithstuff.com/2014/07/05/dropping-loot>

A JS version is available here <https://github.com/vincent/lootr>
<br>
A C# version is available here <https://github.com/Sparox/LootrCsharp>

In Lootr, lootables are organized in a tree of categories and items.
<pre>
ROOT
├─ Staff
├─ equipment
│  ├─ Glove
│  │  Boots
│  └─ leather
│     │  Jacket
│     └─ Pads
└─ weapons
   ├─ Bat
   └─ Knife
</pre>

Then, a collection of Drops describe how items are yield from a loot action.

```ignore
equipment: .5 chances, stack of 1
equipment: .2 chances, stack of 2
equipment: .1 chances, stack of 2
```

This might yield items in the equipment tree, for example
- 1 Boots, once every 2 rolls on average
- 2 Glove, once every 5 rolls
- 1 Knife, once every 10 rolls


Create a loot bag
=====

Create items.

```rust
use lootr::{Lootr, item::Item};
let mut loot = Lootr::new();

loot.add(
    Item::a("Berries")
);
```

Items can have properties.

```rust
use lootr::{Lootr, item::{Item, Props}};
let mut loot = Lootr::new();

loot.add(
    Item::from("crown", Props::from([
        ("strength", "10"),
        ("charisma", "+100")
    ]))
);
```

Each level is composed by a list of `.items` and nested `.branchs`.

Organize the loot repository by adding branchs

```rust
use lootr::Lootr;
let mut loot = Lootr::new();

let weapons = loot.add_branch("weapons", Lootr::new());
let armor = loot.add_branch("armor", Lootr::new());
```

Optionnaly with items

```rust
use lootr::{Lootr, item::Item};
let mut loot = Lootr::new();

loot.add_branch("weapons", Lootr::from(vec![
    Item::a("Staff"),
    Item::an("Uzi")
]));

loot.add_branch("armor", Lootr::from(vec![
    Item::a("Boots"),
    Item::a("Socks")
]));

loot.branch_mut("armor")
    .unwrap()
    .add_branch("leather", Lootr::from(vec![
        Item::a("Belt"),
        Item::a("Hat")
    ]));
```

Looting
=====

Loot against a loot table, described by a like the following.

```rust
use lootr::{ROOT, drops::Drop};

let drops = [
    Drop { path: ROOT, depth: 1, luck: 1.0, stack: 1..=1, modify: false },
];
```

A builder pattern is also available to ease drops creation.

 * [`path()`](crate::drops::DropBuilder::path) selects the root of this drop
 * [`depth()`](crate::drops::DropBuilder::depth) max depth to consider
 * [`luck()`](crate::drops::DropBuilder::luck) the luck we start with, will decrease at each sub tree
 * [`stack()`](crate::drops::DropBuilder::stack) the range of copies to yield

```rust
use lootr::{Lootr, item::Item, drops::DropBuilder};
let mut loot = Lootr::new();

loot.add_branch("weapons", Lootr::from(vec![
    Item::a("Staff"),
    Item::an("Uzi")
]));

loot.add_branch("armor", Lootr::from(vec![
    Item::a("Boots"),
    Item::a("Socks")
]));

let drops = [
    DropBuilder::new()
        .path("armor")
        .luck(1.0)
        .build(),

    DropBuilder::new()
        .path("weapons")
        .luck(1.0)
        .stack(1..=3)
        .build(),
];

// Loot your reward from a dead monster
let rewards = loot.loot(&drops);

// rewards = [ "Berries", "Plates", "Uzi", "Uzi", "Staff" ]
```

Seeded RNG
=====

`Lootr.loot_seeded()` takes a PRNG arguments to yield items in a consitent and reproductible way.

```rust
use lootr::{Lootr, item::Item, drops::DropBuilder};
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

let rng = &mut ChaCha20Rng::seed_from_u64(123);

(0..10).for_each(|_f| {
    let mut loot = Lootr::from(vec![
        Item::named("Socks"),
        Item::named("Boots"),
    ]);
    let drops = [DropBuilder::new().build()];

    loot.loot_seeded(&drops, rng);
    loot.loot_seeded(&drops, rng);

    // Will always loot Boots, then Socks, then Socks, then Boots ..
})
```

Modifiers
=====
The library includes a basic modifiers system.

Add some modifiers to affect the properties of each looted item with `addModifiers`.
* `name` modifier will be used as simple suffixes. Or, if it contains one or more `$property`, each property name will be replaced.
* other properties will be handled as the following
```ignore
loot.add({ name: "Staff", color: "golden" })
loot.addModifiers([
    { name:    "from the shadows" },
    { name:    "A $color $name from the gods" },
    { agility: 5 },
    { force:   "*2" },
    { intell:  "2-10" },
    { name:    "A $color $name from the gods", force: "+4" }
])
```

Then, at loot time:
```ignore
deadMonster.drops = [
    {from: "/equipment", stack:2, modify:true }
]

// Loot your reward from a dead monster
var rewards = loot.loot(deadMonster.drops)

rewards = [
    // some of these could drop
    {name:"Staff from the shadows"},
    {name:"Staff", intell: 6},
    {name:"A golden staff from the gods"},
    {name:"A golden staff from the gods", force:4 }
]
```

Tests
=====

`cargo test`

