
use hyper::Client;


use myconfig;

pub fn submit(qn_num: i32, score: i32) {
 
    let url = format!("http://{}:{}/score/{}/{}/{}", 
                      myconfig::SERVER, myconfig::PORT,
                      myconfig::TEAM_NAME, qn_num, score);

    let client = Client::new();
    
    let res = client.post(&url[..]).send();

    println!("");

    match res {
        Ok(res) => println!("posted score to the server, response: {:?}", res.status),
        Err(_) => println!("unable to post the score to the server"),
    }

    println!("");
}
