use aws_lambda_events::apigw::ApiGatewayProxyResponse;
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderMap;

pub fn new_content_created_response(
    body: String,
    status_code: i64,
    id: String,
) -> ApiGatewayProxyResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("location", format!("/{}", id).parse().unwrap());

    new_response_with_headers(body, status_code, headers)
}

pub fn new_response(body: String, status_code: i64) -> ApiGatewayProxyResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    new_response_with_headers(body, status_code, headers)
}

fn new_response_with_headers(
    body: String,
    status_code: i64,
    headers: HeaderMap,
) -> ApiGatewayProxyResponse {
    let b: Option<Body> = if body == "" { None } else { Some(body.into()) };

    ApiGatewayProxyResponse {
        status_code,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: b,
        headers,
    }
}
