extern crate restson;

#[macro_use]
extern crate serde_derive;

use restson::{Error, RestClient, RestPath};
use std::time::Duration;

#[derive(Deserialize)]
struct HttpBinAnything {
    url: String,
    args: HttpBinAnythingArgs,
}

#[derive(Deserialize)]
struct HttpBinAnythingArgs {
    #[serde(default)]
    a: String,
    #[serde(default)]
    b: String,
}

impl RestPath<()> for HttpBinAnything {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("anything"))
    }
}

impl RestPath<u32> for HttpBinAnything {
    fn get_path(param: u32) -> Result<String, Error> {
        Ok(format!("anything/{}", param))
    }
}

impl<'a> RestPath<(u32, &'a str)> for HttpBinAnything {
    fn get_path(param: (u32, &str)) -> Result<String, Error> {
        let (a, b) = param;
        Ok(format!("anything/{}/{}", a, b))
    }
}

#[tokio::test]
async fn basic_get_builder() {
    let client = RestClient::builder()
        .timeout(Duration::from_secs(10))
        .send_null_body(false)
        .build("https://httpbin.org")
        .unwrap();

    let data = client.get::<_, HttpBinAnything>(()).await.unwrap();
    assert_eq!(data.url, "https://httpbin.org/anything");
}

#[tokio::test]
async fn basic_get_https() {
    let client = RestClient::new("https://httpbin.org").unwrap();

    let data = client.get::<_, HttpBinAnything>(()).await.unwrap();
    assert_eq!(data.url, "https://httpbin.org/anything");
}

#[tokio::test]
async fn get_path_param() {
    let client = RestClient::new("https://httpbin.org").unwrap();

    let data = client.get::<_, HttpBinAnything>(1234).await.unwrap();
    assert_eq!(data.url, "https://httpbin.org/anything/1234");
}

#[tokio::test]
async fn get_multi_path_param() {
    let client = RestClient::new("https://httpbin.org").unwrap();

    let data = client.get::<_, HttpBinAnything>((1234, "abcd")).await.unwrap();
    assert_eq!(data.url, "https://httpbin.org/anything/1234/abcd");
}

#[tokio::test]
async fn get_query_params() {
    let client = RestClient::new("https://httpbin.org").unwrap();

    let params = vec![("a", "2"), ("b", "abcd")];
    let data = client.get_with::<_, HttpBinAnything>((), &params).await.unwrap();

    assert_eq!(data.url, "https://httpbin.org/anything?a=2&b=abcd");
    assert_eq!(data.args.a, "2");
    assert_eq!(data.args.b, "abcd");
}
