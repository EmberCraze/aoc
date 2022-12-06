f = open("d3/data.txt", "r")
sums = 0
for line in f.readlines():
  intersection = set(line[0:len(line)//2]).intersection(set(line[len(line)//2:])).pop()
  sums+= ord(intersection)-38 if intersection.isupper() else ord(intersection)-96
print(sums)