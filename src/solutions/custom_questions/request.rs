// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(CustomQuestionsApiClient, CustomQuestionsIdApiClient, ResourceIdentity::CustomQuestions);

impl CustomQuestionsApiClient {
	post!(
		doc: "Create new navigation property to customQuestions for solutions", 
		name: create_custom_questions,
		path: "/customQuestions",
		body: true
	);
	get!(
		doc: "Get customQuestions from solutions", 
		name: list_custom_questions,
		path: "/customQuestions"
	);
	get!(
		doc: "Get the number of the resource", 
		name: custom_questions,
		path: "/customQuestions/$count"
	);
}

impl CustomQuestionsIdApiClient {
	delete!(
		doc: "Delete navigation property customQuestions for solutions", 
		name: delete_custom_questions,
		path: "/customQuestions/{{RID}}"
	);
	get!(
		doc: "Get customQuestions from solutions", 
		name: get_custom_questions,
		path: "/customQuestions/{{RID}}"
	);
	patch!(
		doc: "Update the navigation property customQuestions in solutions", 
		name: update_custom_questions,
		path: "/customQuestions/{{RID}}",
		body: true
	);
}
