from intcode import *

read_program("input-2")
program[1] = 12
program[2] = 2
run()
print(program[0])