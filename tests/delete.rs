extern crate restson;

#[macro_use]
extern crate serde_derive;

use restson::{Error, RestClient, RestPath};

#[derive(Serialize, Deserialize)]
struct HttpBinDelete {
    data: String,
}

impl RestPath<()> for HttpBinDelete {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("delete"))
    }
}


#[tokio::test]
async fn basic_delete() {
    let client = RestClient::new("http://httpbin.org").unwrap();

    client.delete::<(), HttpBinDelete>(()).await.unwrap();
}

#[tokio::test]
async fn delete_with() {
    let client = RestClient::new("http://httpbin.org").unwrap();

    let params = vec![("a", "2"), ("b", "abcd")];
    let data = HttpBinDelete {
        data: String::from("test data"),
    };
    client.delete_with((), &data, &params).await.unwrap();

    client.delete_with((), &data, &vec![]).await.unwrap();
}
