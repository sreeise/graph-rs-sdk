use graph_oauth::jwt::JWT;
use graph_oauth::jwt::Algorithm;

#[test]
fn jwt_header() {
    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.vgz1gffXdteqASSBz55Yl-cLmTnIv6kDxFMfe6P1BKc";
    let mut jwt = JWT::new(key);
    jwt.validate().unwrap();
    assert_eq!(jwt.header().unwrap().alg(), Algorithm::HS256);

    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.i5Vdk3PhuVleXTwhmqoBkM8NIzw6vRoTcCHml-F49sO0iQSOGechIJllxHxNe0O0U-mNw-chT8VvERY53bQJ6g";
    let mut jwt = JWT::new(key);
    jwt.validate().unwrap();
    assert_eq!(jwt.header().unwrap().alg(), Algorithm::HS512);

    let key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzM4NCJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1NTE2MTc4MDgsImV4cCI6MTU4MzE1MzgwOCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoiand0QGV4YW1wbGUuY29tIiwiR2l2ZW5OYW1lIjoicnVzdCIsIlN1cm5hbWUiOiJvbmVkcml2ZSIsIkVtYWlsIjoiand0QGV4YW1wbGUuY29tIiwiUm9sZSI6WyJBZG1pbiIsIlByb2plY3QgQWRtaW5pc3RyYXRvciJdfQ.i7MTUwMJJkP8msKx_4zTnykAOT85Vyit0R0XPyHR2fFZu2UIFonFBbLNgvH-Y8Dw";
    let mut jwt = JWT::new(key);
    jwt.validate().unwrap();
    assert_eq!(jwt.header().unwrap().alg(), Algorithm::HS384);
}