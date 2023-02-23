// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DeviceEnrollmentConfigurationsApiClient,
    DeviceEnrollmentConfigurationsIdApiClient,
    ResourceIdentity::DeviceEnrollmentConfigurations
);

impl DeviceEnrollmentConfigurationsApiClient {
    post!(
        doc: "Create new navigation property to deviceEnrollmentConfigurations for deviceManagement",
        name: create_device_enrollment_configurations,
        path: "/deviceEnrollmentConfigurations",
        body: true
    );
    get!(
        doc: "Get deviceEnrollmentConfigurations from deviceManagement",
        name: list_device_enrollment_configurations,
        path: "/deviceEnrollmentConfigurations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_enrollment_configurations_count,
        path: "/deviceEnrollmentConfigurations/$count"
    );
}

impl DeviceEnrollmentConfigurationsIdApiClient {
    delete!(
        doc: "Delete navigation property deviceEnrollmentConfigurations for deviceManagement",
        name: delete_device_enrollment_configurations,
        path: "/deviceEnrollmentConfigurations/{{RID}}"
    );
    get!(
        doc: "Get deviceEnrollmentConfigurations from deviceManagement",
        name: get_device_enrollment_configurations,
        path: "/deviceEnrollmentConfigurations/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property deviceEnrollmentConfigurations in deviceManagement",
        name: update_device_enrollment_configurations,
        path: "/deviceEnrollmentConfigurations/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to assignments for deviceManagement",
        name: create_assignments,
        path: "/deviceEnrollmentConfigurations/{{RID}}/assignments",
        body: true
    );
    get!(
        doc: "Get assignments from deviceManagement",
        name: list_assignments,
        path: "/deviceEnrollmentConfigurations/{{RID}}/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/deviceEnrollmentConfigurations/{{RID}}/assignments/$count"
    );
    delete!(
        doc: "Delete navigation property assignments for deviceManagement",
        name: delete_assignments,
        path: "/deviceEnrollmentConfigurations/{{RID}}/assignments/{{id}}",
        params: enrollment_configuration_assignment_id
    );
    get!(
        doc: "Get assignments from deviceManagement",
        name: get_assignments,
        path: "/deviceEnrollmentConfigurations/{{RID}}/assignments/{{id}}",
        params: enrollment_configuration_assignment_id
    );
    patch!(
        doc: "Update the navigation property assignments in deviceManagement",
        name: update_assignments,
        path: "/deviceEnrollmentConfigurations/{{RID}}/assignments/{{id}}",
        body: true,
        params: enrollment_configuration_assignment_id
    );
    post!(
        doc: "Invoke action assign",
        name: assign,
        path: "/deviceEnrollmentConfigurations/{{RID}}/microsoft.graph.assign",
        body: true
    );
    post!(
        doc: "Invoke action setPriority",
        name: set_priority,
        path: "/deviceEnrollmentConfigurations/{{RID}}/microsoft.graph.setPriority",
        body: true
    );
}
