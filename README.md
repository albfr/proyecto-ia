# dlx
An eXact Cover (XC) solver.

## Usage

`cargo run --release -- [OPTIONS]`

It is *very* important to compile using `--release` since it makes the solver
run *much* faster.

Enter via stdin a line of items. These must be unique ASCII strings not having
`'|'`. This character is a reserved separator of primary and secondary items.

Enter secondary items following `'|'` if desired.

For example, the following line has 4 primary and 3 secondary items:

`a b c d | e f g`

Then, again via stdin, enter an option, one per line. An option is a set of
items. These must match the names entered previously and cannot repeat in an
option. Reading of options ends when reaching end-of-file (EOF).

## Options
```
  -f, --show-first                 Print first solution if it exists
  -h, --help                       Print this help menu
  -i, --solution-interval <SPACE>  Print a solution in intervals of <SPACE>
  -l, --level-limit <LEVEL>        Show up to <LEVEL> braches in reports
  -r, --report <SECS>              Print a report every <SECS> seconds
  -s, --randomize <SEED>           Pick item to cover in a random fashion
  -t, --timeout <SECS>             Stop program execution after <SECS> seconds
  -v, --verbose                    Print verbose output
```

### Default values
- `-f`: `false` (does not mean first solution is not printed, if `-i=1` it will),
- `-h`: `false`,
- `-i`: `0` (no solutions are printed by default),
- `-l`: `12`,
- `-r`: `5`,
- `-s`: `None` (first item of minimum length is chosen),
- `-t`: `None`,
- `-v`: `false`.

## Comments

Currently `dlx` *only* supports generalized XC (i. e. XC + secondary items).
Plans to support further generalizations of XC are planned! As well as creating
a problem preprocessor and parsers to make I/O of famous puzzles easier
(queens, sudoku, polyomino packing...)

### A sketch of what's to come

- Option to write solutions and running-time statistics into a file,
- Generalize solver to handle a wider range of XC problems:
    - eXact Covering with Colors (XCC): `colorful_dance()`,
    - Multiple Covering with Colors (MCC): `multiple_dance()`,
    - Minimum cost covering: X$, C$, M$,
    - Output a Zero-suppressed Binary Decision Diagram.
- Problem preprocessor to remove redundant items/options,
- Puzzle parser to `dlx` supported format:
    - n queens placing,
    - Sudoku,
    - Polyomino packing,
    - Futoshiki,
    - KenKen,
    - Hidato,
    - Kakuro,
    - Hitori,
    - And maybe others!
