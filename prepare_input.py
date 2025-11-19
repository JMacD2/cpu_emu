def dec_to_hex(dec):
    output = ""
    while dec != 0:
        remainder = int(dec % 16)
        if remainder < 10:
            output = chr(int(remainder + 48)) + output
        else:
            output = chr(int(remainder + 55)) + output
        dec = int(dec / 16)
    while len(output) < 12:
        output = "0" + output
    return output

file = open("./recursive_fib.txt", "w")
for i in range(1000):
    file.write("0x" + dec_to_hex(i*64) + "|\n")
