// GENERATED CODE

use crate::api_default_imports::*;
use crate::solutions::*;

resource_api_client!(SolutionsApiClient, ResourceIdentity::Solutions);

impl SolutionsApiClient {api_client_link!(booking_businesses, BookingBusinessesApiClient);
api_client_link_id!(booking_business, BookingBusinessesIdApiClient);

	get!(
		doc: "Get solutions", 
		name: get_solutions_root,
		path: "/solutions"
	);
	patch!(
		doc: "Update solutions", 
		name: update_solutions_root,
		path: "/solutions",
		body: true
	);
}
