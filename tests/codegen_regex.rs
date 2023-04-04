use graph_codegen::traits::RequestParser;

#[test]
fn path_without_back_slashes() {
    let s =
		"/workbooks/{{RID}}/listItem/{listItem-id}/getActivitiesByInterval(startDateTime={startDateTime},endDateTime={endDateTime},interval={interval})";

    let path = s.transform_path();
    assert_eq!("/workbooks/{{RID}}/listItem/{{id}}/getActivitiesByInterval(startDateTime={{id2}},endDateTime={{id3}},interval={{id4}})", path);
}

#[test]
fn path_with_back_slashes() {
    let s =
		"/workbooks/{{RID}}/listItem/{listItem-id}/getActivitiesByInterval(startDateTime=\'{startDateTime}\',endDateTime=\'{endDateTime}\',interval=\'{interval}\')";

    let path = s.transform_path();
    assert_eq!("/workbooks/{{RID}}/listItem/{{id}}/getActivitiesByInterval(startDateTime='{{id2}}',endDateTime='{{id3}}',interval='{{id4}}')", path);
}
