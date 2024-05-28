// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(ServicesApiClient, ServicesIdApiClient, ResourceIdentity::Services);

impl ServicesApiClient {
	post!(
		doc: "Create bookingService", 
		name: create_services,
		path: "/services",
		body: true
	);
	get!(
		doc: "List services", 
		name: list_services,
		path: "/services"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_services_count,
		path: "/services/$count"
	);
}

impl ServicesIdApiClient {
	delete!(
		doc: "Delete bookingService", 
		name: delete_services,
		path: "/services/{{RID}}"
	);
	get!(
		doc: "Get bookingService", 
		name: get_services,
		path: "/services/{{RID}}"
	);
	patch!(
		doc: "Update bookingservice", 
		name: update_services,
		path: "/services/{{RID}}",
		body: true
	);
}
