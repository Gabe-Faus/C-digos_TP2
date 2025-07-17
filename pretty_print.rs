
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


