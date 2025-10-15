#[derive(PartialEq, Debug, Eq)]
enum HttpMethod {
    GET,
    HEAD,
    OPTIONS,
    TRACE,
    DELETE,
    PUT,
    POST,
    PATCH,
    CONNECT,
    UNSUPPORTED,
}

#[derive(PartialEq, Debug, Eq)]
enum HttpVersion {
    V09,
    V10,
    V11,
    V2,
    V3,
    UNSUPPORTED,
}

impl HttpMethod {}

pub fn test_http() {
    let get_test_plain = b"GET / HTTP/1.1\r\nHost: www.example.re";

    let get_test_uri_parameters = b"Get /find?color=green HTTP/1.1\r\nHost: www.example.re";

    let post_test_formurl_encoded = b"POST /test HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 27\r\njob=100&priority=2";

    let request_method: HttpMethod = parse_method(get_test_plain);
    assert_eq!(request_method, HttpMethod::GET);

    let http_versaion: HttpVersion = parse_http_version(get_test_plain);

    println!("test_http");
}

fn parse_method(buffer: &[u8]) -> HttpMethod {
    let buf = {
        let mut b = [0u8; 3];
        b.clone_from_slice(&buffer[0..3]);
        b
    };

    match &buf {
        b"GET" => HttpMethod::GET,
        b"HEA" => HttpMethod::HEAD,
        b"OPT" => HttpMethod::OPTIONS,
        b"TRA" => HttpMethod::TRACE,
        b"DEL" => HttpMethod::DELETE,
        b"PUT" => HttpMethod::PUT,
        b"POS" => HttpMethod::POST,
        b"PAT" => HttpMethod::PATCH,
        b"CON" => HttpMethod::CONNECT,
        _ => HttpMethod::UNSUPPORTED,
    }

    // // HttpMethod::GET
    // todo!()
}

fn parse_http_version(buffer: &[u8]) -> HttpVersion {
    let buffer_slice_limit: usize = 1024.min(buffer.len());
    let inner_string = String::from_utf8(buffer[..buffer_slice_limit].to_vec()).unwrap();

    let first_line: String = inner_string.split("\r\n").take(1).collect();

    let line_parts = first_line.split(" ").collect::<Vec<&str>>();

    if line_parts.len() == 2 {
        return HttpVersion::V09;
    }
    if line_parts.len() == 3 {
        match line_parts[2] {
            "HTTP/1.1" => HttpVersion::V11,
            "HTTP/0.9" => HttpVersion::V10,
            "HTTP/2" => HttpVersion::V2,
            "HTTP/3" => HttpVersion::V3,
            _ => HttpVersion::V09,
        }
    } else {
        HttpVersion::UNSUPPORTED
    }

    // split on lines
    // take the 0th result, which is the first line of the request header
    // split on "" character

    // check if we have two string segments, return v09 is true
    // Match against the 3rd element

    // todo!();
}
