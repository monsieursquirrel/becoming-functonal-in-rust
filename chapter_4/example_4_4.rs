
#[derive(Debug)]
pub struct Contact {
    contact_id: usize,
    first_name: String,
    last_name: String,
    email: String,
    enabled: bool
}

impl Contact {
    pub fn new(contact_id: usize, first_name: String, last_name: String,
        email: String, enabled: bool) -> Self
    {
        Contact {
            contact_id: contact_id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            enabled: enabled
        }
    }
}
