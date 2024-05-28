// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    CustomersApiClient,
    CustomersIdApiClient,
    ResourceIdentity::Customers
);

impl CustomersApiClient {
    post!(
        doc: "Create bookingCustomer",
        name: create_customers,
        path: "/customers",
        body: true
    );
    get!(
        doc: "List customers",
        name: list_customers,
        path: "/customers"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_customers_count,
        path: "/customers/$count"
    );
}

impl CustomersIdApiClient {
    delete!(
        doc: "Delete bookingCustomer",
        name: delete_customers,
        path: "/customers/{{RID}}"
    );
    get!(
        doc: "Get bookingCustomer",
        name: get_customers,
        path: "/customers/{{RID}}"
    );
    patch!(
        doc: "Update bookingCustomer",
        name: update_customers,
        path: "/customers/{{RID}}",
        body: true
    );
}
