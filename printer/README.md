# aoc-printer

Solve Advent of Code puzzles through a physical scanner/printer.

First generate a qr code with the program. Print the outputted png file.

To solve, scan the paper with the scanner. The program works by filesystem
watching a specified folder, this is where the scanned image is expected to be.
It will then execute the corresponding solve program in this repo, then print
the results back out using the windows default printer.
