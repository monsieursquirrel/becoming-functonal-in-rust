pub fn update_contract_for_customer_list(self,
    ids: Vec<usize>, cls: &Fn(Contract) -> Contract) -> Self {

    let new_customers = self.all_customers.into_iter()
    .map(|customer| {
        if ids.contains(&customer.id) {
            Customer {
                contract: cls(customer.contract),
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
