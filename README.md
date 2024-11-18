# Ctvrtcalc

Simple calculator, showcasing the code at different intermediate representations.

## Usage

You need to have Rust installed

```shell
sudo apt install cargo
```

Clone the repository, open a shell and run `cargo run`

### Supported operations

- `()` grouping
- `-` (unary) negation
- `+ | -` (binary) addition and subtraction
- `* | / | %` (binary) multiplication, division, modulation
- `^` (binary) exponentiation

### Printed representations

1. List of tokens (after lexing)
2. AST (after parsing)
3. Bytecode (for stack-based execution, RPN style)
4. Stack states after each instruction
5. The numeric result

### Example

```shell
> (1 + 2)^3 * 4 / -5 + 6 % (7 * 8)

=== TOKENS ===
[LParen, Numeral(1.0), Plus, Numeral(2.0), RParen, Caret, Numeral(3.0), Star, Numeral(4.0), Slash, Minus, Numeral(5.0), Plus, Numeral(6.0), Percent, LParen, Numeral(7.0), Star, Numeral(8.0), RParen]

=== AST ===
Add
|  Div
|  |  Mult
|  |  |  Pow
|  |  |  |  Add
|  |  |  |  |  Value(1)
|  |  |  |  |  Value(2)
|  |  |  |  Value(3)
|  |  |  Value(4)
|  |  Neg
|  |  |  Value(5)
|  Mod
|  |  Value(6)
|  |  Mult
|  |  |  Value(7)
|  |  |  Value(8)


=== BYTECODE ===
PUSH(1.0)
PUSH(2.0)
ADD
PUSH(3.0)
POW
PUSH(4.0)
MULT
PUSH(5.0)
NEG
DIV
PUSH(6.0)
PUSH(7.0)
PUSH(8.0)
MULT
MOD
ADD

=== EXECUTION ===
PUSH(1.0)
[1.0]
PUSH(2.0)
[1.0, 2.0]
ADD
[3.0]
PUSH(3.0)
[3.0, 3.0]
POW
[27.0]
PUSH(4.0)
[27.0, 4.0]
MULT
[108.0]
PUSH(5.0)
[108.0, 5.0]
NEG
[108.0, -5.0]
DIV
[-21.6]
PUSH(6.0)
[-21.6, 6.0]
PUSH(7.0)
[-21.6, 6.0, 7.0]
PUSH(8.0)
[-21.6, 6.0, 7.0, 8.0]
MULT
[-21.6, 6.0, 56.0]
MOD
[-21.6, 6.0]
ADD
[-15.600000000000001]

=== RESULT ===
-15.600000000000001
```
