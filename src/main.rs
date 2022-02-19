mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A hard-coded JSON
    let json = r#"
            {
              "main": {
                "temp": 30.94
              }
            }
        "#;

    // Deserialize the hardcoded JSON into a Weather struct
    let weather1: model::Weather = serde_json::from_str(json).unwrap();

    println!("\nWeather from a JSON we hard-coded locally:\n{:?}", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //

    let client = reqwest::Client::new();
    // declared users here:
    let user = model::User {
    username: "Ricardo".into()
    password: "o2LAJH12Y2!kq1dA".into()
    };

    // declared authentication's response:
    let auth_response = reqwest::Client::new()
      .post("http://44.197.118.88:3000/v1/auth")
      .json(&user)
      .send()
      .await?;
    
    // parse the json response:  
    let json_response = auth_response.json::<model::UserResponse()>.await?;

    // declare weather response:
    let weather_response = client
      .get("http://44.197.118.88:3000/v1/weather")
      .header("Authorization", "Bearer ".to_owned() + &js.access_token)
      .send()
      .await?;
    
    // sending the weather response:
    let weather_prime = response.json::<model::Weather>().await?;

    println!("\nWeather_prime is running from the server on EC2 instances:\n {:?}", weather_prime);

    // same as weather's: Declare the variable, then send it back to the user.
    let hello_response = client
        .get("http://44.197.118.88:3000/v1/hello")
        .header("Authorization", "Basic ".to_owned() + &js.access_token)
        .send()
        .await?;

    let hello_prime = response.json::<model::Hello>().await?;    

    println!("\nHello_prime is running from the server on EC2 instances:\n {:?}", hello_prime);

    Ok(())
}
