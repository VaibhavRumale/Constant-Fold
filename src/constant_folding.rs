use crate::ast::*;
use crate::error::FoldError;

/// Performs constant folding optimization on the given program.
/// This function iterates over the statements in the program and attempts
/// to fold any constant expressions within assignment statements.
pub fn fold_constants(program: &mut Program) -> Result<(), Vec<FoldError>> {
    // Vector to collect any errors encountered during folding
    let mut errors = Vec::new();

    for statement in &mut program.statements {
        match statement {
            // Only attempt to fold constants in assignment statements
            Statement::Assign { expression, .. } => {
                match fold_expression(expression) {
                    // Replace with folded expression if successful
                    Ok(folded_expr) => *expression = folded_expr,
                    Err(e) => {
                        // Collect errors
                        errors.push(e);
                    }
                }
            }
        }
    }

    // Return errors
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Recursively folds constant expressions within an expression.
/// This function handles binary operations and attempts to evaluate them
/// if both operands are constants.
fn fold_expression(expression: &Expression) -> Result<Expression, FoldError> {
    match expression {
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let folded_left = fold_value(left)?;
            let folded_right = fold_expression(right)?;

            // Check if both operands are integers to perform constant folding
            if let Value::Integer(left_val) = folded_left {
                if let Expression::Value(right_value) = &folded_right {
                    if let Value::Integer(right_val) = **right_value {
                        // Perform the operation if both operands are constant integers
                        let result = match operator {
                            Operator::Add => {
                                left_val.checked_add(right_val).ok_or(FoldError::Overflow)?
                            }
                            Operator::Subtract => left_val
                                .checked_sub(right_val)
                                .ok_or(FoldError::Underflow)?,
                            Operator::Multiply => {
                                left_val.checked_mul(right_val).ok_or(FoldError::Overflow)?
                            }
                            Operator::Divide => {
                                if right_val == 0 {
                                    return Err(FoldError::DivisionByZero);
                                }
                                left_val.checked_div(right_val).ok_or(FoldError::Overflow)?
                            }
                        };
                        // Return the folded result as a new constant expression
                        return Ok(Expression::Value(Box::new(Value::Integer(result))));
                    } else {
                        // Error if the right operand is not an integer
                        return Err(FoldError::TypeMismatch {
                            expected: "Integer".to_string(),
                            found: format!("{:?}", right_value),
                        });
                    }
                }
            }

            // If folding is not possible, return the original expression
            Ok(Expression::Binary {
                left: folded_left,
                operator: operator.clone(),
                right: Box::new(folded_right),
            })
        }
        // For value expressions, attempt to fold the inner value
        Expression::Value(value) => Ok(Expression::Value(Box::new(fold_value(value)?))),
    }
}

/// Folds constant values within a value structure.
/// This function handles folding for different kinds of values,
/// such as integers, identifiers, and nested expressions.
fn fold_value(value: &Value) -> Result<Value, FoldError> {
    match value {
        // Integers are already constant
        Value::Integer(_) => Ok(value.clone()),
        // Identifiers cannot be folded
        Value::Identifier(_) => Ok(value.clone()),
        Value::Expression(expr) => {
            // Recursively fold the expression
            let folded_expr = fold_expression(expr)?;
            if let Expression::Value(box_val) = folded_expr {
                // Return the folded value if it’s a constant
                Ok(*box_val)
            } else {
                // Wrap the expression if not fully folded
                Ok(Value::Expression(Box::new(folded_expr)))
            }
        }
    }
}
