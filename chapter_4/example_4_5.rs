pub fn send_enabled_customers_emails(&self, message: &str) {
    self.all_customers.iter()
    .filter(|customer| {
        customer.enabled && customer.contract.enabled
    })
    .map(|customer| {
            customer.contacts.iter()
            .filter(|contact| {
                contact.enabled
            })
            .map(|contact| {
                contact.send_email(message)
            })
            .count(); // just need to call something to consume the iterator

        })
    .count();
}
