<p align="center">
	<img src="title-logo.svg" alt="ariadne" width="40%"/>
</p>

# Ariadne - A Maze Solver
A CLI utility for finding a path in an image of a maze.

## About
_Ariadne_ is a simple command-line tool that, given as input an image of a maze, draws in an output file the path that leads from the start to the end.

The goal of the project is a complete, flexible utility that allows the analysis of different pathfinding algorithms, as well as simply solving mazes and presenting the solution in an appealing way.

## Implemented Algorithms
### Shortest path
- Dijkstra
- A*

### Other
- DFS

## Usage
```
Usage: ariadne [OPTIONS] --filename <FILENAME>

Options:
  -f, --filename <FILENAME>        Input image filename
  -a, --algorithm <ALGORITHM>      Pathfinding algorithm [default: dijkstra] [possible values: dijkstra, a-star, dfs]
  -o, --output-file <OUTPUT_FILE>  Output file [default: output.png]
  -l, --logging                    Activate stdout logging
  -w, --wider                      Draw a wider path
  -h, --help                       Print help information
  -V, --version                    Print version information
```
