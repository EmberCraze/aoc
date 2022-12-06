f = open("d3/input-d3.txt", "r")
sums = 0
entries = f.readlines()
for index in range(3, len(entries)+3, 3):
  if index % 3 == 0:
    intersection = set(entries[index-3]).intersection(set(entries[index-2])).intersection(entries[index-1])
    if '\n' in intersection:
      intersection.remove('\n')
    intersection_result = intersection.pop()
    sums += ord(intersection_result)-38 if intersection_result.isupper() else ord(intersection_result)-96
print(sums)