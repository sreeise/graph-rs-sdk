use graph_oauth::jwt::JWT;
use graph_oauth::jwt::Header;
use graph_oauth::jwt::Algorithm;

fn main() {
    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.vgz1gffXdteqASSBz55Yl-cLmTnIv6kDxFMfe6P1BKc";
 //   let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImp0aSI6ImQ3YjU5YjFiLTk4ZWYtNDNhOS04ZDRlLWM2ZjQ1NDQ3Y2RkYyIsImlhdCI6MTU1MTYwODk0NCwiZXhwIjoxNTUxNjEyNTQ0fQ.5-T_wUMxYr_8K-ISdqjr7CiznfjqiCXiSw__olq3zYs";
    let mut jwt = JWT::new(key);
    println!("{:#?}", jwt.validate());
    jwt.validate();
    println!("{:#?}", jwt.header());
}