# get contents of clipboard

import pyperclip

text = pyperclip.paste()

# seperate lines
lines = text.split('\n')

# sort lines
lines.sort()

leading_char = "a"
subcount = 0
subtotal = 0

# print
for i in range(len(lines)):
    if len(lines[i]) > 0:
        if lines[i][0] != leading_char:
            if subcount > 0:
                print(f"Subtotal: {subtotal} (from {subcount} moves)")
                subcount = 0
                subtotal = 0

            print()
            leading_char = lines[i][0]

        print(lines[i])
        subcount += 1
        subtotal += int(lines[i].split(':')[1][1:])

print("Subcount: " + str(subcount))

