# arithmetic-evaluator

command that parses and evaluates simple arithmetic text for learning [combine](https://github.com/Marwes/combine) parser combinator library.

## Usage

    Usage:
      arithmetic-evaluator [options]

```shell
$ ./arithmetic-evaluator -e " 6 * 18 / 9 - 2"
6 * 18 / 9 - 2 = 10

$ ./arithmetic-evaluator
3 + (
5*(3-
6))
# input end here
3 + (5 * (3 - 6)) = -12
```

## Options:

| name            | description                          |
|:----------------|:-------------------------------------|
| `-e TEXT`       | Specify expression to be evaludated  |
| `-h`, `--help`  | Print this message                   |
