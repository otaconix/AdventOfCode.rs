function print_line(day, parsing, part1, part2, full) {
  printf("| %02d | %.6f | %.6f | %.6f | %6f |\n", day, parsing, part1, part2, full)
}

BEGIN {
  day=0
  parsing_total=0.0
  part1_total=0.0
  part2_total=0.0
  total_total=0.0
  print "| Day | Parsing | Part 1 | Part 2 | Total |"
  print "|:----:|:-------:|:------:|:------:|:-----:|"
}

match($0, /Parsing duration: (PT([0-9.]+)S|P(0)D)/, arr) {
  parsing = arr[2]
}

match($0, /Part 1 duration: (PT([0-9.]+)S|P(0)D)/, arr) {
  part1 = arr[2]
}

match($0, /Part 2 duration: (PT([0-9.]+)S|P(0)D)/, arr) {
  part2 = arr[2]
}

match($0, /Full run duration: (PT([0-9.]+)S|P(0)D)/, arr) {
  day += 1
  total = arr[2]
  parsing_total += parsing
  part1_total += part1
  part2_total += part2
  total_total += total

  print_line(day, parsing, part1, part2, total)

  parsing = 0
  part1 = 0
  part2 = 0
  total = 0
}

END {
  printf("| **Sum** | **%.6f** | **%.6f** | **%.6f** | **%.6f** |", parsing_total, part1_total, part2_total, total_total)
}
