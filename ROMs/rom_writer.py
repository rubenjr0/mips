def instruction_compiler(inst):
    return[int(inst.replace(' ', '')[i:i+8], 2) for i in range(0, 32, 8)]


with open("ROMs/rom", 'wb+') as f:
    instructions = [
        instruction_compiler("000010 11111111111111111111111111"),
        instruction_compiler("000100 11111111111111111111111111"),
        instruction_compiler("000000 11111111111111111111111111"),
        instruction_compiler("001000 11111111111111111111111111"),
        instruction_compiler("100011 11111111111111111111111111"),
        instruction_compiler("101011 11111111111111111111111111"),
        instruction_compiler("111111 11111111111111111111111111")
    ]

    for i in instructions:
        print(i)
        f.write(bytearray(i))
