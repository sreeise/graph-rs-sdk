use graph_rs_sdk::*;

//businesses are the equivalent of booking pages
//creating a new business with the name "My Business" will create a booking page and once bookings
//are open, a new alias `mybusiness@tenant.com` will be created. If name already exists, the alias
//will be `mybusiness1@tenant.com`.
//
//
//

async fn create_business(){
    let client = Graph::new("ACCESS_TOKEN");


    let data = serde_json::json!({
        "displayName": "My Business"
    });

    let body = Body::from(data.to_string());
    
    let resp = client
        .solutions()
        .booking_businesses()
        .create_booking_businesses(body)
        .send()
        .await
        ;

    println!("{:#?}", resp);
}

async fn get_businesses() {
    let access_token = log_me_in().await.unwrap();
    let client = Graph::new(&access_token);
    let bus = client.solutions().booking_businesses().list_booking_businesses().send().await.unwrap();

    let businesses: serde_json::Value = bus.json().await.unwrap();
    println!("{:#}", businesses)
}

async fn get_appointments() {
    let access_token = log_me_in().await.unwrap();
    let client = Graph::new(&access_token);

    let appointments = client
        .solutions()
        //can be id retrieved from list_booking_businesses or pass the generated alias
        .booking_business("mybusiness@tenant.com")
        .appointments()
        .list_appointments()
        .send()
        .await
        .unwrap();

    let app_json: serde_json::Value = appointments.json().await.unwrap();
    
    println!("{:#?}", app_json);
}

