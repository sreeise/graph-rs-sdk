// GENERATED CODE

use crate::api_default_imports::*;
use crate::solutions::*;

resource_api_client!(BookingBusinessesApiClient, BookingBusinessesIdApiClient, ResourceIdentity::BookingBusinesses);

impl BookingBusinessesApiClient {
	post!(
		doc: "Create new navigation property to bookingBusinesses for solutions", 
		name: create_booking_businesses,
		path: "/bookingBusinesses",
		body: true
	);
	get!(
		doc: "Get bookingBusinesses from solutions", 
		name: list_booking_businesses,
		path: "/bookingBusinesses"
	);
	get!(
		doc: "Get the number of the resource", 
		name: booking_businesses,
		path: "/bookingBusinesses/$count"
	);
}

impl BookingBusinessesIdApiClient {api_client_link_id!(staff_member, StaffMembersIdApiClient);
api_client_link_id!(customer, CustomersIdApiClient);
api_client_link!(appointments, AppointmentsApiClient);
api_client_link!(services, ServicesApiClient);
api_client_link_id!(appointment, AppointmentsIdApiClient);
api_client_link_id!(service, ServicesIdApiClient);
api_client_link!(custom_questions, CustomQuestionsApiClient);
api_client_link!(customers, CustomersApiClient);
api_client_link_id!(custom_question, CustomQuestionsIdApiClient);
api_client_link!(staff_members, StaffMembersApiClient);

	delete!(
		doc: "Delete navigation property bookingBusinesses for solutions", 
		name: delete_booking_businesses,
		path: "/bookingBusinesses/{{RID}}"
	);
	get!(
		doc: "Get bookingBusinesses from solutions", 
		name: get_booking_businesses,
		path: "/bookingBusinesses/{{RID}}"
	);
	patch!(
		doc: "Update the navigation property bookingBusinesses in solutions", 
		name: update_booking_businesses,
		path: "/bookingBusinesses/{{RID}}",
		body: true
	);
	post!(
		doc: "Create new navigation property to calendarView for solutions", 
		name: create_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView",
		body: true
	);
	get!(
		doc: "Get calendarView from solutions", 
		name: list_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView"
	);
	get!(
		doc: "Get the number of the resource", 
		name: calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView/$count"
	);
	delete!(
		doc: "Delete navigation property calendarView for solutions", 
		name: delete_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView/{{id}}",
		params: booking_appointment_id
	);
	get!(
		doc: "Get calendarView from solutions", 
		name: get_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView/{{id}}",
		params: booking_appointment_id
	);
	patch!(
		doc: "Update the navigation property calendarView in solutions", 
		name: update_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView/{{id}}",
		body: true,
		params: booking_appointment_id
	);
	post!(
		doc: "Invoke action cancel", 
		name: cancel,
		path: "/bookingBusinesses/{{RID}}/calendarView/{{id}}/cancel",
		body: true,
		params: booking_appointment_id
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
