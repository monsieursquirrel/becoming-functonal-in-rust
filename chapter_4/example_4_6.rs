pub fn each_enabled_contact<'a>(&'a self, cls: &Fn(&'a Contact)) {
    self.all_customers.iter()
    .filter(|customer| {
        customer.enabled && customer.contract.enabled
    })
    .map(|customer| {
            customer.contacts.iter()
            .filter(|contact| {
                contact.enabled
            })
            .map(cls)
            .count(); // just need to call something to consume the iterator

        })
    .count();
}
