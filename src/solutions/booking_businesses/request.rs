// GENERATED CODE

use crate::api_default_imports::*;
use crate::solutions::*;

resource_api_client!(BookingBusinessesApiClient, BookingBusinessesIdApiClient, ResourceIdentity::BookingBusinesses);

impl BookingBusinessesApiClient {
	post!(
		doc: "Create bookingBusiness", 
		name: create_booking_businesses,
		path: "/bookingBusinesses",
		body: true
	);
	get!(
		doc: "List bookingBusinesses", 
		name: list_booking_businesses,
		path: "/bookingBusinesses"
	);
	get!(
		doc: "Get the number of the resource", 
		name: booking_businesses,
		path: "/bookingBusinesses/$count"
	);
}

impl BookingBusinessesIdApiClient {api_client_link_id!(appointment, AppointmentsIdApiClient);
api_client_link!(appointments, AppointmentsApiClient);
api_client_link_id!(service, ServicesIdApiClient);
api_client_link!(services, ServicesApiClient);

	delete!(
		doc: "Delete bookingBusiness", 
		name: delete_booking_businesses,
		path: "/bookingBusinesses/{{RID}}"
	);
	get!(
		doc: "Get bookingBusiness", 
		name: get_booking_businesses,
		path: "/bookingBusinesses/{{RID}}"
	);
	patch!(
		doc: "Update bookingbusiness", 
		name: update_booking_businesses,
		path: "/bookingBusinesses/{{RID}}",
		body: true
	);
	post!(
		doc: "Invoke action getStaffAvailability", 
		name: get_staff_availability,
		path: "/bookingBusinesses/{{RID}}/getStaffAvailability",
		body: true
	);
	post!(
		doc: "Invoke action publish", 
		name: publish,
		path: "/bookingBusinesses/{{RID}}/publish"
	);
	post!(
		doc: "Invoke action unpublish", 
		name: unpublish,
		path: "/bookingBusinesses/{{RID}}/unpublish"
	);
}
