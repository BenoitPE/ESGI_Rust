use serde::{Deserialize, Serialize};
use tide::prelude::*;
use tide::{Body, Request};

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

#[derive(Debug, Deserialize)]
struct Compute {
    number: i32,
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}

async fn compute(mut req: Request<()>) -> tide::Result {
    let Compute { number } = req.body_json().await?;
    let mut tmpnumber = 1;
    for i in 1..number + 1 {
        tmpnumber *= i
    }
    Ok(format!("{}", tmpnumber).into())
}

async fn compute2(req: Request<()>) -> tide::Result<String> {
    let n: usize = req.param("n")?.parse().unwrap_or(0);
    let mut tmpnumber = 1;
    for i in 1..n + 1 {
        tmpnumber *= i
    }
    let res = format!("La factorielle du nombre {} est {}\n", n, tmpnumber);
    Ok(res)
}

async fn acronyms(req: Request<()>) -> tide::Result {
    let mut res = json!({
        "meta": { "count": 2 },
        "animals": [
            { "type": "cat", "name": "chashu" },
            { "type": "cat", "name": "nori" }
        ]
    });
    Ok()
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.at("/fact/compute").post(compute);
    app.at("/fact/compute2/:n").get(compute2);
    app.at("/acronyms").get(acronyms);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
