from intcode import *

for noun in range(0, 100):
    for verb in range(0, 100):
        read_program("inputs/input-2")
        program[1] = noun
        program[2] = verb
        run()
        if program[0] == 19690720:
            print(100 * noun + verb)
            exit(0)