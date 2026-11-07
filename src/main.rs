use i_doit_client::{Client, Language};

const HOSTNAME: &str = "https://idoit-demo.keepbit.net";
const BASE_URL: &str = "/src/jsonrpc.php";
const API_KEY: &str = "0dG0a1PWiG12Y7Sv";

#[tokio::main]
async fn main() -> Result<(), ()> {
    let url = format!("{HOSTNAME}{BASE_URL}");

    let mut client = Client::builder().url(url).api_key(API_KEY).build().unwrap();

    /*
    let object_type_groups = client.object_type_groups().execute().await;

    if let Some(groups) = &object_type_groups {
        println!("Anzahl: {}", groups.len());
        println!("{:#?}", groups);
    } else {
        println!("No groups!");
    }
    */

    /*
    let addons = client.addons().execute().await;
    println!("{:#?}", addons);
    */

    /*
    let constants = client.constants().execute().await;
    println!("{:#?}", constants);
    */

    /*
    let licensing = client.license().execute().await;
    println!("{:#?}", licensing);
    */

    /*
    let object = client.object().id(106).execute().await;
    println!("{:#?}", object);
    */

    /*
    let objects = client.cache_update().await
        .objects()
        .filter_by_object_type(5)
        .execute()
        .await;
    println!("{:#?}", objects);
    */

    /*
    let object_types = client.cache_update().await.object_types().execute().await;
    println!("{:#?}", object_types);
    println!("Anzahl: {}", object_types.map(|o| o.len()).unwrap());
    */

    /*
    let statuses = client.status().execute().await;
    println!("{:#?}", statuses);
    */

    let groups = client.cache_update().await
        .objects()
        .filter_by_object_type("C__OBJTYPE__PERSON_GROUP")
        .filter_by_title("Admin")
        .limit(1)
        .execute()
        .await
        .unwrap();

    let admin_group = groups.first().unwrap();

    println!("Id der Admin Gruppe: {}", admin_group.id());

    let result = client.create_object().object_type("C__OBJTYPE__PERSON").title("Automation Backend").execute().await.unwrap();
    println!("Result: {:?}", result);
    // => id: 154

    let person_id = result.id();

    let person_id = 154;

    // todo: next: implement cmdb.category.save
    // todo: next: implement cmdb.category.save


    Ok(())
}
