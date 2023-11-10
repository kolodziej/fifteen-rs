num = 1311768467463790320

for i in range(16):
    x = num & 0b1111
    print(f"{15-i}: {x}")
    num >>= 4;
