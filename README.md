# MathFacts

This package is to help practice common / simple math facts. I wrote it for my 8yo son to practice his addition, subtraction, and multiplication.

At the end of the `number` of questions it will display a tally with the following info:
- Score
- Correct
- Total questions
- Average time per question

## Usage
```
USAGE:
    mathfacts [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help               Print help information
    -m, --max <MAX>          Largest number to possibly generate [default: 12]
    -n, --number <NUMBER>    Number of questions to ask [default: 10]
        --negative           Allow random to generate negative numbers

SUBCOMMANDS:
    add         
    help        Print this message or the help of the given subcommand(s)
    multiply    
    subtract    
```
