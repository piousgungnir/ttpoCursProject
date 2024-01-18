use regex::Regex;
use solve_expr::calculator_client::CalculatorClient;
use solve_expr::{Expression};

use crate::solve_expr_rpc::solve_expr::expression::Type;

pub mod solve_expr {
    tonic::include_proto!("solve_expr");
}

pub async fn rpc_solve_expr(expression: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://[::1]:50051").await?;

    let re = Regex::new(r"(?i)for\s+(.+)").unwrap();

    let arguments;
    if let Some(captures) = re.captures(expression) {
        arguments = captures.get(1).unwrap().as_str();

        println!("Аргументы после for: {}", arguments);
    } else {
        arguments = "";
    }

    let mut result_expression = re.replace_all(expression, "");

    let request = tonic::Request::new(Expression {
        value: result_expression.into(),
        symbols: arguments.to_string(),
        r#type: i32::from(Type::EquationAlgebraically),
    });

    let response = client.solve_expr(request).await?.into_inner();

    println!("RESPONSE={:?}", response);

    Ok(response.value)
}