use graph_codegen::traits::RequestParser;

#[test]
fn parse_get_count_methods() {
    let method_name = "Get.Count.microsoft.graph.orgContact-ba73".method_name();
    assert_eq!("get_org_contact_count".to_string(), method_name);

    let method_name = "Get.Count.microsoft.graph.orgContact-7773".method_name();
    assert_eq!("get_org_contact_count".to_string(), method_name);

    let method_name = "Get.Count.microsoft.graph.orgContact-baac".method_name();
    assert_eq!("get_org_contact_count".to_string(), method_name);

    let method_name = "Get.Count.microsoft.graph.orgContact-BBAA".method_name();
    assert_eq!("get_org_contact_count".to_string(), method_name);

    let method_name = "Get.Count.microsoft.graph.orgContact-A7c4".method_name();
    assert_eq!("get_org_contact_count".to_string(), method_name);
}
