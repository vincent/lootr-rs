Lootr
=====

A simple RPG-like looting system.

Heavily inspired from http://journal.stuffwithstuff.com/2014/07/05/dropping-loot

A JS version is available here https://github.com/vincent/lootr

A C# version is available here https://github.com/Sparox/LootrCsharp

Adding items
=====

Lootr is organized as a tree. Get a new instance, and add items.

```rust
let mut loot = Lootr::default();

loot.add(Item::a("Berries"));
```

Each level is composed by a list of items in `loot.items` and nested branchs in `loot.branchs`.

Organize the loot repository by adding branchs

```rust
let weapons = loot.add_branch("weapons")
let armor = loot.add_branch("armor")
```

Optionnaly with items

```rust
loot.add_branch("weapons", Lootr::from_items(vec![
    Item::a("Staff"),
    Item::an("Uzi")
]));

loot.add_branch("armor", Lootr::from_items(vec![
    Item::a("Boots"),
    Item::a("Socks")
]));
loot.branch("armor")
    .unwrap().add_branch("leather", Lootr::from_items(vec![
        Item::a("Belt"),
        Item::a("Hat")
    ]));
```

Rollin'
=====

Random-pick some items.

It will yield an item in the `path` branch or, if `depth` is given, in an up to `depth` deep branchs, if the depth-decreasing `chance` is greater than a random 0..1

```rust
// Loot something from top level
loot.roll(ROOT, 0, 1.0)               // only Berries

// Loot something from top level or from any subtree
loot.roll_any(ROOT)

// Loot a weapon
loot.roll(Some("/weapons"), 1, 1.0)   // one of [ Pistol, Uzi ]

// Loot an armor
loot.roll(Some("/armor"), 1, 1.0)     // one of [ Boots, Socks ]
loot.roll(Some("/armor"), 2, 1.0)     // one of [ Boots, Socks, Belt, Hat ]

```

Lootin'
=====

Loot against a loot table, described by a definition array like the following. The string stack value allow random stacks in the specified range.

```rust
let drops = [
    Drop { from: ROOT, luck: 1.0, depth: 1, stack: 1..=1, modify: false },
    DropBuilder::new().from("armor").luck(1.0).build(),
    DropBuilder::new().from("weapons").luck(1.0).stack(1..=3).build(),
];

// Loot your reward from a dead monster
let rewards = loot.loot(&drops)

rewards = [ "Berries", "Plates", "Uzi", "Uzi", "Staff" ]
```

Modifiers
=====
The library includes a basic modifiers system.

Add some modifiers to affect the properties of each looted item with `addModifiers`.
* `name` modifier will be used as simple suffixes. Or, if it contains one or more `$property`, each property name will be replaced.
* other properties will be handled as the following
```rust
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
```rust
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

