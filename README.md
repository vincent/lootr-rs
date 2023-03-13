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

let item = Item::from("crown", Props::from([
    ("strength", "10"),
    ("charisma", "+100")
]));

loot.add(item);

// Items can printed
// crown{strength=10,charisma=+100}

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

(0..10).for_each(|_f| {
    let mut loot = Lootr::from(vec![
        Item::named("Socks"),
        Item::named("Boots"),
    ]);
    let drops = [DropBuilder::new().build()];

    let rng = &mut ChaCha20Rng::seed_from_u64(123);

    loot.loot_seeded(&drops, rng);
    loot.loot_seeded(&drops, rng);
    // ...

    // Will always loot Boots, then Socks, then Socks, then Boots ..
})
```

Modifiers
=====

`Lootr.add_modifier()` allows to give some Item transformers, call Modifiers.

Modifiers are simple functions that return a new Item from a given one.

```rust
use lootr::{Lootr, item::{Item, Props}, drops::DropBuilder};
let mut loot = Lootr::new();
loot.add(Item::a("crown"));

fn with_strength(source: Item) -> Item {
    source.extend(source.name, Props::from([
        ("strength", "10"),
    ]))
}

loot.add_modifier(with_strength);

//
// Then, at loot time:

let drops = [DropBuilder::new().modify().build()];

let rewards = loot.loot(&drops);

// rewards = [ crown{strength=10} ]
```

Macros
=====

To make tha building easier, you can use the `bag!` macro.

```rust
use lootr::{bag, Lootr, item::{Item, Props}};
let loot = bag! {
    @Weapons
        Knife attack="1" desc="A simple knife",
        @Wooden
            BarkShield attack="0" magic_power="10" desc="A wooden shield reinforced with bark, providing magic power",
            @Staffs
                WoodenStaff attack="5" magic_power="10" desc="A wooden staff imbued with magic power",
                CrystalStaff attack="8" magic_power="15" ice_damage="10" desc="A crystal staff with ice elemental damage",
                ElementalStaff attack="12" magic_power="20" thunder_damage="15" desc="An elemental staff with thunder elemental damage",
                .
            @Bows
                ShortBow attack="10" accuracy="10" desc="A short bow with high accuracy",
                LongBow attack="20" accuracy="20" ice_damage="10" desc="A long bow with ice elemental damage",
                .
            .
        @Swords
            ShortSword attack="10" critical="5" desc="A short sword with increased critical hit rate",
            LongSword attack="15" critical="10" desc="A long sword with a high critical hit rate",
            TwoHandedSword attack="20" critical="15" desc="A two-handed sword with a very high critical hit rate",
            .
        @Axes
            BattleAxe attack="12" critical="8" desc="A battle axe with increased critical hit rate",
            WarAxe attack="14" critical="9" desc="A war axe with a high critical hit rate",
            .
        @Mace
            MorningStar attack="13" critical="7" desc="A mace with increased critical hit rate",
            Flail attack="16" critical="11" desc="A flail with a very high critical hit rate",
            .
        .
    @Armors
        Shirt defense="0" desc="A simple shirt",
        @LightArmor
            LeatherArmor defense="5" agility="2" desc="Armor made of leather with increased agility",
            Chainmail defense="8" agility="1" desc="Armor made of interlocking rings with moderate agility",
            .
        @HeavyArmor
            PlateArmor defense="10" agility="-2" desc="Heavy armor made of plates with decreased agility",
            FullPlateArmor defense="15" agility="-5" desc="Very heavy armor made of plates with greatly decreased agility",
            .
        .
    @Consumables
        Water healing="2" desc="Just water",
        @Potion
            HealthPotion healing="20" desc="A potion that restores a small amount of health",
            GreaterHealthPotion healing="40" desc="A potion that restores a moderate amount of health",
            ManaPotion mana_restoration="20" desc="A potion that restores a small amount of mana",
            GreaterManaPotion mana_restoration="40" desc="A potion that restores a moderate amount of mana",
            .
        @Elixirs
            ElixirOfStrength strength_boost="5" desc="An elixir that boosts strength",
            GreaterElixirOfStrength strength_boost="10" desc="An elixir that greatly boosts strength",
            ElixirOfAgility agility_boost="5" desc="An elixir that boosts agility",
            GreaterElixirOfAgility agility_boost="10" desc="An elixir that greatly boosts agility",
            .
        .
};

println!("{}", loot);
```

```ignore
ROOT
 ├─ test{}
 ├─ Armors
 │  ├─ Shirt{defense="0",desc="A simple shirt"}
 │  ├─ HeavyArmor
 │  │  └─ PlateArmor{agility="-2",defense="10",desc="Heavy armor made of plates with decreased agility"}
 │  │     FullPlateArmor{agility="-5",defense="15",desc="Very heavy armor made of plates with greatly decreased agility"}
 │  └─ LightArmor
 │     └─ LeatherArmor{defense="5",desc="Armor made of leather with increased agility",agility="2"}
 │        Chainmail{agility="1",defense="8",desc="Armor made of interlocking rings with moderate agility"}
 ├─ Consumables
 │  ├─ Water{desc="Just water",healing="2"}
 │  ├─ Elixirs
 │  │  └─ ElixirOfStrength{strength_boost="5",desc="An elixir that boosts strength"}
 │  │     GreaterElixirOfStrength{strength_boost="10",desc="An elixir that greatly boosts strength"}
 │  │     ElixirOfAgility{agility_boost="5",desc="An elixir that boosts agility"}
 │  │     GreaterElixirOfAgility{desc="An elixir that greatly boosts agility",agility_boost="10"}
 │  └─ Potion
 │     └─ HealthPotion{desc="A potion that restores a small amount of health",healing="20"}
 │        GreaterHealthPotion{desc="A potion that restores a moderate amount of health",healing="40"}
 │        ManaPotion{mana_restoration="20",desc="A potion that restores a small amount of mana"}
 │        GreaterManaPotion{desc="A potion that restores a moderate amount of mana",mana_restoration="40"}
 └─ Weapons
    ├─ Knife{desc="A simple knife",attack="1"}
    ├─ Axes
    │  └─ BattleAxe{attack="12",critical="8",desc="A battle axe with increased critical hit rate"}
    │     WarAxe{attack="14",desc="A war axe with a high critical hit rate",critical="9"}
    ├─ Mace
    │  └─ MorningStar{attack="13",critical="7",desc="A mace with increased critical hit rate"}
    │     Flail{desc="A flail with a very high critical hit rate",attack="16",critical="11"}
    ├─ Swords
    │  └─ ShortSword{critical="5",desc="A short sword with increased critical hit rate",attack="10"}
    │     LongSword{desc="A long sword with a high critical hit rate",attack="15",critical="10"}
    │     TwoHandedSword{attack="20",desc="A two-handed sword with a very high critical hit rate",critical="15"}
    └─ Wooden
       ├─ BarkShield{attack="0",magic_power="10",desc="A wooden shield reinforced with bark, providing magic power"}
       ├─ Bows
       │  └─ ShortBow{accuracy="10",attack="10",desc="A short bow with high accuracy"}
       │     LongBow{desc="A long bow with ice elemental damage",attack="20",ice_damage="10",accuracy="20"}
       └─ Staffs
          └─ WoodenStaff{desc="A wooden staff imbued with magic power",magic_power="10",attack="5"}
             CrystalStaff{ice_damage="10",desc="A crystal staff with ice elemental damage",magic_power="15",attack="8"}
             ElementalStaff{thunder_damage="15",desc="An elemental staff with thunder elemental damage",magic_power="20",attack="12"}
```

Tests
=====

`cargo test`

Bump version
=====

`cargo bump minor`

