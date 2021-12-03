use super::*;

const INPUT: &str = include_str!("../../input/2020/day14.txt");

const EXAMPLE1: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

const EXAMPLE2: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

#[test]
fn parse_init_program_in_example1() {
    let program = parse_init_program(EXAMPLE1);

    assert_eq!(
        program,
        Program {
            instructions: vec![
                Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".into()),
                Instruction::Mem(8, 11),
                Instruction::Mem(7, 101),
                Instruction::Mem(8, 0)
            ]
        }
    )
}

#[test]
fn parse_init_program_in_puzzle_input() {
    let program = parse_init_program(INPUT);

    assert_eq!(program.instructions.len(), 576);
}

#[test]
fn sum_of_all_values_in_memory_v1_in_example1() {
    let init_program = parse_init_program(EXAMPLE1);

    let result = sum_of_all_values_in_memory_v1(&init_program);

    assert_eq!(result, 165);
}

#[test]
fn sum_of_all_values_in_memory_in_v1_puzzle_input() {
    let init_program = parse_init_program(INPUT);

    let result = sum_of_all_values_in_memory_v1(&init_program);

    assert_eq!(result, 7997531787333);
}

#[test]
fn build_mask_permutations_for_first_mask_in_example2() {
    let init_program = parse_init_program(EXAMPLE2);
    if let Some(Instruction::Mask(mask)) = init_program.instructions.first() {
        let masks = build_mask_permutations(&mask);

        assert_eq!(
            masks,
            vec![
                Mask {
                    clr: 0x_000F_FFFF_FFDE,
                    set: 0x_0000_0000_0012,
                },
                Mask {
                    clr: 0x_000F_FFFF_FFDE,
                    set: 0x_0000_0000_0032,
                },
                Mask {
                    clr: 0x_000F_FFFF_FFDE,
                    set: 0x_0000_0000_0013,
                },
                Mask {
                    clr: 0x_000F_FFFF_FFDE,
                    set: 0x_0000_0000_0033,
                },
            ]
        );
    } else {
        panic!("not a mask in first instruction");
    }
}

#[test]
fn sum_of_all_values_in_memory_v2_in_example2() {
    let init_program = parse_init_program(EXAMPLE2);

    let result = sum_of_all_values_in_memory_v2(&init_program);

    assert_eq!(result, 208);
}

#[test]
fn sum_of_all_values_in_memory_in_v2_puzzle_input() {
    let init_program = parse_init_program(INPUT);

    let result = sum_of_all_values_in_memory_v2(&init_program);

    assert_eq!(result, 3564822193820);
}
