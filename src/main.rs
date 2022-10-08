use tokio;
use soup::prelude::*;
use reqwest;

async fn test() {
    let body = reqwest::get("https://www.officialdata.org/countries").await.expect("Error").text().await.unwrap();
    let s = Soup::new(&body);

    let v = vec!["Country:", "Currency:", "Anual:", "Quaterly:","Monthly:"];

    for e in s.tag("tr").find_all() {
        for (i,e2) in e.tag("td").find_all().enumerate() {


            if i == 0 {

            }
            if i == 5{
                break;
            }
            else {
                println!("{}    {}",v[i],e2.text())
            }
        }   
        println!();
    }
}

#[tokio::main]
async fn main(){
    test().await;
}
