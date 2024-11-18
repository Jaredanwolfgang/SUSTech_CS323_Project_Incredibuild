use std::fmt;
use crate::tree::*;

impl fmt::Display for ProgramPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramPart::Statement(statement) => write!(f, "Statement: {}", statement),
            ProgramPart::Function(function) => write!(f, "Functions: {}", function),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Include(s) => write!(f, "Include: {}", s),
            Statement::GlobalVariable(v) => write!(f, "Global Variable: {}", v),
            Statement::Struct(vars) => write!(f, "Struct: [{}]", 
                vars.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::VarDeclaration(ident, values, dims) => write!(f, "Variable Declaration: {} = [{}] with dimensions [{}]", 
                ident, 
                values.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(", "), 
                dims.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", ")),
            Variable::FormalParameter(ident, values, dims) => write!(f, "Formal Parameter: {} = [{}] with dimensions [{}]", 
                ident, 
                values.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(", "), 
                dims.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", ")),
            Variable::VarReference(ident) => write!(f, "{}", ident),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::FuncReference(ident, input_params) => write!(f, "FuncCall: {}[{}]", 
                ident, 
                input_params.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(", ")
            ),
            Function::FuncDeclaration(ident, _input_params, _output_param, body) => write!(f, "Function: {}:[{}]", ident,  body),
        }
    }
}

impl fmt::Display for CompExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompExpr::Value(val) => write!(f, "{}", val),
            CompExpr::Variable(var) => write!(f, "{}", var),
            CompExpr::UnaryOperation(op, expr) => write!(f, "({} {})", op, expr),
            CompExpr::BinaryOperation(left, op, right) => write!(f, "({} {} {})", left, op, right),
            CompExpr::Error => write!(f, "MissingTermError"),
        }
    }
}

impl fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignExpr::AssignOperation(var, expr) => write!(f, "Assignment: {} = {}", var, expr),
        }
    }
}

impl fmt::Display for CondExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CondExpr::Bool(b) => write!(f, "Condition: {}", b),
            CondExpr::UnaryCondition(op, expr) => write!(f, "Condition: {} {}", op, expr),
            CondExpr::Condition(left, op, right) => write!(f, "Condition: {} {} {}", left, op, right),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}: i32", i),
            Value::Float(fl) => write!(f, "{}: f32", fl),
            Value::String(s) => write!(f, "{}: String", s),
            Value::Char(c) => write!(f, "{}: char", c),
            Value::Bool(b) => write!(f, "{}: bool", b),
            Value::Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Pow => write!(f, "^"),
            BinaryOperator::Mod => write!(f, "%"),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
        }
    }
}


impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::Inc => write!(f, "++"),
            UnaryOperator::Dec => write!(f, "--"),
        }
    }
}

impl fmt::Display for JudgeOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JudgeOperator::GT => write!(f, ">"),
            JudgeOperator::GE => write!(f, ">="),
            JudgeOperator::LT => write!(f, "<"),
            JudgeOperator::LE => write!(f, "<="),
            JudgeOperator::EQ => write!(f, "=="),
            JudgeOperator::NE => write!(f, "!="),
        }
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            If::IfExpr(cond, body) => write!(f, "If: {} then {}", cond, body),
            If::IfElseExpr(cond, body, opt_body) => 
                write!(f, "If: {} then {} else {}", cond, body, opt_body),
        }
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Loop::WhileExpr(cond, body) => write!(f, "While Loop ({}): \n do {}", cond, body),
            Loop::ForExpr(init, cond, update, body) => write!(f, "For Loop ({}; {}; {}): \n do {}", init, cond, update, body),
        }
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Body(expressions) => {
                write!(f, "Body: [{}]", 
                    expressions.iter().map(|expr| format!("{}", expr)).collect::<Vec<String>>().join(", "))
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::If(if_expr) => write!(f, "{}", if_expr),
            Expr::Loop(loop_expr) => write!(f, "{}", loop_expr),
            Expr::Assign(assign_expr) => write!(f, "{}", assign_expr),
            Expr::Break => write!(f, "Break"),
            Expr::Continue => write!(f, "Continue"),
            Expr::Return(val) => write!(f, "Return: {}", val),
            Expr::FuncCall(func) => write!(f, "{}", func),
            Expr::VarDec(var) => write!(f, "{}", var),
        }
    }
}