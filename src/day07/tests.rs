use super::*;

const INPUT: &str = include_str!("../../input/2020/day7.txt");

const EXAMPLE1: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

const EXAMPLE2: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

#[test]
fn parse_bag_rules_from_example1() {
    let bag_rules = parse_bag_rules(EXAMPLE1);

    assert_eq!(
        bag_rules,
        vec![
            Bag {
                color_code: ColorCode {
                    modifier: "light",
                    color: "red"
                },
                contents: vec![
                    Content {
                        color_code: ColorCode {
                            modifier: "bright",
                            color: "white"
                        },
                        quantity: 1
                    },
                    Content {
                        color_code: ColorCode {
                            modifier: "muted",
                            color: "yellow"
                        },
                        quantity: 2
                    }
                ]
            },
            Bag {
                color_code: ColorCode {
                    modifier: "dark",
                    color: "orange"
                },
                contents: vec![
                    Content {
                        color_code: ColorCode {
                            modifier: "bright",
                            color: "white"
                        },
                        quantity: 3
                    },
                    Content {
                        color_code: ColorCode {
                            modifier: "muted",
                            color: "yellow"
                        },
                        quantity: 4
                    }
                ]
            },
            Bag {
                color_code: ColorCode {
                    modifier: "bright",
                    color: "white"
                },
                contents: vec![Content {
                    color_code: ColorCode {
                        modifier: "shiny",
                        color: "gold"
                    },
                    quantity: 1
                },]
            },
            Bag {
                color_code: ColorCode {
                    modifier: "muted",
                    color: "yellow"
                },
                contents: vec![
                    Content {
                        color_code: ColorCode {
                            modifier: "shiny",
                            color: "gold"
                        },
                        quantity: 2
                    },
                    Content {
                        color_code: ColorCode {
                            modifier: "faded",
                            color: "blue"
                        },
                        quantity: 9
                    }
                ]
            },
            Bag {
                color_code: ColorCode {
                    modifier: "shiny",
                    color: "gold"
                },
                contents: vec![
                    Content {
                        color_code: ColorCode {
                            modifier: "dark",
                            color: "olive"
                        },
                        quantity: 1
                    },
                    Content {
                        color_code: ColorCode {
                            modifier: "vibrant",
                            color: "plum"
                        },
                        quantity: 2
                    }
                ]
            },
            Bag {
                color_code: ColorCode {
                    modifier: "dark",
                    color: "olive"
                },
                contents: vec![
                    Content {
                        color_code: ColorCode {
                            modifier: "faded",
                            color: "blue"
                        },
                        quantity: 3
                    },
                    Content {
                        color_code: ColorCode {
                            modifier: "dotted",
                            color: "black"
                        },
                        quantity: 4
                    }
                ]
            },
            Bag {
                color_code: ColorCode {
                    modifier: "vibrant",
                    color: "plum"
                },
                contents: vec![
                    Content {
                        color_code: ColorCode {
                            modifier: "faded",
                            color: "blue"
                        },
                        quantity: 5
                    },
                    Content {
                        color_code: ColorCode {
                            modifier: "dotted",
                            color: "black"
                        },
                        quantity: 6
                    }
                ]
            },
            Bag {
                color_code: ColorCode {
                    modifier: "faded",
                    color: "blue"
                },
                contents: vec![]
            },
            Bag {
                color_code: ColorCode {
                    modifier: "dotted",
                    color: "black"
                },
                contents: vec![]
            },
        ]
    );
}

#[test]
fn parse_bag_rules_from_puzzle_input() {
    let bag_rules = parse_bag_rules(INPUT);

    assert_eq!(bag_rules.len(), 594);
}

#[test]
fn number_of_bag_colors_containing_shiny_gold_bags_example1() {
    let num_bag_colors =
        number_of_bag_colors_containing_shiny_gold_bags(&parse_bag_rules(EXAMPLE1));

    assert_eq!(num_bag_colors, 4);
}

#[test]
fn number_of_bag_colors_containing_shiny_gold_bags_puzzle_input() {
    let num_bag_colors = number_of_bag_colors_containing_shiny_gold_bags(&parse_bag_rules(INPUT));

    assert_eq!(num_bag_colors, 261);
}

#[test]
fn number_of_bags_inside_shiny_gold_bag_example1() {
    let num_bags = number_of_bags_inside_shiny_gold_bag(&parse_bag_rules(EXAMPLE1));

    assert_eq!(num_bags, 32);
}

#[test]
fn number_of_bags_inside_shiny_gold_bag_example2() {
    let num_bags = number_of_bags_inside_shiny_gold_bag(&parse_bag_rules(EXAMPLE2));

    assert_eq!(num_bags, 126);
}

#[test]
fn number_of_bags_inside_shiny_gold_bag_puzzle_input() {
    let num_bags = number_of_bags_inside_shiny_gold_bag(&parse_bag_rules(INPUT));

    assert_eq!(num_bags, 3765);
}
