# arithmetic-evaluator

command that parses and evaluates simple arithmetic expression, which is created for learning [combine](https://github.com/Marwes/combine) parser combinator library.

[![Build Status](https://travis-ci.org/hyone/arithmetic-evaluator.svg?branch=master)](https://travis-ci.org/hyone/arithmetic-evaluator)
[![Build status](https://ci.appveyor.com/api/projects/status/s40xd5r0d2n31epr/branch/master?svg=true)](https://ci.appveyor.com/project/hyone/arithmetic-evaluator/branch/master)

## Usage

    Usage:
      arithmetic-evaluator [options]

```shell
$ ./arithmetic-evaluator -e " 6 * 18 / 9 - 2"
6 * 18 / 9 - 2 = 10

$ ./arithmetic-evaluator
3 + (
7/3 *(3-
6/11))
# input end here
3 + (7/3 * (3 - 6/11)) = 96/11
```

## Options:

| name            | description                          |
|:----------------|:-------------------------------------|
| `-e TEXT`       | Specify expression to be evaludated  |
| `-h`, `--help`  | Print this message                   |
