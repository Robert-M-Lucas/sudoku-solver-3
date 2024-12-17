import math

out = "["

next = 1
pos = 0

for i in range(257):
    if i == next:
        out += f"{pos}, "
        next *= 2
        pos += 1
    else:
        out += "255, "

out = out[:-1] + "]"
print(out)