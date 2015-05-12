pub fn update_customer_for_id_list(self,
    ids: Vec<usize>, cls: &Fn(Customer) -> Customer) -> Self {

    let new_customers = self.all_customers.into_iter()
    .map(|customer| {
        if ids.contains(&customer.id) {
            cls(customer)
        }
        else {
            customer
        }
    });

    AllCustomers {
        all_customers: new_customers.collect()
    }
}

pub fn update_contact(self, customer_id: usize, contact_id: usize,
    cls: &Fn(Contact) -> Contact) -> Self {

    self.update_customer_for_id_list(vec![customer_id], &|customer| {
        Customer{
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
    })
}

pub fn update_contract_for_customer_list(self,
    ids: Vec<usize>, cls: &Fn(Contract) -> Contract) -> Self {

    self.update_customer_for_id_list(ids, &|customer| {
        Customer{
            contract: cls(customer.contract),
            .. customer
        }
    })
}
