import argparse


def parse_line(line):
    try:
        op_code = int(line[:3], 2)
        arg1 = int(line[3:9], 2)
        arg2 = int(line[9:15], 2)
        if op_code < 4:
            function = int(line[-6:-3], 2)
        else:
            function = 0
        if op_code == 1:
            arg3 = int(line[15:21], 2)
        else:
            arg3 = int(line[15:27], 2)
    except Exception as e:
        print("Invalid instruction line")
        raise (e)

    return (op_code, arg1, arg2, arg3, function)


def execute_instruction(op_code, arg1, arg2, arg3, function, cpu_registers, ram, prog_ptr):
    if op_code == 1:
        if arg1 >= len(cpu_registers) or arg2 >= len(cpu_registers) or arg3 >= len(cpu_registers):
            raise ValueError("SEGFAULT : Registry adress out of bounds.")
        if function == 1:
            cpu_registers[arg1] = (
                cpu_registers[arg2] + cpu_registers[arg3]) & 0xFFFFFFFF
        elif function == 2:
            cpu_registers[arg1] = (
                cpu_registers[arg2] - cpu_registers[arg3]) & 0xFFFFFFFF
        elif function == 3:
            cpu_registers[arg1] = (
                cpu_registers[arg2] * cpu_registers[arg3]) & 0xFFFFFFFF
        elif function == 4:
            cpu_registers[arg1] = (
                cpu_registers[arg2] / cpu_registers[arg3]) & 0xFFFFFFFF
        elif function == 5:
            cpu_registers[arg1] = (
                cpu_registers[arg2] & cpu_registers[arg3]) & 0xFFFFFFFF
        elif function == 6:
            cpu_registers[arg1] = (
                cpu_registers[arg2] | cpu_registers[arg3]) & 0xFFFFFFFF
        elif function == 7:
            cpu_registers[arg1] = (
                cpu_registers[arg2] ^ cpu_registers[arg3]) & 0xFFFFFFFF
        else:
            raise (
                ValueError(f"Invalid function code {function} for op code {op_code}"))
    elif op_code == 2:
        if arg1 >= len(cpu_registers) or arg2 >= len(cpu_registers):
            raise ValueError("SEGFAULT : Registry adress out of bounds.")
        if function == 1:
            cpu_registers[arg1] = (
                cpu_registers[arg2] + arg3) & 0xFFFFFFFF
        elif function == 2:
            cpu_registers[arg1] = (
                cpu_registers[arg2] - arg3) & 0xFFFFFFFF
        elif function == 3:
            cpu_registers[arg1] = (
                cpu_registers[arg2] * arg3) & 0xFFFFFFFF
        elif function == 4:
            cpu_registers[arg1] = (
                cpu_registers[arg2] / arg3) & 0xFFFFFFFF
        elif function == 5:
            cpu_registers[arg1] = (
                cpu_registers[arg2] & arg3) & 0xFFFFFFFF
        elif function == 6:
            cpu_registers[arg1] = (
                cpu_registers[arg2] | arg3) & 0xFFFFFFFF
        elif function == 7:
            cpu_registers[arg1] = (
                cpu_registers[arg2] ^ arg3) & 0xFFFFFFFF
        else:
            raise (
                ValueError(f"Invalid function code {function} for op code {op_code}"))
    elif op_code == 3:
        if arg1 >= len(cpu_registers) or arg2 >= len(cpu_registers):
            raise ValueError("SEGFAULT : Registry adress out of bounds.")
        if function == 1 and (cpu_registers[arg1] == cpu_registers[arg2]):
            prog_ptr = arg3
            return (cpu_registers, ram, prog_ptr)
        elif function == 2 and (cpu_registers[arg1] != cpu_registers[arg2]):
            prog_ptr = arg3
            return (cpu_registers, ram, prog_ptr)
        elif function == 3 and (cpu_registers[arg1] < cpu_registers[arg2]):
            prog_ptr = arg3
            return (cpu_registers, ram, prog_ptr)
        elif function == 4 and (cpu_registers[arg1] > cpu_registers[arg2]):
            prog_ptr = arg3
            return (cpu_registers, ram, prog_ptr)
        elif function > 4:
            raise (
                ValueError(f"Invalid function code {function} for op code {op_code}"))
        else:
            pass
    elif op_code == 4:
        if arg1 >= len(cpu_registers) or (cpu_registers[arg2] + arg3) >= len(ram):
            raise ValueError("SEGFAULT : Memory adress out of bounds.")
        cpu_registers[arg1] = (ram[cpu_registers[arg2] + arg3] << 24) + (ram[cpu_registers[arg2] + arg3 + 1]
                                                                         << 16) + (ram[cpu_registers[arg2] + arg3 + 2] << 8) + (ram[cpu_registers[arg2] + arg3 + 3])
    elif op_code == 5:
        if arg1 >= len(cpu_registers) or (cpu_registers[arg2] + arg3) >= len(ram):
            raise ValueError("SEGFAULT : Memory adress out of bounds.")
        ram[cpu_registers[arg2] +
            arg3] = (cpu_registers[arg1] & 0xFF000000) >> 24
        ram[cpu_registers[arg2] + arg3 +
            1] = (cpu_registers[arg1] & 0x00FF0000) >> 16
        ram[cpu_registers[arg2] + arg3 +
            2] = (cpu_registers[arg1] & 0x0000FF00) >> 8
        ram[cpu_registers[arg2] + arg3 + 3] = cpu_registers[arg1] & 0x000000FF
    else:
        raise (ValueError(f"Invalid op code {op_code}"))
    return (cpu_registers, ram, prog_ptr+1)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("filename", help="Program file name", type=str)
    args = parser.parse_args()

    with open(args.filename, "r") as f:
        program_lines = f.readlines()

    cpu_registers = [0]*32
    ram = [0]*32
    prog_ptr = 0

    while 0 <= prog_ptr < len(program_lines):
        try:
            (op_code, arg1, arg2, arg3, function) = parse_line(
                program_lines[prog_ptr])
        except Exception as e:
            print(f"Execution failed because of : {e}")
            return ()

        try:
            (cpu_registers, ram, prog_ptr) = execute_instruction(
                op_code, arg1, arg2, arg3, function, cpu_registers, ram, prog_ptr)
        except Exception as e:
            print(f"Execution failed because of : {e}")
            return ()

        print(
            f"\n{prog_ptr=}, {op_code=}, {arg1=}, {arg2=}, {arg3=}, {function=}\n {cpu_registers=}\n {ram=}\n")


if __name__ == "__main__":
    main()
