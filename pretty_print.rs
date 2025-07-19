
use super::ast::{Type, Expression, Statement, Function, ValueConstructor};


//Gabriel Pessoa Faustino - 231006121
pub fn print_type(t: &Type) {
    match t {
        Type::TInteger => print!("Int"),
        Type::TBool => print!("Bool"),
        Type::TReal => print!("Real"),
        Type::TString => print!("String"),
        Type::TVoid => print!("Void"),
        Type::TAny => print!("Any"),
        Type::TList(inner) => {
            print!("List<");
            print_type(inner);
            print!(">");
        }
        Type::TTuple(types) => {
            print!("(");
            for (i, ty) in types.iter().enumerate() {
                print_type(ty);
                if i != types.len() - 1 {
                    print!(", ");
                }
            }
            print!(")");
        }
        Type::TMaybe(inner) => {
            print!("Maybe<");
            print_type(inner);
            print!(">");
        }
        Type::TResult(ok, err) => {
            print!("Result<");
            print_type(ok);
            print!(", ");
            print_type(err);
            print!(">");
        }
        Type::TFunction(ret, args) => {
            print!("fn(");
            for (i, arg) in args.iter().enumerate() {
                print_type(arg);
                if i != args.len() - 1 {
                    print!(", ");
                }
            }
            print!(")");
            if let Some(ret_ty) = ret.as_ref() {
                print!(" -> ");
                print_type(ret_ty);
            }
        }
        Type::TAlgebraicData(name, constructors) => {
            print!("ADT {} {{ ", name);
            for c in constructors {
                print!("{}(", c.name);
                for (i, ty) in c.types.iter().enumerate() {
                    print_type(ty);
                    if i != c.types.len() - 1 {
                        print!(", ");
                    }
                }
                print!(") ");
            }
            print!("}}");
        }
    }
}

//Gabriel Pessoa Faustino - 231006121
pub fn print_statement(stmt: &Statement, indent: usize) {
    let space = " ".repeat(indent);
    match stmt {
        Statement::VarDeclaration(name, expr) => {
            println!("{}var {} =", space, name);
            print_expression(expr, indent + 2);
        }
        Statement::ValDeclaration(name, expr) => {
            println!("{}val {} =", space, name);
            print_expression(expr, indent + 2);
        }
        Statement::Assignment(name, expr) => {
            println!("{}{} =", space, name);
            print_expression(expr, indent + 2);
        }
        Statement::IfThenElse(cond, then_stmt, else_stmt) => {
            println!("{}if", space);
            print_expression(cond, indent + 2);
            println!("{}then", space);
            print_statement(then_stmt, indent + 2);
            if let Some(else_s) = else_stmt {
                println!("{}else", space);
                print_statement(else_s, indent + 2);
            }
        }
        Statement::Block(stmts) => {
            println!("{}{{", space);
            for s in stmts {
                print_statement(s, indent + 2);
            }
            println!("{}}}", space);
        }
        Statement::FuncDef(f) => {
            println!("{}fn {}(", space, f.name);
            for (i, param) in f.params.iter().enumerate() {
                print!("{}: ", param.argument_name);
                print_type(&param.argument_type);
                if i != f.params.len() - 1 {
                    print!(", ");
                }
            }
            println!(")");
            if let Some(body) = &f.body {
                print_statement(body, indent + 2);
            }
        }
        _ => println!("{}<outros statements>", space),
    }
}

// Wagner de Souza da Silva - 242039882

pub fn print_expression(e: &Expression, indent: usize) {
    let space = " ".repeat(indent);
    match e {
        Expression::CTrue => print!("True"),
        Expression::CFalse => print!("False"),
        Expression::CInt => print!("Int"),
        Expression::CReal => print!("Real"),
        Expression::CString => print!("String"),
        Expression::CVoid => print!("Void"),
        Expression::Var(name) => {
            print!("{}var {}", space, name);
        }
        Expression::FuncCall(name, expr) => {
            print!("{}(", name);
            for e in expr {
                print!("{}", e.name);
                for (i, ty) in e.types.iter().enumerate() {
                    print_expression(ty);
                    if i != e.types.len() - 1 {
                        print!(", ");
                    }
                }
            }
            print!(");");
        }
        Expression::Add(expr1, expr2) => {
            print!("{} + {}", expr1, expr2);
        }
        Expression::Sub(expr1, expr2) => {
            print!("{} - {}", expr1, expr2);
        }
        Expression::Mul(expr1, expr2) => {
            print!("{} * {}", expr1, expr2);
        }
        Expression::Div(expr1, expr2) => {
            print!("{} / {}", expr1, expr2);
        }
        Expression::And(expr1, expr2) => {
            print!("{} and {}", expr1, expr2);
        }
        Expression::Or(expr1, expr2) => {
            print!("{} or {}", expr1, expr2);
        }
        Expression::Not(expr) => {
            print!("not {}", expr);
        }
        Expression::EQ(expr1, expr2) => {
            print!("{} eq {}", expr1, expr2);
        }
        Expression::NEQ(expr1, expr2) => {
            print!("{} neq {}", expr1, expr2);
        }
        Expression::GT(expr1, expr2) => {
            print!("{} gt {}", expr1, expr2);
        }
        Expression::LT(expr1, expr2) => {
            print!("{} lt {}", expr1, expr2);
        }
        Expression::GTE(expr1, expr2) => {
            print!("{} gte {}", expr1, expr2);
        }
        Expression::LTE(expr1, expr2) => {
            print!("{} lte {}", expr1, expr2);
        }
        Expression::COk(inner) => {
            print!("Ok(");
            print_expression(inner);
            print!(")");
        }
        Expression::CErr(inner) => {
            print!("Err(");
            print_expression(inner);
            print!(")");
        }
        Expression::CJust(inner) => {
            print!("Just(");
            print_expression(inner);
            print!(")");
        }
        Expression::CNothing => print!("Nothing"),
        Expression::Unwrap(inner) => {
            print!("unwrap(");
            print_expression(inner);
            print!(")");
        }
        Expression::IsError(inner) => {
            print!("is_err(");
            print_expression(inner);
            print!(")");
        }
        Expression::IsNothing(inner) => {
            print!("is_nothing(");
            print_expression(inner);
            print!(")");
        }
        Expression::Propagate(inner) => {
            print!("propagate(");
            print_expression(inner);
            print!(")");
        }
        Expression::ListValue(expr) => {
            print!("ListValue {} ( ", expr);
            for e in expr {
                print!("{}", e.name);
                for (i, ty) in e.types.iter().enumerate() {
                    print_expression(ty);
                    if i != e.types.len() - 1 {
                        print!(", ");
                    }
                }
            }
            print!(")");
        }
        Expression::Constructor(name, expr) => {
            println!("impl {} {", name);
            print!("{}fn new(", space);
            for e in expr {
                print!("{}: {}", e.name, e.types);
                for (i, ty) in e.types.iter().enumerate() {
                    print_expression(ty);
                    if i != e.types.len() - 1 {
                        print!(", ");
                    }
                }
            }
            println!(") -> Self {");
            for e in expr {
                println!("{}Self { {} }", indent + 2, e.name);
            }
            print!("}");
        }
    }
}