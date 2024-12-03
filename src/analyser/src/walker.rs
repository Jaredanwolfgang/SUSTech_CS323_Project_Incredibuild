use spl_ast::tree::*;
use crate::manager::SymbolManager;
use crate::error::SemanticError;
use crate::symbol::*;

pub struct Walker {
    pub program: Program,
    pub manager: SymbolManager,
    pub errors: Vec<SemanticError>,
    pub vecs: Vec<VarSymbol>,
}

impl Walker {
    pub fn new(program: Program, manager: SymbolManager) -> Walker {
        Walker {
            program,
            manager,
            errors: Vec::new(),
            vecs: Vec::new(),
        }
    }

    pub fn get_symbols(&self) -> Vec<VarSymbol> {
        self.vecs.clone()
    }

    pub fn traverse(&mut self) {
        let program_clone = self.program.clone();
        self.traverse_program(&program_clone);
    }


    fn traverse_program(&mut self, program: &Program) {
        match program {
            Program::Program(parts) => {
                println!("Program");
                for part in parts {
                    self.traverse_program_part(part);
                }
            }
            Program::Error => {
                println!("Error in Program");
            }
        }
    }

    fn traverse_program_part(&mut self, part: &ProgramPart) {
        match part {
            ProgramPart::Statement(statement) => {
                println!("Statement");
                self.traverse_statement(statement);
            }
            ProgramPart::Function(function) => {
                println!("Function");
                self.traverse_function(function);
            }
        }
    }

    fn traverse_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Include(include) => println!("Include: {:?}", include),
            Statement::GlobalVariable(vars) => {
                println!("Global Variables");
                for var in vars {
                    self.traverse_variable(var);
                }
            }
            Statement::Struct(var) => {
                println!("Struct");
                self.traverse_variable(var);
            }
        }
    }

    fn traverse_variable(&mut self, variable: &Variable) {
        match variable {
            Variable::VarReference(name, dimensions) => {
                println!("VarReference: {:?}, Dimensions: {:?}", name, dimensions);
            }
            Variable::VarDeclaration(name, values, dimensions) => {
                self.vecs.push(VarSymbol::from((&mut self.manager, Variable::VarDeclaration(name.clone(), values.clone(), dimensions.clone()))));
                println!("VarDeclaration: {:?}, Values: {:?}, Dimensions: {:?}", name, values, dimensions);
            }
            Variable::VarAssignment(name, value, dimensions) => {
                println!("VarAssignment: {:?}, Value: {:?}, Dimensions: {:?}", name, value, dimensions);
            }
            Variable::StructReference(name) => println!("StructReference: {:?}", name),
            Variable::StructDefinition(name, variables) => {
                println!("StructDefinition: {:?}", name);
                for var in variables.iter() {
                    self.traverse_variable(var);
                }
            }
            Variable::StructDeclaration(obj_type, name, variables) => {
                println!("StructDeclaration: {:?}, Name: {:?}", obj_type, name);
                for var in variables.iter() {
                    self.traverse_variable(var);
                }
            }
            Variable::StructAssignment(name, field, value) => {
                println!("StructAssignment: {:?}, Field: {:?}, Value: {:?}", name, field, value);
            }
            Variable::MemberReference(name, field) => println!("MemberReference: {:?}, Field: {:?}", name, field),
            Variable::FormalParameter(name, values, dimensions) => {
                println!("FormalParameter: {:?}, Values: {:?}, Dimensions: {:?}", name, values, dimensions);
            }
            Variable::Error => println!("Error in Variable"),
        }
    }

    fn traverse_function(&mut self, function: &Function) {
        match function {
            Function::FuncReference(name, params) => {
                println!("FuncReference: {:?}, Params: {:?}", name, params);
            }
            Function::FuncDeclaration(name, inputs, output, body) => {
                println!("FuncDeclaration: {:?}, Output: {:?}", name, output);
                for input in inputs.iter() {
                    self.traverse_variable(input);
                }
                self.traverse_body(body);
            }
            Function::Error => println!("Error in Function"),
        }
    }

    fn traverse_body(&mut self, body: &Body) {
        match body {
            Body::Body(exprs) => {
                println!("Body");
                for expr in exprs {
                    self.traverse_expr(expr);
                }
            }
            Body::Error => println!("Error in Body"),
        }
    }

    fn traverse_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::If(if_expr) => {
                println!("If Expression");
                self.traverse_if(if_expr);
            }
            Expr::Loop(loop_expr) => {
                println!("Loop Expression");
                self.traverse_loop(loop_expr);
            }
            Expr::VarManagement(vars) => {
                println!("VarManagement");
                for var in vars {
                    self.traverse_variable(var);
                }
            }
            Expr::FuncCall(function) => {
                println!("Function Call");
                self.traverse_function(function);
            }
            Expr::Break => println!("Break"),
            Expr::Continue => println!("Continue"),
            Expr::Return(comp_expr) => {
                println!("Return");
                self.traverse_comp_expr(comp_expr);
            }
            Expr::Error => println!("Error in Expression"),
        }
    }

    fn traverse_if(&mut self, if_expr: &If) {
        match if_expr {
            If::IfExpr(cond, body) => {
                println!("IfExpr");
                self.traverse_cond_expr(cond);
                self.traverse_body(body);
            }
            If::IfElseExpr(cond, then_body, else_body) => {
                println!("IfElseExpr");
                self.traverse_cond_expr(cond);
                self.traverse_body(then_body);
                self.traverse_body(else_body);
            }
            If::Error => println!("Error in If"),
        }
    }

    fn traverse_loop(&mut self, loop_expr: &Loop) {
        match loop_expr {
            Loop::WhileExpr(cond, body) => {
                println!("WhileExpr");
                self.traverse_cond_expr(cond);
                self.traverse_body(body);
            }
            Loop::ForExpr(init, cond, increment, body) => {
                println!("ForExpr");
                self.traverse_expr(init);
                self.traverse_cond_expr(cond);
                self.traverse_expr(increment);
                self.traverse_body(body);
            }
            Loop::Error => println!("Error in Loop"),
        }
    }

    fn traverse_cond_expr(&mut self, cond: &CondExpr) {
        match cond {
            CondExpr::Bool(value) => println!("Bool Condition: {:?}", value),
            CondExpr::UnaryCondition(op, expr) => {
                println!("UnaryCondition: {:?}", op);
                self.traverse_cond_expr(expr);
            }
            CondExpr::BinaryCondition(lhs, op, rhs) => {
                println!("BinaryCondition: {:?} {:?} {:?}", lhs, op, rhs);
                self.traverse_cond_expr(lhs);
                self.traverse_cond_expr(rhs);
            }
            CondExpr::Condition(lhs, op, rhs) => {
                println!("Condition: {:?} {:?} {:?}", lhs, op, rhs);
                self.traverse_comp_expr(lhs);
                self.traverse_comp_expr(rhs);
            }
            CondExpr::Error => println!("Error in Condition Expression"),
        }
    }

    fn traverse_comp_expr(&mut self, comp: &CompExpr) {
        match comp {
            CompExpr::Value(value) => println!("Value: {:?}", value),
            CompExpr::Variable(variable) => {
                println!("Variable");
                self.traverse_variable(variable);
            }
            CompExpr::FuncCall(function) => {
                println!("Function Call");
                self.traverse_function(function);
            }
            CompExpr::UnaryOperation(op, expr) => {
                println!("UnaryOperation: {:?}", op);
                self.traverse_comp_expr(expr);
            }
            CompExpr::BinaryOperation(lhs, op, rhs) => {
                println!("BinaryOperation: {:?} {:?} {:?}", lhs, op, rhs);
                self.traverse_comp_expr(lhs);
                self.traverse_comp_expr(rhs);
            }
            CompExpr::Error => println!("Error in Computation Expression"),
        }
    }

}