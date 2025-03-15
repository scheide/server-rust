use calc_near_x::calc_with_1;
use tide::Request;

#[async_std::main]

async fn main()-> tide::Result<()> {
    let mut app = tide::new(); 

    app.at("/").get(|_| async { Ok("Hello Tide! 😃") });
    app.at("/sum/:x/:y").get(sum);
    app.at("/sub/:x/:y").get(sub);

    println!("Server running on port 3003");

    app.listen("127.0.0.1:3003").await?;

    Ok(())
}

async fn sum(req: Request<()>) -> tide::Result {
    let x= req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y= req.param("y").unwrap_or("0").parse().unwrap_or(0);
    
    let z=calc_with_1::sum_plus_one(x, y);
    
    Ok(format!("Result: {}", z).into())
}

async fn sub(req: Request<()>) -> tide::Result {
    let x= req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y= req.param("y").unwrap_or("0").parse().unwrap_or(0);

    let z=calc_with_1::sub_less_one(x, y);
    
    Ok(format!("Result: {}", z).into())
}