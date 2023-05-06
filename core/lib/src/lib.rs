pub struct Request {
    pub message_body_size: u64,
    pub message_type: String,
    pub message_body: String,
}

impl Request {
    pub fn to_buffer(self: &Self) -> Vec<u8> {
        let mut req_buffer = Vec::new();

        req_buffer.extend_from_slice(&self.message_body_size.to_le_bytes());
    
        return req_buffer;
    }
}

pub struct Response {
    pub response_body_size: u64,
    pub response_type: String,
    pub response_body: String,
}