def read_file(fn):
    with open(f'ROMs/{fn}.s', 'r') as f:
        return [l.strip() for l in f.readlines()]

def decode_opcode(keyword):
    if keyword == 'j':
        return 0x08000000
    elif keyword == 'lw':
        return 0x8c000000
    elif keyword == 'sw':
        return 0xac000000
    elif keyword == 'addi':
        return 0x20000000
    elif keyword == 'alu':
        return 0x0

def extract_w_desp(body):
    rs = int(body[0][1:])
    desp, rt = body[1].split('(')
    rt = int(rt[1:len(rt)-1])
    return rs, rt, int(desp, 16)

def extract_wo_desp(rs, rt, ct):
    return int(rs[1:]), int(rt[1:]), int(ct)

def parse(instruction):
    if 'nop' in instruction:
        return 0x0
    elif instruction == 'halt':
        return 0xfc000000
    else:
        opcode, *body = instruction.replace(',', '').split()
        if opcode == 'j':
            return decode_opcode(opcode) + int(body[0])
        else:
            rt, rs, ct = extract_w_desp(body) if opcode != 'addi' else extract_wo_desp(*body)
            rt = int(rt) << 16
            rs = int(rs) << 21
            return decode_opcode(opcode) + rs + rt + ct



def instruction_compiler(inst):
    return[int(inst.replace(' ', '')[i:i+8], 2) for i in range(0, 32, 8)]


def write_rom(instructions):
    with open("ROMs/rom", 'wb+') as f:
        for i in instructions:
            f.write(i.to_bytes(4, byteorder='big'))

write_rom([parse(l) for l in read_file(input('Source file: '))])