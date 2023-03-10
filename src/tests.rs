#[cfg(test)]
mod tests {
    use crate::{
        bag,
        drops::{Drop, DropBuilder},
        item::Props,
        Item, Lootr, ROOT,
    };
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use std::{collections::HashMap, fmt};

    #[test]
    fn success_item() {
        let item = Item::from(
            "crown",
            Props::from([("strength", "10"), ("charisma", "+100")]),
        );

        assert_eq!(item.has_prop("strength"), true);
        assert_eq!(item.get_prop("strength").unwrap(), "10");
    }

    #[test]
    fn success_item_display() {
        let item = Item::from(
            "crown",
            Props::from([("strength", "10"), ("charisma", "+100")]),
        );
        // println!("{}", item);
        let output = fmt::format(format_args!("{}", item));
        assert_eq!(
            output == "crown{strength=10,charisma=+100}"
                || output == "crown{charisma=+100,strength=10}",
            true
        );
    }

    #[test]
    fn success_from() {
        let loot = Lootr::from(vec![Item::a("Staff"), Item::an("Uzi")]);
        assert_eq!(loot.items().len(), 2);
        assert_eq!(loot.self_count(), 2);
    }

    #[test]
    fn success_bag_macro() {
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
        assert_eq!(loot.all_count(), 29);
    }

    #[test]
    fn success_branch_display() {
        // println!("{}", stuffed());
        let output = fmt::format(format_args!("{}", stuffed()));
        assert_eq!(output.split("???").count(), 10);
    }

    #[test]
    fn success_add_item() {
        let mut loot = Lootr::new();

        loot.add(Item::a("Staff"));

        assert_eq!(loot.self_count(), 1);
    }

    #[test]
    fn success_add_branch() {
        let mut loot = Lootr::new();

        loot.add_branch(
            "weapons",
            Lootr::from(vec![Item::a("Staff"), Item::an("Uzi")]),
        );

        loot.branch_mut("weapons").unwrap().add_branch(
            "leather",
            Lootr::from(vec![Item::a("Boots"), Item::a("Cap")]),
        );
    }

    #[test]
    fn success_get_branch() {
        let mut loot = Lootr::new();
        let mut weapons = Lootr::new();
        let mut deadly = Lootr::new();
        let mut fire = Lootr::new();

        fire.add(Item::an("Uzi"));

        deadly.add_branch("fire", fire);
        weapons.add_branch("deadly", deadly);
        loot.add_branch("weapons", weapons);

        let fire_branch = loot.branch_mut("weapons/deadly/fire");
        assert_eq!(fire_branch.unwrap().self_count(), 1);
    }

    #[test]
    fn success_add_item_in_branch() {
        let mut loot = Lootr::new();

        let weapons = Lootr::new();
        loot.add_branch("weapons", weapons);

        loot.add_in(Item::an("Uzi"), "weapons");

        assert_eq!(loot.all_items().len(), 1);
        assert_eq!(loot.all_count(), 1);
    }

    #[test]
    fn success_get_all_items() {
        let mut loot = Lootr::from(vec![Item::a("Staff")]);

        loot.add_branch(
            "weapons",
            Lootr::from(vec![Item::a("Bat"), Item::an("Uzi")]),
        );

        assert_eq!(loot.all_items().len(), 3);
    }

    #[test]
    fn success_roll_root() {
        let loot = stuffed();

        assert_eq!(
            loot.roll(ROOT, 0, 1.0).unwrap().name,
            "Staff",
            "Should return the only element of root"
        );
    }

    #[test]
    fn success_roll_any() {
        let loot = stuffed();
        let picked = loot.roll_any().unwrap();

        let expected = ["Staff", "Bat", "Uzi", "Gloves", "Boots", "Jacket", "Pads"];
        assert_eq!(
            expected.contains(&picked.name),
            true,
            "Should return any element"
        );
    }

    #[test]
    fn success_roll_any_seeded() {
        (1..9).for_each(|i| {
            let loot = stuffed();
            let first_picked = loot
                .roll_seeded(
                    ROOT,
                    i16::MAX,
                    1.0,
                    &mut ChaCha20Rng::seed_from_u64(123 * i),
                )
                .unwrap();

            (1..9).for_each(|_| {
                let nloot = stuffed();
                let picked = nloot
                    .roll_seeded(
                        ROOT,
                        i16::MAX,
                        1.0,
                        &mut ChaCha20Rng::seed_from_u64(123 * i),
                    )
                    .unwrap();

                assert_eq!(
                    &picked.name, &first_picked.name,
                    "Should return the same elements"
                );
            });
        })
    }

    #[test]
    fn success_roll_any_depth1() {
        let loot = stuffed();
        let picked = loot.roll(ROOT, 1, 1.0).unwrap();

        let expected = ["Staff", "Bat", "Uzi", "Gloves", "Boots"];
        assert_eq!(
            expected.contains(&picked.name),
            true,
            "Should return a depth1 element"
        );
    }

    #[test]
    fn success_roll_any_depth1_branched() {
        let loot = stuffed();
        let picked = loot.roll(Some("/equipment/leather"), 0, 1.0).unwrap();

        let expected = ["Jacket", "Pads"];
        assert_eq!(
            expected.contains(&picked.name),
            true,
            "Should return a depth1 element"
        );
    }

    #[test]
    fn success_loot_any() {
        let loot = stuffed();

        let drops = [
            Drop {
                path: ROOT,
                luck: 1.0,
                depth: 1,
                stack: 1..=1,
                modify: false,
            },
            DropBuilder::new().path("equipment").luck(1.0).build(),
            DropBuilder::new().path("weapons").luck(1.0).build(),
        ];

        let rewards = loot.loot(&drops);

        assert_eq!(rewards.len() >= 3, true, "Should reward at least 3 items");
    }

    #[test]
    fn success_loot_stats() {
        let loot = stuffed();

        let luck_for_equipment = 0.3;
        let luck_for_weapons = 0.8;
        let drops = [
            DropBuilder::new()
                .path("equipment")
                .luck(luck_for_equipment)
                .anydepth()
                .build(),
            DropBuilder::new()
                .path("weapons")
                .luck(luck_for_weapons)
                .anydepth()
                .build(),
        ];

        let rolls = 100_000;
        let f_rolls: f64 = Into::<f64>::into(rolls);
        let mut overall_count = 0;
        let mut overall_rewards = HashMap::<&'static str, i32>::new();

        (0..rolls).for_each(|_| {
            loot.loot(&drops).iter().for_each(|r| {
                let current = match overall_rewards.get(r.name) {
                    Some(number) => number.clone(),
                    None => 0,
                };
                overall_rewards.insert(r.name, current + 1);
                overall_count += 1;
            })
        });

        let gloves = overall_rewards.get("Gloves");
        let boots = overall_rewards.get("Boots");
        let jacket = overall_rewards.get("Jacket");
        let pads = overall_rewards.get("Pads");
        let armband = overall_rewards.get("ArmBand");
        let patch = overall_rewards.get("Patch");
        let bat = overall_rewards.get("Bat");
        let uzi = overall_rewards.get("Uzi");

        assert_ne!(gloves, None, "There should be some Gloves");
        assert_ne!(boots, None, "There should be some Boots");
        assert_ne!(jacket, None, "There should be some Jacket");
        assert_ne!(pads, None, "There should be some Pads");
        assert_ne!(armband, None, "There should be some ArmBand");
        assert_ne!(patch, None, "There should be some Patch");
        assert_ne!(bat, None, "There should be some Bat");
        assert_ne!(uzi, None, "There should be some Uzi");

        let zero = &0;
        let equipment = 0
            + overall_rewards.get("Gloves").unwrap_or(zero)
            + overall_rewards.get("Boots").unwrap_or(zero)
            + overall_rewards.get("Jacket").unwrap_or(zero)
            + overall_rewards.get("Pads").unwrap_or(zero)
            + overall_rewards.get("ArmBand").unwrap_or(zero)
            + overall_rewards.get("Patch").unwrap_or(zero);

        let weapons = 0
            + overall_rewards.get("Bat").unwrap_or(zero)
            + overall_rewards.get("Uzi").unwrap_or(zero);

        assert_eq!(equipment + weapons, overall_count);

        let theory = f_rolls * Into::<f64>::into(luck_for_equipment);
        let expected_equipment = (theory * 0.7)..(theory * 1.6);

        let theory = f_rolls * Into::<f64>::into(luck_for_weapons);
        let expected_weapons = (theory * 0.7)..(theory * 1.6);

        assert_eq!(
            expected_equipment.contains(&equipment.into()),
            true,
            "There should be enough equipment"
        );
        assert_eq!(
            expected_weapons.contains(&weapons.into()),
            true,
            "There should be enough weapons"
        );
    }

    #[test]
    fn success_loot_seeded() {
        let loot = stuffed();
        let drops = [
            DropBuilder::new().path("equipment").anydepth().build(),
            DropBuilder::new().path("weapons").anydepth().build(),
        ];

        let rewards = loot.loot_seeded(&drops, &mut ChaCha20Rng::seed_from_u64(123));

        (0..10).for_each(|_| {
            let nloot = stuffed();
            let nrewards = nloot.loot_seeded(&drops, &mut ChaCha20Rng::seed_from_u64(123));

            nrewards.iter().enumerate().for_each(|(i, r)| {
                assert_eq!(
                    r.name,
                    rewards.get(i).unwrap().name,
                    "Should return same elements"
                )
            });
        });
    }

    #[test]
    fn success_loot_simple_modifier() {
        let mut loot = Lootr::new();

        fn with_strength(source: Item) -> Item {
            source.extend(source.name, Props::from([("strength", "+10")]))
        }

        loot.add_modifier(with_strength).add(Item::a("crown"));

        let picked = loot.loot(&[
            Drop {
                path: ROOT,
                luck: 1.0,
                depth: 1,
                stack: 1..=1,
                modify: false,
            },
            Drop {
                path: ROOT,
                luck: 1.0,
                depth: 1,
                stack: 1..=1,
                modify: true,
            },
        ]);

        let first = &picked.first().unwrap().clone();
        let last = &picked.last().unwrap().clone();

        assert_eq!(first.has_prop("strength"), false);

        assert_eq!(last.has_prop("strength"), true);
        assert_eq!(last.get_prop("strength").unwrap().to_owned(), "+10");
    }

    ////////////////////////////////////////////////////

    fn stuffed<'a>() -> Lootr<'a> {
        let mut loot = Lootr::from(vec![Item::a("Staff")]);

        loot.add_branch(
            "weapons",
            Lootr::from(vec![Item::a("Bat"), Item::an("Uzi")]),
        );

        loot.add_branch(
            "equipment",
            Lootr::from(vec![Item::a("Gloves"), Item::a("Boots")]),
        );

        loot.branch_mut("equipment").unwrap().add_branch(
            "leather",
            Lootr::from(vec![Item::a("Jacket"), Item::a("Pads")]),
        );

        loot.branch_mut("equipment/leather").unwrap().add_branch(
            "Scraps",
            Lootr::from(vec![Item::a("ArmBand"), Item::a("Patch")]),
        );

        loot
    }
}
