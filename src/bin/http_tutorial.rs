use reqwest::blocking::get;
use reqwest::Error;


fn main() -> Result<(), Error> {
    let o_resp = match get("https://httpbin.org/get") {
        Ok(response) => Some(response),
        Err(e) => {
            println!("An error occurred: {}", e);
            None // Return None to indicate the request failed
        }
    };


    if let Some(response) = o_resp {
        // Process the response if the request was successful
        println!("Request succeeded: {:?}", response);
        // println!("o_resp: {:?}", o_resp.expect("reason").status());
    } else {
        // Handle the case where the request failed
        println!("Failed to make the request.");
    }

    println!("done");
    Ok(())
}
