enum HTTPMethod{
    GET,
    POST,
    HEAD,
    PUT,
    PATCH,
    DELETE,
    TRACE,
    OPTIONS,
    CONNECT
}

pub struct HTTPResponseHeader{
    header: String
}

impl HTTPResponseHeader{
    pub fn new() -> HTTPResponseHeader{
        HTTPResponseHeader{
            header: String::new()
        }
    }

    pub fn append(&mut self, property: &str, value: &str){
        let f = format!("{0}: {1}\r\n", property, value);
        self.header.push_str(f.as_str());
    }

    pub fn get(self) -> String {
        return self.header;
    }
}

pub struct HTTPResponse{
    version: String, // Example: HTTP/1.1
    status: u16, // Example: 200 OK
    phrase: String, // Exmple: OK
    header: HTTPResponseHeader,
    body: String
}

impl HTTPResponse{
    pub fn new(version: &str, status: u16, header: HTTPResponseHeader, body: String) -> HTTPResponse{
        HTTPResponse{
            version: String::from(version),
            status: status,
            phrase: String::from("OK"),
            header: header,
            body: body
        }
    }

    pub fn build(self) -> String {
        let mut buffer = String::new();
        let serverstr = format!("{0} {1} {2}\r\n", self.version, self.status, self.phrase);
        buffer.push_str(serverstr.as_str());
        buffer.push_str(self.header.get().as_str());
        buffer.push_str("\r\n");
        buffer.push_str(&self.body.to_string());
        return buffer;
    }

    /*pub fn matchStatusToPhrase(status: i8) -> String{
        return match status{
            200 => String::from("OK")
        };
    }*/
}