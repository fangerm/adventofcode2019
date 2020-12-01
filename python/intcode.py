program = []

class Halt(Exception):
    pass

def exec_instruction(inst: int, first: int, second: int, third: int):
    if inst == 1: # ADD
        program[third] = program[first] + program[second]
    elif inst == 2: # MUL
        program[third] = program[first] * program[second]
    elif inst == 99: # HALT
        raise Halt

def read_program(name: str):
    program.clear()
    with open(name, 'r') as f:
        code = f.readline()
        for num in code.split(','):
            program.append(int(num))

def run() -> list:
    try:
        for pc in range(0, len(program), 4):
            exec_instruction(program[pc], program[pc + 1], program[pc + 2], program[pc + 3])
    except Halt:
        pass

    return program
