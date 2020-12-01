numbers = []
with open("input-1", 'r') as f:
    r = f.readline()
    while r != "":
        numbers.append(int(r))
        r = f.readline()

fuel = 0
for mass in numbers:
    fuel += (int(mass / 3) - 2)

print(fuel)