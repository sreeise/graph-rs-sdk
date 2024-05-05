// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(ServicesApiClient, ServicesIdApiClient, ResourceIdentity::Services);

impl ServicesApiClient {
	post!(
		doc: "Create new navigation property to services for solutions", 
		name: create_services,
		path: "/services",
		body: true
	);
	get!(
		doc: "Get services from solutions", 
		name: list_services,
		path: "/services"
	);
	get!(
		doc: "Get the number of the resource", 
		name: services,
		path: "/services/$count"
	);
}

impl ServicesIdApiClient {
	delete!(
		doc: "Delete navigation property services for solutions", 
		name: delete_services,
		path: "/services/{{RID}}"
	);
	get!(
		doc: "Get services from solutions", 
		name: get_services,
		path: "/services/{{RID}}"
	);
	patch!(
		doc: "Update the navigation property services in solutions", 
		name: update_services,
		path: "/services/{{RID}}",
		body: true
	);
}
