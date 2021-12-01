fn run_program(program: Vec<usize>) -> Vec<usize> {
    let length = program.len();
    let mut pc = 0;
    let mut program = program;

    while pc < length {
        let pos1 = pc + 1;
        let pos2 = pc + 2;
        let pos3 = pc + 3;

        let result = match program[pc] {
            1 => program[program[pos1]] + program[program[pos2]],
            2 => program[program[pos1]] * program[program[pos2]],
            99 => break,
            _ => unreachable!(),
        };
        let destination = program[pos3];

        program[destination] = result;
        pc += 4;
    }

    program
}

#[cfg(test)]
mod tests {
    use super::run_program;

    fn INPUT() -> Vec<usize> {
        vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 2, 9, 19, 23, 1, 9, 23,
            27, 2, 27, 9, 31, 1, 31, 5, 35, 2, 35, 9, 39, 1, 39, 10, 43, 2, 43, 13, 47, 1, 47, 6,
            51, 2, 51, 10, 55, 1, 9, 55, 59, 2, 6, 59, 63, 1, 63, 6, 67, 1, 67, 10, 71, 1, 71, 10,
            75, 2, 9, 75, 79, 1, 5, 79, 83, 2, 9, 83, 87, 1, 87, 9, 91, 2, 91, 13, 95, 1, 95, 9,
            99, 1, 99, 6, 103, 2, 103, 6, 107, 1, 107, 5, 111, 1, 13, 111, 115, 2, 115, 6, 119, 1,
            119, 5, 123, 1, 2, 123, 127, 1, 6, 127, 0, 99, 2, 14, 0, 0,
        ]
    }

    #[test]
    fn test_intcode_operation() {
        let programs = [
            [vec![1, 0, 0, 3, 99], vec![1, 0, 0, 2, 99]],
            [vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]],
            [vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]],
            [
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ],
        ];

        for [program, expected] in &programs {
            assert_eq!(run_program(program.to_vec()), expected.to_vec());
        }
    }

    #[test]
    fn run_advent_part_1() {
        let expected: Vec<usize> = vec![
            9581917, 12, 2, 2, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 48, 2, 9, 19, 144, 1,
            9, 23, 147, 2, 27, 9, 441, 1, 31, 5, 442, 2, 35, 9, 1326, 1, 39, 10, 1330, 2, 43, 13,
            6650, 1, 47, 6, 6652, 2, 51, 10, 26608, 1, 9, 55, 26611, 2, 6, 59, 53222, 1, 63, 6,
            53224, 1, 67, 10, 53228, 1, 71, 10, 53232, 2, 9, 75, 159696, 1, 5, 79, 159697, 2, 9,
            83, 479091, 1, 87, 9, 479094, 2, 91, 13, 2395470, 1, 95, 9, 2395473, 1, 99, 6, 2395475,
            2, 103, 6, 4790950, 1, 107, 5, 4790951, 1, 13, 111, 4790956, 2, 115, 6, 9581912, 1,
            119, 5, 9581913, 1, 2, 123, 9581915, 1, 6, 127, 0, 99, 2, 14, 0, 0,
        ];
        let mut program = INPUT();
        program[1] = 12; // Clear out 1202
        program[2] = 2;

        assert_eq!(run_program(program), expected);
    }

    #[test]
    fn run_advent_part2() {
        for noun in 0..99 {
            for verb in 0..99 {
                let mut program = INPUT();
                program[1] = noun;
                program[2] = verb;

                let copy = program.clone();
                let output = run_program(program);
                if output[0] == 19690720 {
                    println!("{:?} {:?}", noun, verb);
                    println!("{:?}", 100 * noun + verb);
                    println!("{:?}", copy);
                    println!("{:?}", output);
                    break;
                }
            }
        }

        assert!(false);
    }
}
