<p align="center">
	<img src="logo.svg" alt="ariadne" width="20%"/>
</p>

# Ariadne - A maze solver
A CLI utility for finding a path in an image of a maze.

## About
_Ariadne_ is a simple command-line tool that, given as input an image of a maze, draws in an output file the path that leads from the start to the end.

The goal of the project is a complete, flexible utility that allows the analysis of different pathfinding algorithms, as well as simply solving mazes and presenting the solution in an appealing way.

## Usage
```
Usage: ariadne [OPTIONS] --filename <FILENAME>

Options:
  -f, --filename <FILENAME>        Input image filename
  -a, --algorithm <ALGORITHM>      Pathfinding algorithm [default: dijkstra] [possible values: dijkstra, a-star]
  -o, --output-file <OUTPUT_FILE>  Output file [default: output.png]
  -l, --logging                    Activate stdout logging
  -w, --wider                      Draw a wider path
  -h, --help                       Print help information
  -V, --version                    Print version information
```
