use super::*;

const INPUT: &str = include_str!("../../../input/2020/day8.txt");

const EXAMPLE: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

#[test]
fn parse_program_example() {
    let program = parse_program(EXAMPLE);

    assert_eq!(
        program,
        Program(vec![
            Instruction {
                operation: OperationCode::Nop,
                argument: 0
            },
            Instruction {
                operation: OperationCode::Acc,
                argument: 1
            },
            Instruction {
                operation: OperationCode::Jmp,
                argument: 4
            },
            Instruction {
                operation: OperationCode::Acc,
                argument: 3
            },
            Instruction {
                operation: OperationCode::Jmp,
                argument: -3
            },
            Instruction {
                operation: OperationCode::Acc,
                argument: -99
            },
            Instruction {
                operation: OperationCode::Acc,
                argument: 1
            },
            Instruction {
                operation: OperationCode::Jmp,
                argument: -4
            },
            Instruction {
                operation: OperationCode::Acc,
                argument: 6
            },
        ])
    )
}

#[test]
fn accumulator_value_before_second_run_of_example_program() {
    let program = parse_program(EXAMPLE);

    let acc = accumulator_value_before_second_run(&program);

    assert_eq!(acc, 5);
}

#[test]
fn accumulator_value_before_second_run_of_program_in_puzzle_input() {
    let program = parse_program(INPUT);

    let acc = accumulator_value_before_second_run(&program);

    assert_eq!(acc, 1723);
}

#[test]
fn accumulator_value_after_fixing_the_endless_loop_in_puzzle_input() {
    let program = parse_program(INPUT);

    let acc = accumulator_value_after_fixing_the_endless_loop(&program);

    assert_eq!(acc, 846);
}
