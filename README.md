## To apply constant folding optimization, use the following command:

```
cargo run optimize --constant-fold --emit-leo
```
This command will parse before.leo, apply constant folding optimization, and save the optimized code to `src/files/generated.leo`


Alternatively, you can specify custom input and output files:

```
cargo run optimize --constant-fold --emit-leo --input <file name with path> --output <file name with path> 
```
EXAMPLE:
```
cargo run optimize --constant-fold --emit-leo --input src/files/before.leo --output src/files/optimizedCode.leo
```

Command Flags:

* The `--constant-fold` or `-c` flag applies constant folding optimization

* The `--emit-leo` or `-e` flag generates and saves the Leo code to a file

* The `--input` or `-i` flag allows you to specify the input file and path. If not provided, constant folding is applied to `src/files/before.leo` by default

* The `--output` or `-o` flag allows you to specify the output file name and path. If not provided, the generated code is saved to `src/files/generated.leo` (The `.leo` extension is automatically appended if not specified)


### How to Run Tests

The following test cases are included in the module:

* `test_parsing`: Validates the basic parsing of a simple function

* `test_basic_arithmetic_operations`: Tests the constant folding of basic arithmetic operations

* `test_order_of_operations_and_parentheses`: Ensures correct folding with order of operations and parentheses

* `test_nested_expressions`: Tests constant folding on nested expressions. example: `(2u8 + 3u8) * ((4u8 - 1u8) / (2u8 + 1u8))`

* `test_division_by_zero`: Checks if the constant folding correctly handles division by zero

* `test_smallest_and_largest_values`: Validates folding with boundary values (smallest and largest)

* `test_overflow_and_underflow_handling`: Tests handling of arithmetic overflow and underflow scenarios

To execute all these tests, simply run:

``` 
cargo test
 ```
