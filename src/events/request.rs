// GENERATED CODE

use crate::api_default_imports::*;
use crate::solutions::*;

api_client!(VirtualEventsEventsApiClient, VirtualEventsEventsIdApiClient, ResourceIdentity::VirtualEventsEvents);

impl VirtualEventsEventsApiClient {
	post!(
		doc: "Create new navigation property to events for solutions", 
		name: create_events,
		path: "/events",
		body: true
	);
	get!(
		doc: "Get events from solutions", 
		name: list_events,
		path: "/events"
	);
	get!(
		doc: "Get the number of the resource", 
		name: events,
		path: "/events/$count"
	);
}

impl VirtualEventsEventsIdApiClient {api_client_link_id!(sessions, VirtualEventsSessionsIdApiClient);

	delete!(
		doc: "Delete navigation property events for solutions", 
		name: delete_events,
		path: "/events/{{RID}}"
	);
	get!(
		doc: "Get events from solutions", 
		name: get_events,
		path: "/events/{{RID}}"
	);
	patch!(
		doc: "Update the navigation property events in solutions", 
		name: update_events,
		path: "/events/{{RID}}",
		body: true
	);
}
