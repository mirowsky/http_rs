#[allow(unused)]

pub mod response {
    pub const RAW_RESPONSE_MOCK: &str = r#"HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 12

{"hello":"world","foo":"bar","baz":"qux","quux":"corge"}"#;

    pub const RAW_COMPLEX_RESPONSE_MOCK: &str = r#"HTTP/1.1 200 OK
Date: Wed, 29 Jun 2022 23:47:07 GMT
Content-Type: application/json; charset=utf-8
Content-Length: 83
Connection: close
x-powered-by: Express
x-ratelimit-limit: 1000
x-ratelimit-remaining: 999
x-ratelimit-reset: 1640243547
vary: Origin, Accept-Encoding
access-control-allow-credentials: true
cache-control: max-age=43200
pragma: no-cache
expires: -1
x-content-type-options: nosniff
etag: W/"53-hfEnumeNh6YirfjyjaujcOPPT+s"
via: 1.1 vegur
CF-Cache-Status: HIT
Age: 11262
Accept-Ranges: bytes
Report-To: {"endpoints":[{"url":"https:\/\/a.nel.cloudflare.com\/report\/v3?s=ALe86Zew3aBuGcrM0TsdOrVzIjCgejVmc9%2Fi6xmKDsuN1tcBqcJIOi6klwlwxAxLT3lgcCDY5qmBBJB1jXnAH%2BJx5FmxnIWSQ6gx1VmFLGbel9Z2jEh0qo3lcNv8dSfqfw%2BCXASrLFbQH51%2BBw%2Bx5hD2"}],"group":"cf-nel","max_age":604800}
NEL: {"success_fraction":0,"report_to":"cf-nel","max_age":604800}
Server: cloudflare
CF-RAY: 72328ba1b89b8c87-EWR
alt-svc: h3=":443"; ma=86400, h3-29=":443"; ma=86400

{
  "userId": 1,
  "id": 1,
  "title": "delectus aut autem",
  "completed": false
}"#;
}
