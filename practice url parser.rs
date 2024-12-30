use url::Url;


fn main() {
  let url = "http://jim:123456@localhost:8080?page=1";
  let parsed_url = Url::parse(&url).unwrap();
  println!("{:?}", parsed_url.domain()); //Some("localhost") 
  println!("{:?}", parsed_url.host()); // Some(Domain("localhost"))
  println!("{:?}", parsed_url.port()); // Some(8000)
  println!("{:?}", parsed_url.password()); // Some("123456")
  println!("{:?}", parsed_url.username()); // "jim"
  println!("{:?}", parsed_url.scheme()); // "http"
  println!("{:?}", parsed_url.path()); // "/"
  println!("{:?}", parsed_url.query()); // Some("page=1")
  
}
