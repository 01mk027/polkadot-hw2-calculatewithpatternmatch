use std::io;

fn main()
{
    println!("Please enter the first number to be processed: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x64: f64 = x.trim().parse().expect("Input is not float");

    println!("Please enter the second number to be processed: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read line");
    let y64: f64 = y.trim().parse().expect("Input is not float");
    /* 
    let operation = Operation::Divide(x64, y64);
    let result = calculate(operation);
    match result{
        Ok(value) => println!("{}", value),
        Err(error_message) => println!("{}", error_message)
    }
    */

    let operation = Operation::Multiply(x64, y64);
    let result = operation.calculate_implemented();

    match result{
        Ok(value) => println!("{}", value),
        Err(error_message) => println!("{}", error_message)
    }
}   

#[derive(Debug)]
enum Operation
{
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}


impl Operation{
    fn calculate_implemented(&self) -> Result<f64, String>
    {
        match self
        {
            Operation::Add(x, y) => Ok(x + y),
            Operation::Subtract(x, y) => Ok(x - y),
            Operation::Multiply(x, y) => Ok(x * y),
            Operation::Divide(x, y) => {
                
                if y == &f64::from(0.0) 
                {
                    Err("Division by 0 is not allowed".to_string())
                }   
                else
                {
                    Ok(x/y)
                }
            }
        }
    } 
}


// This function is used on "Operation" enums instances.
fn calculate(operation: Operation) -> Result<f64, String>
{
    match operation
    {
        Operation::Add(x, y) => Ok(x + y),
        Operation::Subtract(x, y) =>Ok(x - y),
        Operation::Multiply(x, y) => Ok(x * y),
        Operation::Divide(x, y) => {
            if y == 0.0
            {
                Err("Division by zero is not allowed".to_string())
            }
            else {
                Ok(x / y)    
            }
            
        }
    }
}