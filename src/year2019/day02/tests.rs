use super::*;

const INPUT: &str = include_str!("../../../input/2019/day2.txt");

mod run_program {
    use super::*;

    #[test]
    fn example1() {
        let mut memory = Memory::init(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        let mut intcode_vm = IntcodeVM::default();

        intcode_vm.run(&mut memory);

        assert_eq!(
            memory,
            Memory::init(&[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])
        );
    }

    #[test]
    fn example2() {
        let mut memory = Memory::init(&[1, 0, 0, 0, 99]);

        let mut intcode_vm = IntcodeVM::default();

        intcode_vm.run(&mut memory);

        assert_eq!(memory, Memory::init(&[2, 0, 0, 0, 99]));
    }

    #[test]
    fn example3() {
        let mut memory = Memory::init(&[2, 3, 0, 3, 99]);

        let mut intcode_vm = IntcodeVM::default();

        intcode_vm.run(&mut memory);

        assert_eq!(memory, Memory::init(&[2, 3, 0, 6, 99]));
    }

    #[test]
    fn example4() {
        let mut memory = Memory::init(&[2, 4, 4, 5, 99, 0]);

        let mut intcode_vm = IntcodeVM::default();

        intcode_vm.run(&mut memory);

        assert_eq!(memory, Memory::init(&[2, 4, 4, 5, 99, 9801]));
    }

    #[test]
    fn example5() {
        let mut memory = Memory::init(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);

        let mut intcode_vm = IntcodeVM::default();

        intcode_vm.run(&mut memory);

        assert_eq!(memory, Memory::init(&[30, 1, 1, 4, 2, 5, 6, 0, 99]));
    }
}

mod part1 {
    use super::*;

    #[test]
    fn answer() {
        let memory = run_original_program(&parse(INPUT));

        assert_eq!(3_101_878, memory.read_word(0));
    }
}

mod part2 {
    use super::*;

    #[test]
    fn answer() {
        let result = find_noun_and_verb_for_gravity_assist(&parse(INPUT));

        assert_eq!(8444, result);
    }
}
