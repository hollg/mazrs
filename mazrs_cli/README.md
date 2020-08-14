# mazrs

A Rust implementation of the maze-generation algorithms in
[Mazes For Programmers](https://www.amazon.co.uk/Mazes-Programmers-Twisty-Little-Passages/dp/1680500554)
by James Buck.

## USAGE:

    mazrs [OPTIONS]

## FLAGS:

    -h, --help       Prints help information
    -V, --version    Prints version information

## OPTIONS:

    -a, --algorithm <algorithm>    Sets the algorithm used to build the maze [default: sidewinder]  [values: binary, sidewinder]
    -o, --output <output>          Specifies how the maze should be output [default: svg]  [values: ascii, svg]
