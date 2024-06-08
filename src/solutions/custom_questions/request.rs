// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    CustomQuestionsApiClient,
    CustomQuestionsIdApiClient,
    ResourceIdentity::CustomQuestions
);

impl CustomQuestionsApiClient {
    post!(
        doc: "Create bookingCustomQuestion",
        name: create_custom_questions,
        path: "/customQuestions",
        body: true
    );
    get!(
        doc: "List customQuestions",
        name: list_custom_questions,
        path: "/customQuestions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_custom_questions_count,
        path: "/customQuestions/$count"
    );
}

impl CustomQuestionsIdApiClient {
    delete!(
        doc: "Delete bookingCustomQuestion",
        name: delete_custom_questions,
        path: "/customQuestions/{{RID}}"
    );
    get!(
        doc: "Get bookingCustomQuestion",
        name: get_custom_questions,
        path: "/customQuestions/{{RID}}"
    );
    patch!(
        doc: "Update bookingCustomQuestion",
        name: update_custom_questions,
        path: "/customQuestions/{{RID}}",
        body: true
    );
}
