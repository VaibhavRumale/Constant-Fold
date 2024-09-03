#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::constant_folding::fold_constants;
    use crate::parser::parse;

    // 1. Parsing Tests
    #[test]
    fn test_parsing() {
        let input = "function main() {\n    let a = 1u8 + 2u8;\n}\n";
        let program = parse(input).unwrap();
        assert_eq!(program.name, "main");
        assert_eq!(program.inputs.len(), 0);
        assert_eq!(program.statements.len(), 1);
    }

    // 2. Basic Arithmetic Operations
    #[test]
    fn test_basic_arithmetic_operations() {
        let input = "function main() {
    let a = 1u8 + 2u8;
    let b = 3u8 * 4u8;
    let c = 10u8 - 5u8;
    let d = 20u8 / 4u8;
}\n";
        let mut program = parse(input).unwrap();
        fold_constants(&mut program).unwrap();

        let expected_results = vec![3u8, 12u8, 5u8, 5u8];

        for (i, statement) in program.statements.iter().enumerate() {
            let Statement::Assign { expression, .. } = statement;
            match expression {
                Expression::Value(value) => {
                    if let Value::Integer(result) = **value {
                        assert_eq!(result, expected_results[i], "Mismatch at statement {}", i);
                    } else {
                        panic!("Expected folded constant (Integer), got: {:?}", value);
                    }
                }
                _ => panic!("Expected folded constant (Value), got: {:?}", expression),
            }
        }
    }

    // 3. Order of Operations and Parentheses
    #[test]
    fn test_order_of_operations_and_parentheses() {
        let input = "function main() {
    let a = 7u8 + 3u8 * 2u8;
    let b = (1u8 + 2u8) * (5u8 - 4u8);
    let c = 2u8 + (3u8 * (4u8 + 5u8));
    let d = (6u8 + 2u8) * (3u8 - 1u8);
}\n";
        let mut program = parse(input).unwrap();
        fold_constants(&mut program).unwrap();

        let expected_results = vec![13u8, 3u8, 29u8, 16u8];

        for (i, statement) in program.statements.iter().enumerate() {
            let Statement::Assign { expression, .. } = statement;
            match expression {
                Expression::Value(value) => {
                    if let Value::Integer(result) = **value {
                        assert_eq!(result, expected_results[i], "Mismatch at statement {}", i);
                    } else {
                        panic!("Expected folded constant (Integer), got: {:?}", value);
                    }
                }
                _ => panic!("Expected folded constant (Value), got: {:?}", expression),
            }
        }
    }

    // 4. Nested Expressions
    #[test]
    fn test_nested_expressions_and_edge_cases() {
        let input = "function main() {
    let a = ((12u8 * 2u8) * 2u8) + 2u8 + (3u8 * (4u8 - 2u8));
    let b = (5u8 + 3u8) * (10u8 / 2u8);
    let c = 10u8 + (2u8 * (3u8 + 4u8));
    let d = (2u8 + 3u8) * ((4u8 - 1u8) / (2u8 + 1u8));
    let e = (1u8 + (2u8 * (3u8 + (4u8 * (5u8 - 1u8))))) + 6u8;
    let f = 255u8 - (1u8 * (2u8 + (3u8 * 4u8)));
    let g = ((10u8 / 2u8) * 3u8) + (4u8 * (2u8 + 1u8));
}\n";
        let mut program = parse(input).unwrap();
        fold_constants(&mut program).unwrap();

        let expected_results = vec![56u8, 40u8, 24u8, 5u8, 45u8, 241u8, 27u8];

        for (i, statement) in program.statements.iter().enumerate() {
            let Statement::Assign { expression, .. } = statement;
            match expression {
                Expression::Value(value) => {
                    if let Value::Integer(result) = **value {
                        assert_eq!(result, expected_results[i], "Mismatch at statement {}", i);
                    } else {
                        panic!("Expected folded constant (Integer), got: {:?}", value);
                    }
                }
                _ => panic!("Expected folded constant (Value), got: {:?}", expression),
            }
        }
    }

    // 5. Error Handling
    #[test]
    fn test_division_by_zero() {
        let input = "function main() {
    let a = 10u8 / 0u8;
    let b = 0u8 / 0u8;
    let c = 5u8 * (1u8 / 0u8);
}\n";
        let mut program = parse(input).unwrap();
        assert!(fold_constants(&mut program).is_err());
    }
    // 6. Boundary Values
    #[test]
    fn test_smallest_and_largest_values() {
        let input = "function main() {
    let a = 0u8 + 0u8;
    let b = 255u8 + 0u8;
    let c = 255u8 * 1u8;
    let d = 0u8 * 255u8;
    let e = 255u8 - 255u8;
    let f = 0u8 - 0u8;
    let g = 255u8 / 1u8;
    let h = 255u8 / 255u8;
}\n";
        let mut program = parse(input).unwrap();
        fold_constants(&mut program).unwrap();

        let expected_results = vec![0u8, 255u8, 255u8, 0u8, 0u8, 0u8, 255u8, 1u8];

        for (i, statement) in program.statements.iter().enumerate() {
            let Statement::Assign { expression, .. } = statement;
            match expression {
                Expression::Value(value) => {
                    if let Value::Integer(result) = **value {
                        assert_eq!(result, expected_results[i], "Mismatch at statement {}", i);
                    } else {
                        panic!("Expected folded constant (Integer), got: {:?}", value);
                    }
                }
                _ => panic!("Expected folded constant (Value), got: {:?}", expression),
            }
        }
    }
    // 7. Overflow and Underflow Handling
    #[test]
    fn test_overflow_and_underflow_handling() {
        let input = "function main() {
    let a = 255u8 + 1u8;   
    let b = 128u8 * 2u8;   
    let c = 0u8 - 1u8;     
    let d = 1u8 - 2u8;     
}\n";
        let mut program = parse(input).unwrap();
        let result = fold_constants(&mut program);

        assert!(
            result.is_err(),
            "Expected overflow/underflow error, but folding succeeded."
        );
    }
}
