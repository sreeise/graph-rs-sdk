use crate::api_types::RequestTask;
use crate::parser::HttpMethod;
use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum GeneratedMacroType {
    FnName(&'static str),
    Path(&'static str),
    ParamSize(usize),
    RequestTask(RequestTask),
    FnNameAndPath(&'static str, &'static str),
    Method(HttpMethod),
    Default,
}

#[derive(Builder, Debug, Clone, Eq, PartialEq)]
#[builder(
    pattern = "mutable",
    derive(Debug, Eq, PartialEq),
    setter(into, strip_option),
    default
)]
pub struct MethodMacroModifier {
    pub matching: Vec<GeneratedMacroType>,
    pub update: GeneratedMacroType,
}

impl MethodMacroModifier {
    pub fn builder() -> MethodMacroModifierBuilder {
        MethodMacroModifierBuilder::default()
    }

    pub fn fn_name_and_path(
        fn_name: &'static str,
        path: &'static str,
        update: GeneratedMacroType,
    ) -> MethodMacroModifier {
        MethodMacroModifier::builder()
            .matching(vec![GeneratedMacroType::FnNameAndPath(fn_name, path)])
            .update(update)
            .build()
            .unwrap()
    }
}

impl Default for MethodMacroModifier {
    fn default() -> Self {
        MethodMacroModifier {
            matching: vec![],
            update: GeneratedMacroType::Default,
        }
    }
}

pub fn get_method_macro_modifiers(resource_identity: ResourceIdentity) -> Vec<MethodMacroModifier> {
    match resource_identity {
		ResourceIdentity::AccessReviewsDefinitions =>
			vec![
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("stop"),
						GeneratedMacroType::Path("/definitions/{{RID}}/instances/{{id}}/microsoft.graph.stop")
					],
					update: GeneratedMacroType::FnName("stop_instances"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("filter_by_current_user"),
						GeneratedMacroType::Path("/definitions/{{RID}}/instances/microsoft.graph.filterByCurrentUser(on='{{id}}')")
					],
					update: GeneratedMacroType::FnName("filter_instances_by_current_user"),
				}
			],
		ResourceIdentity::AccessReviews =>
			vec![
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/accessReviews/historyDefinitions/$count")
					],
					update: GeneratedMacroType::FnName("get_history_definitions_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/accessReviews/historyDefinitions/{{id}}/instances/$count")
					],
					update: GeneratedMacroType::FnName("get_history_definitions_instances_count"),
				}
			],
		ResourceIdentity::Admin =>
			vec![
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("create_issues"),
						GeneratedMacroType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues")
					],
					update: GeneratedMacroType::FnName("create_health_overview_issues"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("list_issues"),
						GeneratedMacroType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues")
					],
					update: GeneratedMacroType::FnName("list_health_overview_issues"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_issues_count"),
						GeneratedMacroType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/$count")
					],
					update: GeneratedMacroType::FnName("get_health_overview_issues_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("delete_issues"),
						GeneratedMacroType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}")
					],
					update: GeneratedMacroType::FnName("delete_health_overview_issues"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_issues"),
						GeneratedMacroType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}")
					],
					update: GeneratedMacroType::FnName("get_health_overview_issues"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("update_issues"),
						GeneratedMacroType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}")
					],
					update: GeneratedMacroType::FnName("update_health_overview_issues"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("incident_report"),
						GeneratedMacroType::Path(
							"/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}/microsoft.graph.incidentReport()"
						)
					],
					update: GeneratedMacroType::FnName("health_overviews_incident_report"),
				}
			],
		ResourceIdentity::Agreements =>
			vec![
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/agreements/{{RID}}/acceptances/$count")
					],
					update: GeneratedMacroType::FnName("get_acceptances_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("list_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions")
					],
					update: GeneratedMacroType::FnName("list_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}")
					],
					update: GeneratedMacroType::FnName("get_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("create_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions")
					],
					update: GeneratedMacroType::FnName("create_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("list_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/files/{{id}}/versions")
					],
					update: GeneratedMacroType::FnName("list_files_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("update_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}")
					],
					update: GeneratedMacroType::FnName("update_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("delete_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}")
					],
					update: GeneratedMacroType::FnName("delete_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("update_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}")
					],
					update: GeneratedMacroType::FnName("update_files_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}")
					],
					update: GeneratedMacroType::FnName("get_files_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("create_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/files/{{id}}/versions")
					],
					update: GeneratedMacroType::FnName("create_files_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("delete_versions"),
						GeneratedMacroType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}")
					],
					update: GeneratedMacroType::FnName("delete_files_versions"),
				}
			],
		ResourceIdentity::AppConsent =>
			vec![
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/appConsent/appConsentRequests/$count")
					],
					update: GeneratedMacroType::FnName("get_app_consent_requests_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/appConsent/appConsentRequests/{{id}}/userConsentRequests/$count")
					],
					update: GeneratedMacroType::FnName("get_user_consent_requests_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/$count")
					],
					update: GeneratedMacroType::FnName("get_user_consent_requests_approval_stages_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("filter_by_current_user"),
						GeneratedMacroType::Path("/appConsent/appConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')")
					],
					update: GeneratedMacroType::FnName("filter_app_consent_requests_by_current_user"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("filter_by_current_user"),
						GeneratedMacroType::Path(
							"/appConsent/appConsentRequests/{{id}}/userConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id2}}')"
						)
					],
					update: GeneratedMacroType::FnName("filter_user_consent_requests_by_current_user"),
				}
			],
		ResourceIdentity::Authentication => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("get_device", "/authentication/windowsHelloForBusinessMethods/{{id}}/device"),
				],
				update: GeneratedMacroType::FnName("get_windows_hello_for_business_methods_device"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("get_device", "authentication/microsoftAuthenticatorMethods/{{id}}/device"),
				],
				update: GeneratedMacroType::FnName("get_microsoft_authenticator_methods_device"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("authentication", "/authentication/fido2Methods/{{id}}"),
					GeneratedMacroType::Method(HttpMethod::DELETE),
				],
				update: GeneratedMacroType::FnName("delete_fido_2_authentication"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("authentication", "/authentication/fido2Methods/{{id}}"),
					GeneratedMacroType::Method(HttpMethod::GET),
				],
				update: GeneratedMacroType::FnName("get_fido_2_authentication"),
			}
		],
		ResourceIdentity::AuthenticationMethodsPolicy =>
			vec![MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnName("count"),
					GeneratedMacroType::Path("/authenticationMethodConfigurations/$count")
				],
				update: GeneratedMacroType::FnName("get_authentication_method_configurations_count"),
			}],
		ResourceIdentity::ConnectedOrganizations =>
			vec![
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/connectedOrganizations/{{RID}}/internalSponsors/$count")
					],
					update: GeneratedMacroType::FnName("get_internal_sponsors_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/connectedOrganizations/{{RID}}/externalSponsors/$count")
					],
					update: GeneratedMacroType::FnName("get_external_sponsors_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("validate_properties"),
						GeneratedMacroType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.validateProperties")
					],
					update: GeneratedMacroType::FnName("validate_external_sponsors_properties"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("validate_properties"),
						GeneratedMacroType::Path("/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.validateProperties")
					],
					update: GeneratedMacroType::FnName("validate_internal_sponsors_properties"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_by_ids"),
						GeneratedMacroType::Path("/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.getByIds")
					],
					update: GeneratedMacroType::FnName("get_internal_sponsors_by_ids"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_by_ids"),
						GeneratedMacroType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.getByIds")
					],
					update: GeneratedMacroType::FnName("get_external_sponsors_by_ids"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_available_extension_properties"),
						GeneratedMacroType::Path(
							"/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.getAvailableExtensionProperties"
						)
					],
					update: GeneratedMacroType::FnName("get_internal_sponsors_available_extension_properties"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_available_extension_properties"),
						GeneratedMacroType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.getByIds")
					],
					update: GeneratedMacroType::FnName("get_external_sponsors_available_extension_properties"),
				}
			],
		ResourceIdentity::AccessReviewsDefinitionsInstances =>
			vec![
				MethodMacroModifier {
					matching: vec![GeneratedMacroType::FnName("count"), GeneratedMacroType::Path("/instances/{{RID}}/decisions/$count")],
					update: GeneratedMacroType::FnName("get_decisions_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/instances/{{RID}}/contactedReviewers/$count")
					],
					update: GeneratedMacroType::FnName("get_contacted_reviewers_count"),
				}
			],
		ResourceIdentity::DrivesItems => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnName("drive_item"),
					GeneratedMacroType::Path("/items/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')")
				],
				update: GeneratedMacroType::FnName("get_drive_item_activities_by_interval"),
			},
			MethodMacroModifier {
				matching: vec![GeneratedMacroType::FnName("drive_item"), GeneratedMacroType::Path("/items/{{RID}}/delta(token='{{id}}')")],
				update: GeneratedMacroType::FnName("get_drive_item_delta_token"),
			},
			MethodMacroModifier {
				matching: vec![GeneratedMacroType::FnName("drive_item"), GeneratedMacroType::Path("/items/{{RID}}/delta()")],
				update: GeneratedMacroType::FnName("get_drive_item_delta"),
			},
		],
		ResourceIdentity::TermsAndConditions =>
			vec![MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnName("get_terms_and_conditions"),
					GeneratedMacroType::Path("/termsAndConditions/{{RID}}/acceptanceStatuses/{{id}}/termsAndConditions")
				],
				update: GeneratedMacroType::FnName("get_acceptance_statuses_terms_and_conditions"),
			}],
		ResourceIdentity::EntitlementManagement =>
			vec![
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("filter_by_current_user"),
						GeneratedMacroType::Path("/entitlementManagement/assignments/microsoft.graph.filterByCurrentUser(on='{{id}}')")
					],
					update: GeneratedMacroType::FnName("filter_assignments_by_current_user"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("filter_by_current_user"),
						GeneratedMacroType::Path("/entitlementManagement/assignmentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')")
					],
					update: GeneratedMacroType::FnName("filter_assignment_requests_by_current_user"),
				}
			],
		ResourceIdentity::IdentityGovernance =>
			vec![
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("create_versions"),
						GeneratedMacroType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions")
					],
					update: GeneratedMacroType::FnName("create_file_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("list_versions"),
						GeneratedMacroType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions")
					],
					update: GeneratedMacroType::FnName("list_file_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_versions_count"),
						GeneratedMacroType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/$count")
					],
					update: GeneratedMacroType::FnName("get_file_localizations_versions_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("delete_versions"),
						GeneratedMacroType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}")
					],
					update: GeneratedMacroType::FnName("delete_file_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("get_versions"),
						GeneratedMacroType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}")
					],
					update: GeneratedMacroType::FnName("get_file_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("update_versions"),
						GeneratedMacroType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}")
					],
					update: GeneratedMacroType::FnName("update_file_localizations_versions"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("filter_by_current_user"),
						GeneratedMacroType::Path(
							"/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id2}}')"
						)
					],
					update: GeneratedMacroType::FnName("filter_user_consent_requests_by_current_user"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("filter_by_current_user"),
						GeneratedMacroType::Path(
							"/identityGovernance/appConsent/appConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')"
						)
					],
					update: GeneratedMacroType::FnName("filter_app_consent_requests_by_current_user"),
				}
			],
		ResourceIdentity::Teams =>
			vec![
				MethodMacroModifier {
					matching: vec![GeneratedMacroType::FnName("count"), GeneratedMacroType::Path("/teams/{{RID}}/allChannels/$count")],
					update: GeneratedMacroType::FnName("get_all_channels_count"),
				},
				MethodMacroModifier {
					matching: vec![
						GeneratedMacroType::FnName("count"),
						GeneratedMacroType::Path("/teams/{{RID}}/incomingChannels/$count")
					],
					update: GeneratedMacroType::FnName("get_incoming_channels_count"),
				},
				MethodMacroModifier {
					matching: vec![GeneratedMacroType::FnName("count"), GeneratedMacroType::Path("/teams/{{RID}}/installedApps/$count")],
					update: GeneratedMacroType::FnName("get_installed_apps_count"),
				},
				MethodMacroModifier {
					matching: vec![GeneratedMacroType::FnName("count"), GeneratedMacroType::Path("/teams/{{RID}}/operations/$count")],
					update: GeneratedMacroType::FnName("get_operations_count"),
				},
				MethodMacroModifier {
					matching: vec![GeneratedMacroType::FnName("create_team"), GeneratedMacroType::Path("/teams")],
					update: GeneratedMacroType::RequestTask(RequestTask::NoContent),
				}
			],
		ResourceIdentity::Calls =>
			vec![MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnName("mute"),
					GeneratedMacroType::Path("/calls/{{RID}}/participants/{{id}}/microsoft.graph.mute")
				],
				update: GeneratedMacroType::FnName("mute_participants"),
			}],
		ResourceIdentity::EducationAssignmentsSubmissions =>
			vec![MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnName("return"),
					GeneratedMacroType::Path("/submissions/{{RID}}/return")
				],
				update: GeneratedMacroType::FnName("submissions_return"),
			}],
		ResourceIdentity::Users => 	vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("user", "/users/{{RID}}/exportDeviceAndAppManagementData(skip={{id}},top={{id2}})"),
				],
				update: GeneratedMacroType::FnName("export_device_app_management"),
			}],
		ResourceIdentity::Chats => 	vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("get_teams_app", "/chats/{{RID}}/installedApps/{{id}}/teamsApp"),
				],
				update: GeneratedMacroType::FnName("get_installed_apps_teams_app"),
			}],
		ResourceIdentity::Insights => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("get_resource", "/insights/used/{{id}}/resource"),
				],
				update: GeneratedMacroType::FnName("get_used_resource"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("get_resource", "/insights/shared/{{id}}/resource"),
				],
				update: GeneratedMacroType::FnName("get_shared_resource"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("get_resource", "/insights/trending/{{id}}/resource"),
				],
				update: GeneratedMacroType::FnName("get_trending_resource"),
			}
		],
		ResourceIdentity::UsersMessages => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("move", "/messages/{{RID}}/move"),
				],
				update: GeneratedMacroType::FnName("move_message"),
			},
		],
		ResourceIdentity::MailFolders => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("move", "/mailFolders/{{RID}}/move"),
				],
				update: GeneratedMacroType::FnName("move_mail_folder"),
			},
		],
		ResourceIdentity::ChildFolders => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("move", "/childFolders/{{RID}}/move"),
				],
				update: GeneratedMacroType::FnName("move_child_folders"),
			},
		],
		ResourceIdentity::ServicePrincipalsOwners => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("app_role_assignment_abce", "/owners/graph.appRoleAssignment/$count"),
				],
				update: GeneratedMacroType::FnName("get_app_role_assignments_count"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("app_role_assignment_eaec", "/owners/graph.appRoleAssignment"),
				],
				update: GeneratedMacroType::FnName("get_app_role_assignments"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("app_role_assignment_eaec", "/owners/{{RID}}/graph.appRoleAssignment"),
				],
				update: GeneratedMacroType::FnName("get_app_role_assignments"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/owners/graph.user"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_user_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/owners/graph.servicePrincipal"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_service_principal_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/owners/graph.endpoint"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_endpoint_type"),
			},
		],
		ResourceIdentity::DirectReports => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/directReports/graph.orgContact"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_org_contact_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/directReports/graph.user"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_user_type"),
			},
		],
		ResourceIdentity::OwnedDevices => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/ownedDevices/graph.appRoleAssignment"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_app_role_assignment_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/ownedDevices/graph.device"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_device_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/ownedDevices/graph.endpoint"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_endpoint_type"),
			},
		],
		ResourceIdentity::OwnedObjects => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/ownedObjects/graph.group"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_group_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/ownedObjects/graph.servicePrincipal"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_service_principal_type"),
			},
		],
		ResourceIdentity::RegisteredDevices => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/registeredDevices/graph.appRoleAssignment"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_app_role_assignment_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/registeredDevices/graph.device"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_device_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/registeredDevices/graph.endpoint"),
				],
				update: GeneratedMacroType::FnName("get_directory_object_items_as_endpoint_type"),
			},
		],
		ResourceIdentity::OnenoteSectionGroups => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("get_section_groups", "/sectionGroups/{{RID}}/sectionGroups/{{id}}"),
				],
				update: GeneratedMacroType::FnName("get_section_groups_section_group"),
			},
		],
		ResourceIdentity::MobileApps => vec![
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/mobileApps/graph.managedMobileLobApp"),
				],
				update: GeneratedMacroType::FnName("get_mobile_app_items_as_managed_mobile_lob_app_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/mobileApps/graph.managedMobileLobApp"),
				],
				update: GeneratedMacroType::FnName("get_mobile_app_items_as_mobile_lob_app_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/mobileApps/{{RID}}/graph.managedMobileLobApp"),
				],
				update: GeneratedMacroType::FnName("get_mobile_app_item_as_managed_mobile_lob_app_type"),
			},
			MethodMacroModifier {
				matching: vec![
					GeneratedMacroType::FnNameAndPath("graph", "/mobileApps/{{RID}}/graph.mobileLobApp"),
				],
				update: GeneratedMacroType::FnName("get_mobile_app_item_as_mobile_lob_app_type"),
			},
		],
		ResourceIdentity::SitesItems => vec![
			MethodMacroModifier::fn_name_and_path(
				"list_item", "/items/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')",
				GeneratedMacroType::FnName("get_activities_by_interval")
			)
		],
		ResourceIdentity::TransitiveMembers => vec![
			MethodMacroModifier::fn_name_and_path(
				"application_eafb", "/transitiveMembers/graph.application/$count",
				GeneratedMacroType::FnName("get_application_count")
			)
		],
		_ => vec![],
	}
}
