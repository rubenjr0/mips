with open("ROMs\\" + input(), 'wb+') as f:
    f.write(bytearray([0x01, 0x02, 0x03, 0x04]))
