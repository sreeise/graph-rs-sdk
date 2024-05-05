// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(CustomersApiClient, CustomersIdApiClient, ResourceIdentity::Customers);

impl CustomersApiClient {
	post!(
		doc: "Create new navigation property to customers for solutions", 
		name: create_customers,
		path: "/customers",
		body: true
	);
	get!(
		doc: "Get customers from solutions", 
		name: list_customers,
		path: "/customers"
	);
	get!(
		doc: "Get the number of the resource", 
		name: customers,
		path: "/customers/$count"
	);
}

impl CustomersIdApiClient {
	delete!(
		doc: "Delete navigation property customers for solutions", 
		name: delete_customers,
		path: "/customers/{{RID}}"
	);
	get!(
		doc: "Get customers from solutions", 
		name: get_customers,
		path: "/customers/{{RID}}"
	);
	patch!(
		doc: "Update the navigation property customers in solutions", 
		name: update_customers,
		path: "/customers/{{RID}}",
		body: true
	);
}
