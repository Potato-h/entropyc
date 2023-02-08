use crate::ast::*;
use std::{collections::BTreeMap, ops::Deref};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Float(f64),
    Bool(bool),
}

impl From<Value> for f64 {
    fn from(val: Value) -> Self {
        match val {
            Value::Float(v) => v,
            Value::Bool(true) => 1.0,
            Value::Bool(false) => 0.0,
        }
    }
}

impl BinaryExpr {
    pub fn eval(&self, values: &BTreeMap<String, f64>) -> Value {
        // Generic lamdas is not allowed yet
        fn naive_apply(
            lhs: &Expression,
            rhs: &Expression,
            values: &BTreeMap<String, f64>,
            op: impl FnOnce(f64, f64) -> f64,
        ) -> Value {
            let lhs: f64 = lhs.eval(values).into();
            let rhs: f64 = rhs.eval(values).into();
            Value::Float(op(lhs, rhs))
        }

        match self.op {
            BinaryOp::Add => naive_apply(&self.lhs, &self.rhs, values, |x, y| x + y),
            BinaryOp::Sub => naive_apply(&self.lhs, &self.rhs, values, |x, y| x - y),
            BinaryOp::Mul => naive_apply(&self.lhs, &self.rhs, values, |x, y| x * y),
            BinaryOp::Div => naive_apply(&self.lhs, &self.rhs, values, |x, y| x / y),
            BinaryOp::CmpL => {
                let lhs: f64 = self.lhs.eval(values).into();
                let rhs: f64 = self.rhs.eval(values).into();
                Value::Bool(lhs > rhs)
            }
            BinaryOp::CmpR => {
                let lhs: f64 = self.lhs.eval(values).into();
                let rhs: f64 = self.rhs.eval(values).into();
                Value::Bool(lhs < rhs)
            }
        }
    }
}

impl Expression {
    pub fn eval(&self, values: &BTreeMap<String, f64>) -> Value {
        match self {
            Expression::Literal(lit) => Value::Float(lit.parse::<f64>().unwrap()),
            Expression::Variable(name) => Value::Float(*values.get(name).unwrap()),
            Expression::Binary(expr) => expr.eval(values),
            _ => todo!(),
        }
    }
}

#[derive(Default)]
pub struct Interpreter {
    values: BTreeMap<String, f64>,
}

impl Interpreter {
    pub fn execute_statement(&mut self, statement: &Statement) {
        match &statement {
            Statement::Let(statement) => {
                let Let { variable, value } = statement.deref();
                self.values
                    .insert(variable.clone(), value.eval(&self.values).into());
            }
            Statement::If(statement) => {
                let If { condition, body } = statement.deref();

                if let Value::Bool(true) = condition.eval(&self.values) {
                    body.iter().for_each(|stat| self.execute_statement(stat));
                }
            }
            Statement::While(statement) => {
                let While { condition, body } = statement.deref();

                while let Value::Bool(true) = condition.eval(&self.values) {
                    body.iter().for_each(|stat| self.execute_statement(stat));
                }
            }
            Statement::Print(statement) => {
                let val: f64 = statement.eval(&self.values).into();
                println!("{val}");
            }
        }
    }

    pub fn execute(&mut self, program: &Program) {
        program
            .body
            .iter()
            .for_each(|statement| self.execute_statement(statement))
    }
}
