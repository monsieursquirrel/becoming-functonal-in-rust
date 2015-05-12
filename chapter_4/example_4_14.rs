pub fn update_contact(self, customer_id: usize, contact_id: usize,
    cls: &Fn(Contact) -> Contact) -> Self {

    let new_customers = self.all_customers.into_iter()
    .map(|customer| {
        if customer.id == customer_id {
            Customer {
                contacts: customer.contacts.into_iter()
                .map(|contact| {
                    if contact.contact_id == contact_id {
                        cls(contact)
                    }
                    else {
                        contact
                    }
                })
                .collect(),
                .. customer
            }
        }
        else {
            customer
        }
    });

    AllCustomers {
        all_customers: new_customers.collect()
    }
}
