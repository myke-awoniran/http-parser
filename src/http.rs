use std::collections::HashMap;

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

#[derive(Debug, PartialEq)]
enum HttpParserError {
    NonUtf8InHttpHeader,
    WrongFormat,
}

#[derive(Debug, Default)]
struct HeaderKeys {
    pub key: HashMap<String, String>,
}

pub fn test_http() {
    let get_test_plain = b"GET / HTTP/1.1\r\nHost: www.example.re";

    let get_test_uri_parameters = b"Get /find?color=green HTTP/1.1\r\nHost: www.example.re";

    let post_test_formurl_encoded = b"POST /test HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 27\r\n\rjob=100&priority=2";

    let request_method: HttpMethod = parse_method(get_test_plain);
    assert_eq!(request_method, HttpMethod::GET);

    let http_version: Result<HttpVersion, HttpParserError> = parse_http_version(get_test_plain);

    assert_eq!(http_version, Ok(HttpVersion::V11));

    let uri_data = parse_uri_data(get_test_uri_parameters);

    let header_keys = parse_header_keys(post_test_formurl_encoded);

    if header_keys.is_some() {
        let header_keys = header_keys.unwrap();
        assert_eq!(header_keys.key, header_keys.key);
        assert_eq!(header_keys.key.len(), 3usize);
    } else {
        panic!("Blow!");
    }

    if uri_data.is_ok() {
        let uri_data = uri_data.unwrap();
        assert_eq!(uri_data.query_parameters.len(), 1);
        assert_eq!(uri_data.resource, "/find");
    } else {
        panic!("Blow!");
    }
}

fn find_header_in_request_bytes(
    buffer: &[u8],
    buffer_limit: usize,
) -> Result<String, HttpParserError> {
    let buffer_slice_limit: usize = buffer_limit.min(buffer.len());
    // &buffer[..buffer_slice_limit]

    let inner_string = {
        if let Ok(string_data) = String::from_utf8(buffer[..buffer_slice_limit].to_vec()) {
            string_data
        } else {
            return Err(HttpParserError::NonUtf8InHttpHeader);
        }
    };
    let header = inner_string.split("\r\n\r").nth(0).unwrap().to_string();
    Ok(header)
}

fn parse_header_keys(buffer: &[u8]) -> Option<HeaderKeys> {
    let header_bytes = find_header_in_request_bytes(&buffer, 4092).unwrap();

    let first_line: String = header_bytes.split("\r\n").take(1).collect();

    let header_lines: Vec<String> = first_line
        .split(" ")
        .map(|segment| segment.to_string())
        .collect::<Vec<String>>();

    if header_lines.len() == 1 {
        return None;
    }

    let mut header_keys = HeaderKeys::default();

    for line in header_lines[1..].iter() {
        let line_parts: Vec<&str> = line.split(":").collect();
        if line_parts.len() == 1 {
            panic!("")
        } else {
            header_keys
                .key
                .insert(line_parts[0].to_string(), line_parts[1].trim().to_string());
        }
    }
    Some(header_keys)
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
}

struct URIData {
    resource: String,
    query_parameters: HashMap<String, String>,
}

fn parse_http_version(buffer: &[u8]) -> Result<HttpVersion, HttpParserError> {
    let buffer_slice_limit: usize = 1024.min(buffer.len());
    let inner_string = {
        if let Ok(string_data) = String::from_utf8(buffer[..buffer_slice_limit].to_vec()) {
            string_data
        } else {
            return Err(HttpParserError::NonUtf8InHttpHeader);
        }
    };

    let first_line: String = inner_string.split("\r\n").take(1).collect();

    let line_parts = first_line.split(" ").collect::<Vec<&str>>();

    if line_parts.len() == 2 {
        return Ok(HttpVersion::V09);
    }

    if line_parts.len() == 3 {
        match line_parts[2] {
            "HTTP/1.1" => Ok(HttpVersion::V11),
            "HTTP/0.9" => Ok(HttpVersion::V10),
            "HTTP/2" => Ok(HttpVersion::V2),
            "HTTP/3" => Ok(HttpVersion::V3),
            _ => Ok(HttpVersion::V09),
        }
    } else {
        Ok(HttpVersion::UNSUPPORTED)
    }
}

fn parse_uri_data(buffer: &[u8]) -> Result<URIData, HttpParserError> {
    let buffer_slice_limit: usize = 1024.min(buffer.len());
    let inner_string = {
        if let Ok(string_data) = String::from_utf8(buffer[..buffer_slice_limit].to_vec()) {
            string_data
        } else {
            return Err(HttpParserError::NonUtf8InHttpHeader);
        }
    };

    let first_line: String = inner_string.split("\r\n").take(1).collect();

    let line_parts = first_line.split(" ").collect::<Vec<&str>>();

    let uri_string = match line_parts.len() {
        2 => line_parts[1],
        3 => line_parts[1],
        _ => return Err(HttpParserError::WrongFormat),
    };

    let resource_params_vec = uri_string.split("?").collect::<Vec<&str>>();

    let mut qv: HashMap<String, String> = HashMap::new();

    if resource_params_vec.len() == 2 {
        let qv_string = resource_params_vec[1];

        let kv_pairs: Vec<&str> = qv_string.split("&").collect();

        for kv_pair in kv_pairs {
            let key_value_vec = kv_pair.split("=").collect::<Vec<&str>>();
            qv.insert(key_value_vec[0].to_string(), key_value_vec[1].to_string());
        }
    }

    Ok(URIData {
        resource: resource_params_vec[0].to_string(),
        query_parameters: qv,
    })
}
