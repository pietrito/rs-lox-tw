use crate::environment::Environment;
use crate::errors::ExprError;
use crate::errors::StmtError;
use crate::expr::*;
use crate::stmt::*;
use crate::token::Object;
use crate::token_type::TokenType;

pub struct Interpreter {
    pub environment: Environment,
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, ExprError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, ExprError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => {
                if let Object::Num(x) = right {
                    Ok(Object::Num(-x))
                } else {
                    Err(ExprError::ExpectedNumberOperand)
                }
            }
            TokenType::Bang => Ok(Object::from(!self.is_truthy(right))),
            _ => Err(ExprError::UnreachableCode),
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, ExprError> {
        self.evaluate(&expr.expression)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, ExprError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left - right));
                    }
                }

                // TODO: Specific error
                Err(ExprError::InvalidExpression)
            }

            TokenType::Slash => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left / right));
                    }
                }

                // TODO: Specific error
                Err(ExprError::InvalidExpression)
            }

            // Handle number multiplication
            TokenType::Star => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left * right));
                    }
                }

                // TODO: Specific error
                Err(ExprError::InvalidExpression)
            }

            // Handle addition (number or string)
            TokenType::Plus => {
                // Handle 2 numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left + right));
                    }
                }

                // Handle 2 strings
                if let Object::Str(left) = left {
                    if let Object::Str(right) = right {
                        let mut s = left;
                        s.push_str(&right);
                        return Ok(Object::from(s));
                    }
                }

                // TODO: Specific error for when 2 different type (a string and a number)
                // TODO: Specific error
                Err(ExprError::ExpectedAddableOperands)
            }

            // Comparison operators
            //Handle '>'
            TokenType::Greater => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left > right));
                    }
                }

                // TODO: Specific error
                Err(ExprError::ExpectedNumberOperands)
            }

            //Handle '>='
            TokenType::GreaterEqual => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left >= right));
                    }
                }

                // TODO: Specific error
                Err(ExprError::ExpectedNumberOperands)
            }

            //Handle '<'
            TokenType::Less => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left < right));
                    }
                }

                // TODO: Specific error
                Err(ExprError::ExpectedNumberOperands)
            }

            //Handle '<='
            TokenType::LessEqual => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left <= right));
                    }
                }

                // TODO: Specific error
                Err(ExprError::ExpectedNumberOperands)
            }

            //Handle '!='
            TokenType::BangEqual => Ok(Object::from(left != right)),

            //Handle '=='
            TokenType::EqualEqual => Ok(Object::from(left == right)),

            _ => Err(ExprError::InvalidExpression),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, ExprError> {
        match self.environment.get(expr.name.dup()) {
            Ok(o) => Ok(o),
            Err(_) => Err(ExprError::InvalidExpression),
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), StmtError> {
        if let Err(e) = self.evaluate(&stmt.expression) {
            eprintln!("{}", e);
        }

        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), StmtError> {
        if let Ok(value) = self.evaluate(&stmt.expression) {
            println!("{}", value);
        }

        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), StmtError> {
        let mut value = Object::Nil;

        if stmt.initializer.is_some() {
            value = self.evaluate(stmt.initializer.as_ref().unwrap())?;
        }

        // TODO
        // self.environment.define(stmt.name.lexeme.clone(), value);

        Ok(())
    }
}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> Result<Object, ExprError> {
        expr.accept(self)
    }

    pub fn is_truthy(&self, obj: Object) -> bool {
        !(obj == Object::Nil || obj == Object::False)
    }

    pub fn interpret(&self, statements: &[Stmt]) {
        for statement in statements {
            self.execute(statement);
            /*
            {
                Ok(obj) => {
                    println!("Final result: {}", obj);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
            */
        }
    }

    pub fn execute(&self, stmt: &Stmt) {
        match stmt.accept(self) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}
