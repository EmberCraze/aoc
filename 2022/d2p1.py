points = {'X': 1, 'Y': 2, 'Z': 3}
outcomes = {
  'A X': 3,
  'A Y': 6,
  'B Y': 3,
  'B Z': 6,
  'C Z': 3,
  'C X': 6
}
f = open("data.txt", "r")
tot = 0
for line in f.readlines():
  tot += points[line[2]]
  tot += outcomes.get(line.strip(), 0)
  print(tot)