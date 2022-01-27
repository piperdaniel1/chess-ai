with open('readout1.txt', 'r') as f1:
    lines1 = f1.readlines()
with open('readout2.txt', 'r') as f2:
    lines2 = f2.readlines()

for i in range(len(lines1)):
    lines1[i] = lines1[i].lower()

for i in range(len(lines2)):
    lines2[i] = lines2[i].lower()

print("Lines in file 1 but not in 2:")
for line in lines1:
    if line.lower() not in lines2:
        print(line, end="")

print("Lines in file 2 but not in 1:")
for line in lines2:
    if line.lower() not in lines1:
        print(line, end="")
print()