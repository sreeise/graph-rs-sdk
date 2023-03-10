use crate::api_types::RequestTask;
use crate::parser::HttpMethod;
use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(
    Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, FromFile, AsFile,
)]
pub enum MacroModifierType {
    FnName(String),
    Path(String),
    ParamSize(usize),
    RequestTask(RequestTask),
    FnNameAndPath(String, String),
    Method(HttpMethod),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub struct MethodMacroModifier {
    pub matching: Vec<MacroModifierType>,
    pub update: MacroModifierType,
}

impl MethodMacroModifier {
    pub fn fn_name_by_path(
        current_fn_name: &str,
        path: &str,
        new_fn_name: &str,
    ) -> MethodMacroModifier {
        MethodMacroModifier {
            matching: vec![
                MacroModifierType::FnName(current_fn_name.into()),
                MacroModifierType::Path(path.into()),
            ],
            update: MacroModifierType::FnName(new_fn_name.into()),
        }
    }
}

pub fn get_method_macro_modifiers(resource_identity: ResourceIdentity) -> Vec<MethodMacroModifier> {
    match resource_identity {
		ResourceIdentity::AccessReviewsDefinitions =>
			vec![
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("stop".into()),
						MacroModifierType::Path("/definitions/{{RID}}/instances/{{id}}/microsoft.graph.stop".into())
					],
					update: MacroModifierType::FnName("stop_instances".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("filter_by_current_user".into()),
						MacroModifierType::Path("/definitions/{{RID}}/instances/microsoft.graph.filterByCurrentUser(on='{{id}}')".into())
					],
					update: MacroModifierType::FnName("filter_instances_by_current_user".into()),
				}
			],
		ResourceIdentity::AccessReviews =>
			vec![
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/accessReviews/historyDefinitions/$count".into())
					],
					update: MacroModifierType::FnName("get_history_definitions_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/accessReviews/historyDefinitions/{{id}}/instances/$count".into())
					],
					update: MacroModifierType::FnName("get_history_definitions_instances_count".into()),
				}
			],
		ResourceIdentity::Admin =>
			vec![
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("create_issues".into()),
						MacroModifierType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues".into())
					],
					update: MacroModifierType::FnName("create_health_overview_issues".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("list_issues".into()),
						MacroModifierType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues".into())
					],
					update: MacroModifierType::FnName("list_health_overview_issues".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_issues_count".into()),
						MacroModifierType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/$count".into())
					],
					update: MacroModifierType::FnName("get_health_overview_issues_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("delete_issues".into()),
						MacroModifierType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}".into())
					],
					update: MacroModifierType::FnName("delete_health_overview_issues".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_issues".into()),
						MacroModifierType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}".into())
					],
					update: MacroModifierType::FnName("get_health_overview_issues".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("update_issues".into()),
						MacroModifierType::Path("/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}".into())
					],
					update: MacroModifierType::FnName("update_health_overview_issues".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("incident_report".into()),
						MacroModifierType::Path(
							"/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}/microsoft.graph.incidentReport()".into()
						)
					],
					update: MacroModifierType::FnName("health_overviews_incident_report".into()),
				}
			],
		ResourceIdentity::Agreements =>
			vec![
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/agreements/{{RID}}/acceptances/$count".into())
					],
					update: MacroModifierType::FnName("get_acceptances_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("list_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions".into())
					],
					update: MacroModifierType::FnName("list_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}".into())
					],
					update: MacroModifierType::FnName("get_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("create_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions".into())
					],
					update: MacroModifierType::FnName("create_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("list_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions".into())
					],
					update: MacroModifierType::FnName("list_files_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("update_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}".into())
					],
					update: MacroModifierType::FnName("update_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("delete_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}".into())
					],
					update: MacroModifierType::FnName("delete_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("update_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}".into())
					],
					update: MacroModifierType::FnName("update_files_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}".into())
					],
					update: MacroModifierType::FnName("get_files_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("create_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions".into())
					],
					update: MacroModifierType::FnName("create_files_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("delete_versions".into()),
						MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}".into())
					],
					update: MacroModifierType::FnName("delete_files_versions".into()),
				}
			],
		ResourceIdentity::AppConsent =>
			vec![
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/appConsent/appConsentRequests/$count".into())
					],
					update: MacroModifierType::FnName("get_app_consent_requests_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/appConsent/appConsentRequests/{{id}}/userConsentRequests/$count".into())
					],
					update: MacroModifierType::FnName("get_user_consent_requests_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/$count".into())
					],
					update: MacroModifierType::FnName("get_user_consent_requests_approval_stages_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("filter_by_current_user".into()),
						MacroModifierType::Path("/appConsent/appConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')".into())
					],
					update: MacroModifierType::FnName("filter_app_consent_requests_by_current_user".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("filter_by_current_user".into()),
						MacroModifierType::Path(
							"/appConsent/appConsentRequests/{{id}}/userConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id2}}')".into()
						)
					],
					update: MacroModifierType::FnName("filter_user_consent_requests_by_current_user".into()),
				}
			],
		ResourceIdentity::Authentication => vec![
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("get_device".into(), "/authentication/windowsHelloForBusinessMethods/{{id}}/device".into()),
				],
				update: MacroModifierType::FnName("get_windows_hello_for_business_methods_device".into()),
			},
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("get_device".into(), "authentication/microsoftAuthenticatorMethods/{{id}}/device".into()),
				],
				update: MacroModifierType::FnName("get_microsoft_authenticator_methods_device".into()),
			},
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("authentication".into(), "/authentication/fido2Methods/{{id}}".into()),
					MacroModifierType::Method(HttpMethod::DELETE),
				],
				update: MacroModifierType::FnName("delete_fido_2_authentication".into()),
			},
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("authentication".into(), "/authentication/fido2Methods/{{id}}".into()),
					MacroModifierType::Method(HttpMethod::GET),
				],
				update: MacroModifierType::FnName("get_fido_2_authentication".into()),
			}
		],
		ResourceIdentity::AuthenticationMethodsPolicy =>
			vec![MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnName("count".into()),
					MacroModifierType::Path("/authenticationMethodConfigurations/$count".into())
				],
				update: MacroModifierType::FnName("get_authentication_method_configurations_count".into()),
			}],
		ResourceIdentity::ConnectedOrganizations =>
			vec![
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/connectedOrganizations/{{RID}}/internalSponsors/$count".into())
					],
					update: MacroModifierType::FnName("get_internal_sponsors_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/connectedOrganizations/{{RID}}/externalSponsors/$count".into())
					],
					update: MacroModifierType::FnName("get_external_sponsors_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("validate_properties".into()),
						MacroModifierType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.validateProperties".into())
					],
					update: MacroModifierType::FnName("validate_external_sponsors_properties".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("validate_properties".into()),
						MacroModifierType::Path("/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.validateProperties".into())
					],
					update: MacroModifierType::FnName("validate_internal_sponsors_properties".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_by_ids".into()),
						MacroModifierType::Path("/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.getByIds".into())
					],
					update: MacroModifierType::FnName("get_internal_sponsors_by_ids".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_by_ids".into()),
						MacroModifierType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.getByIds".into())
					],
					update: MacroModifierType::FnName("get_external_sponsors_by_ids".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_available_extension_properties".into()),
						MacroModifierType::Path(
							"/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.getAvailableExtensionProperties".into()
						)
					],
					update: MacroModifierType::FnName("get_internal_sponsors_available_extension_properties".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_available_extension_properties".into()),
						MacroModifierType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.getByIds".into())
					],
					update: MacroModifierType::FnName("get_external_sponsors_available_extension_properties".into()),
				}
			],
		ResourceIdentity::AccessReviewsDefinitionsInstances =>
			vec![
				MethodMacroModifier {
					matching: vec![MacroModifierType::FnName("count".into()), MacroModifierType::Path("/instances/{{RID}}/decisions/$count".into())],
					update: MacroModifierType::FnName("get_decisions_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/instances/{{RID}}/contactedReviewers/$count".into())
					],
					update: MacroModifierType::FnName("get_contacted_reviewers_count".into()),
				}
			],
		ResourceIdentity::DrivesItems => vec![
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnName("drive_item".into()),
					MacroModifierType::Path("/items/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')".into())
				],
				update: MacroModifierType::FnName("get_drive_item_activities_by_interval".into()),
			},
			MethodMacroModifier {
				matching: vec![MacroModifierType::FnName("drive_item".into()), MacroModifierType::Path("/items/{{RID}}/delta(token='{{id}}')".into())],
				update: MacroModifierType::FnName("get_drive_item_delta_token".into()),
			},
			MethodMacroModifier {
				matching: vec![MacroModifierType::FnName("drive_item".into()), MacroModifierType::Path("/items/{{RID}}/delta()".into())],
				update: MacroModifierType::FnName("get_drive_item_delta".into()),
			},
		],
		ResourceIdentity::TermsAndConditions =>
			vec![MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnName("get_terms_and_conditions".into()),
					MacroModifierType::Path("/termsAndConditions/{{RID}}/acceptanceStatuses/{{id}}/termsAndConditions".into())
				],
				update: MacroModifierType::FnName("get_acceptance_statuses_terms_and_conditions".into()),
			}],
		ResourceIdentity::EntitlementManagement =>
			vec![
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("filter_by_current_user".into()),
						MacroModifierType::Path("/entitlementManagement/assignments/microsoft.graph.filterByCurrentUser(on='{{id}}')".into())
					],
					update: MacroModifierType::FnName("filter_assignments_by_current_user".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("filter_by_current_user".into()),
						MacroModifierType::Path("/entitlementManagement/assignmentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')".into())
					],
					update: MacroModifierType::FnName("filter_assignment_requests_by_current_user".into()),
				}
			],
		ResourceIdentity::IdentityGovernance =>
			vec![
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("create_versions".into()),
						MacroModifierType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions".into())
					],
					update: MacroModifierType::FnName("create_file_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("list_versions".into()),
						MacroModifierType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions".into())
					],
					update: MacroModifierType::FnName("list_file_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_versions_count".into()),
						MacroModifierType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/$count".into())
					],
					update: MacroModifierType::FnName("get_file_localizations_versions_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("delete_versions".into()),
						MacroModifierType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}".into())
					],
					update: MacroModifierType::FnName("delete_file_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("get_versions".into()),
						MacroModifierType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}".into())
					],
					update: MacroModifierType::FnName("get_file_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("update_versions".into()),
						MacroModifierType::Path("/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}".into())
					],
					update: MacroModifierType::FnName("update_file_localizations_versions".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("filter_by_current_user".into()),
						MacroModifierType::Path(
							"/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id2}}')".into()
						)
					],
					update: MacroModifierType::FnName("filter_user_consent_requests_by_current_user".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("filter_by_current_user".into()),
						MacroModifierType::Path(
							"/identityGovernance/appConsent/appConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')".into()
						)
					],
					update: MacroModifierType::FnName("filter_app_consent_requests_by_current_user".into()),
				}
			],
		ResourceIdentity::Teams =>
			vec![
				MethodMacroModifier {
					matching: vec![MacroModifierType::FnName("count".into()), MacroModifierType::Path("/teams/{{RID}}/allChannels/$count".into())],
					update: MacroModifierType::FnName("get_all_channels_count".into()),
				},
				MethodMacroModifier {
					matching: vec![
						MacroModifierType::FnName("count".into()),
						MacroModifierType::Path("/teams/{{RID}}/incomingChannels/$count".into())
					],
					update: MacroModifierType::FnName("get_incoming_channels_count".into()),
				},
				MethodMacroModifier {
					matching: vec![MacroModifierType::FnName("count".into()), MacroModifierType::Path("/teams/{{RID}}/installedApps/$count".into())],
					update: MacroModifierType::FnName("get_installed_apps_count".into()),
				},
				MethodMacroModifier {
					matching: vec![MacroModifierType::FnName("count".into()), MacroModifierType::Path("/teams/{{RID}}/operations/$count".into())],
					update: MacroModifierType::FnName("get_operations_count".into()),
				},
				MethodMacroModifier {
					matching: vec![MacroModifierType::FnName("create_team".into()), MacroModifierType::Path("/teams".into())],
					update: MacroModifierType::RequestTask(RequestTask::NoContent),
				}
			],
		ResourceIdentity::Calls =>
			vec![MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnName("mute".into()),
					MacroModifierType::Path("/calls/{{RID}}/participants/{{id}}/microsoft.graph.mute".into())
				],
				update: MacroModifierType::FnName("mute_participants".into()),
			}],
		ResourceIdentity::EducationAssignmentsSubmissions =>
			vec![MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnName("return".into()),
					MacroModifierType::Path("/submissions/{{RID}}/return".into())
				],
				update: MacroModifierType::FnName("submissions_return".into()),
			}],
		ResourceIdentity::Users => 	vec![
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("user".into(), "/users/{{RID}}/exportDeviceAndAppManagementData(skip={{id}},top={{id2}})".into()),
				],
				update: MacroModifierType::FnName("export_device_app_management".into()),
			}],
		ResourceIdentity::Chats => 	vec![
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("get_teams_app".into(), "/chats/{{RID}}/installedApps/{{id}}/teamsApp".into()),
				],
				update: MacroModifierType::FnName("get_installed_apps_teams_app".into()),
			}],
		ResourceIdentity::Insights => vec![
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("get_resource".into(), "/insights/used/{{id}}/resource".into()),
				],
				update: MacroModifierType::FnName("get_used_resource".into()),
			},
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("get_resource".into(), "/insights/shared/{{id}}/resource".into()),
				],
				update: MacroModifierType::FnName("get_shared_resource".into()),
			},
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("get_resource".into(), "/insights/trending/{{id}}/resource".into()),
				],
				update: MacroModifierType::FnName("get_trending_resource".into()),
			}
		],
		ResourceIdentity::UsersMessages => vec![
			MethodMacroModifier {
				matching: vec![
					MacroModifierType::FnNameAndPath("move".into(), "/messages/{{RID}}/move".into()),
				],
				update: MacroModifierType::FnName("move_messages".into()),
			},
		],
		_ => vec![],
	}
}
