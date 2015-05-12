// I've used functional update syntax to shorten this: putting ".. variable"
// at the end of a struct literal means, copy the fields from variable.
// Note: the preceding item must have its trailing comma; the compiler
// error in this case is unintuitive.

pub fn set_contract_for_customer(self,
    ids: Vec<usize>, status: bool) -> Self {

    // Note: into_iter() not iter(). This turns the vec
    // into an iterator parmanently, allowing the values
    // to be moved out.
    let new_customers = self.all_customers.into_iter()
    .map(|customer| {
        if ids.contains(&customer.id) {
            Customer {
                contract: Contract {
                    enabled: status,
                    .. customer.contract
                },
                .. customer
            }
        }
        else {
            customer
        }
    });

    // An additional step of building a new AllCustomers object so it can
    // be assigned to a suitable variable by the caller.
    AllCustomers {
        all_customers: new_customers.collect()
    }
}
