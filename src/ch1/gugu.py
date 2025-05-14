# for row in range(9):
#   for col in range(9):
#     number = (row + 1) * (col + 1)
#     print(f"{number:>4},", end="")
#   print()

# 이중 for 문은 안되.
for row in range(1, 10):
  a = ["{:>4}".format(row * col) for col in range(1, 10)]
  print(",".join(a))