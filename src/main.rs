use std::io::{self, Write};

fn main(){
    loop{
        println!("Enter an expression(eg 2+2) or 'exit' to quit  ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit"){
            break;
        }

        match parse_and_calculate(input){
            Ok(result) => println!("Result : {}", result),
            Err(error) => println!("Error : {}", error),
        }
    }
}

fn parse_and_calculate(expression: &str) -> Result<f64, &'static str>{
    let token: Vec<&str> = expression.split_whitespace().collect();
    
    if token.len() != 3 {
        return Err("Invalid expression. Please provide an expression with two numbers and an operator.");
    }
    let left_operand = token[0].parse::<f64>();
    let operator = token[1];
    let right_operand = token[2].parse::<f64>();

    match (left_operand, right_operand){
        (Ok(left), Ok(right)) => match operator {
                "+" => Ok(left + right),
                "-" => Ok(left - right),
                "*" => Ok(left * right),
                "/" => {
                    if right == 0.0 {
                        Err("Division by zero is not allowed.")
                    } else {
                        Ok(left / right)
                    }
                },
                _ => Err("Invalid operator. Please use '+', '-', '*', or '/'."),
         },
         _ => Err("Invalid expression. Please provide two numbers and an operator."),
 }
    }
     
