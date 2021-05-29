use graph_codegen::traits::RequestParser;

static KEY_VALUE_PAIR_PATH: &str = "/workbooks/{{RID}}/listItem/{{id}}/\
                                    getActivitiesByInterval(startDateTime={{id2}},\
                                    endDateTime={{id3}},interval={{id4}})";

#[test]
fn path_without_back_slashes() {
    let s = "/workbooks/{{RID}}/listItem/{listItem-id}/\
             getActivitiesByInterval(startDateTime={startDateTime},endDateTime={endDateTime},\
             interval={interval})";

    let path = s.transform_path();
    assert_eq!(KEY_VALUE_PAIR_PATH, path);
}

#[test]
fn path_with_back_slashes() {
    let s = "/workbooks/{{RID}}/listItem/{listItem-id}/getActivitiesByInterval(startDateTime=\'\
             {startDateTime}\',endDateTime=\'{endDateTime}\',interval=\'{interval}\')";

    let path = s.transform_path();
    assert_eq!(KEY_VALUE_PAIR_PATH, path);
}
