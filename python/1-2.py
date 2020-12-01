numbers = []
with open("inputs/input-1", 'r') as f:
    r = f.readline()
    while r != "":
        numbers.append(int(r))
        r = f.readline()

fuel = 0
for mass in numbers:
    massFuel = (int(mass / 3) - 2)
    while massFuel > 0:
        fuel += massFuel
        massFuel = (int(massFuel / 3) - 2)

print(fuel)