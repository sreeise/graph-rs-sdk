use crate::api_types::RequestTask;
use crate::parser::HttpMethod;
use graph_core::resource::ResourceIdentity;

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
		ResourceIdentity::Drives => vec![
			MethodMacroModifier::fn_name_and_path(
				"special", "/drives/{{RID}}/special/$count",
				GeneratedMacroType::FnName("get_special_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"following", "/drives/{{RID}}/following/$count",
				GeneratedMacroType::FnName("get_following_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"bundles", "/drives/{{RID}}/bundles/$count",
				GeneratedMacroType::FnName("get_bundles_count")
			),
		],
		ResourceIdentity::DrivesList => vec![
			MethodMacroModifier::fn_name_and_path(
				"subscriptions", "/list/subscriptions/$count",
				GeneratedMacroType::FnName("get_subscriptions_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"operations", "/list/operations/$count",
				GeneratedMacroType::FnName("get_operations_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"columns", "/list/columns/$count",
				GeneratedMacroType::FnName("get_columns_count")
			),
		],
		ResourceIdentity::DrivesItems => vec![
			MethodMacroModifier::fn_name_and_path(
				"children", "/items/{{RID}}/children/$count",
				GeneratedMacroType::FnName("get_children_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"items", "/items/$count",
				GeneratedMacroType::FnName("get_items_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"item_activity_stats", "/items/{{RID}}/analytics/itemActivityStats/$count",
				GeneratedMacroType::FnName("get_item_activity_stats_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"activities", "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/$count",
				GeneratedMacroType::FnName("get_activities_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"permissions", "/items/{{RID}}/permissions/$count",
				GeneratedMacroType::FnName("get_permissions_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"thumbnails", "/items/{{RID}}/thumbnails/$count",
				GeneratedMacroType::FnName("get_thumbnails_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"versions", "/items/{{RID}}/versions/$count",
				GeneratedMacroType::FnName("get_versions_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"subscriptions", "/list/subscriptions/$count",
				GeneratedMacroType::FnName("get_subscriptions_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"columns", "/list/columns/$count",
				GeneratedMacroType::FnName("get_columns_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"operations", "/list/operations/$count",
				GeneratedMacroType::FnName("get_operations_count")
			),
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
		ResourceIdentity::DrivesListContentTypes => vec![
			MethodMacroModifier::fn_name_and_path(
				"content_types", "/contentTypes/$count",
				GeneratedMacroType::FnName("get_content_types_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"base_types", "/contentTypes/{{RID}}/baseTypes/$count",
				GeneratedMacroType::FnName("get_base_types_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"column_links", "/contentTypes/{{RID}}/columnLinks/$count",
				GeneratedMacroType::FnName("get_column_links_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"column_positions", "/contentTypes/{{RID}}/columnPositions/$count",
				GeneratedMacroType::FnName("get_column_positions_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"columns", "/contentTypes/{{RID}}/columns/$count",
				GeneratedMacroType::FnName("get_columns_count")
			),
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
		/*
		    get!(
        doc: "Get the number of the resource",
        name: worksheets,
        path: "/worksheets/$count"
    );
		 */
        ResourceIdentity::Worksheets => vec![
			MethodMacroModifier::fn_name_and_path(
				"worksheets", "/worksheets/$count",
				GeneratedMacroType::FnName("get_worksheets_count")
			),
            MethodMacroModifier::fn_name_and_path(
                "workbook_worksheet", "/worksheets/{{RID}}/range()",
                GeneratedMacroType::FnName("get_range_object")
            ),
            MethodMacroModifier::fn_name_and_path(
                "workbook_worksheet", "/worksheets/{{RID}}/range(address='{{id}}')",
                GeneratedMacroType::FnName("get_range_object_by_address")
            ),
            MethodMacroModifier::fn_name_and_path(
                "workbook_worksheet", "/worksheets/{{RID}}/usedRange()",
                GeneratedMacroType::FnName("get_used_range_object")
            ),
            MethodMacroModifier::fn_name_and_path(
                "workbook_worksheet", "/worksheets/{{RID}}/usedRange(valuesOnly={{id}})",
                GeneratedMacroType::FnName("get_used_range_object_with_values_only")
            )
        ],
		ResourceIdentity::WorksheetsCharts => vec![
			MethodMacroModifier::fn_name_and_path(
				"get_worksheet", "/charts/itemAt(index={{id}})/worksheet",
				GeneratedMacroType::FnName("item_at_get_worksheet")
			),
			MethodMacroModifier::fn_name_and_path(
				"set_data", "/charts/itemAt(index={{id}})/setData",
				GeneratedMacroType::FnName("item_at_set_data")
			),
			MethodMacroModifier::fn_name_and_path(
				"set_position", "/charts/itemAt(index={{id}})/setPosition",
				GeneratedMacroType::FnName("item_at_set_position")
			),
			MethodMacroModifier::fn_name_and_path(
				"item_at", "/charts/itemAt(index={{id}})/image(width={{id2}},height={{id3}},fittingMode='{{id4}}')",
				GeneratedMacroType::FnName("item_at_image_by_width_and_height_and_fitting_mode")
			),
			MethodMacroModifier::fn_name_and_path(
				"item_at", "/charts/itemAt(index={{id}})/image(width={{id2}},height={{id3}})",
				GeneratedMacroType::FnName("item_at_image_by_width_and_height")
			),
			MethodMacroModifier::fn_name_and_path(
				"item_at", "/charts/itemAt(index={{id}})/image(width={{id2}})",
				GeneratedMacroType::FnName("item_at_image_by_width")
			),
			MethodMacroModifier::fn_name_and_path(
				"item_at", "/charts/itemAt(index={{id}})/image()",
				GeneratedMacroType::FnName("item_at_image")
			),
			MethodMacroModifier::fn_name_and_path(
				"count", "/charts/count()",
				GeneratedMacroType::FnName("get_charts_count")
			),
			MethodMacroModifier::fn_name_and_path(
				"item", "/charts/item(name='{{id}}')/image(width={{id2}},height={{id3}},fittingMode='{{id4}}')",
				GeneratedMacroType::FnName("get_charts_item_by_name_and_width_and_height_and_fitting_mode")
			),
			MethodMacroModifier::fn_name_and_path(
				"item", "/charts/item(name='{{id}}')/image(width={{id2}},height={{id3}})",
				GeneratedMacroType::FnName("get_charts_item_by_name_and_width_and_height")
			),
			MethodMacroModifier::fn_name_and_path(
				"item", "/charts/item(name='{{id}}')/image(width={{id2}})",
				GeneratedMacroType::FnName("get_charts_item_by_name_and_width")
			),
			MethodMacroModifier::fn_name_and_path(
				"item", "/charts/item(name='{{id}}')/image()",
				GeneratedMacroType::FnName("get_charts_item_image_by_name")
			),
			MethodMacroModifier::fn_name_and_path(
				"item", "/charts/item(name='{{id}}')",
				GeneratedMacroType::FnName("get_charts_item_by_name")
			),
			MethodMacroModifier::fn_name_and_path(
				"delete_format", "/charts/{{RID}}/title/format",
				GeneratedMacroType::FnName("delete_title_format")
			),
			MethodMacroModifier::fn_name_and_path(
				"get_format", "/charts/{{RID}}/title/format",
				GeneratedMacroType::FnName("get_title_format")
			),
			MethodMacroModifier::fn_name_and_path(
				"update_format", "/charts/{{RID}}/title/format",
				GeneratedMacroType::FnName("update_title_format")
			),
			MethodMacroModifier::fn_name_and_path(
				"delete_fill", "/charts/{{RID}}/title/format/fill",
				GeneratedMacroType::FnName("delete_title_format_fill")
			),
			MethodMacroModifier::fn_name_and_path(
				"get_fill", "/charts/{{RID}}/title/format/fill",
				GeneratedMacroType::FnName("get_title_format_fill")
			),
			MethodMacroModifier::fn_name_and_path(
				"update_fill", "/charts/{{RID}}/title/format/fill",
				GeneratedMacroType::FnName("update_title_format_fill")
			),
			MethodMacroModifier::fn_name_and_path(
				"clear", "/charts/{{RID}}/title/format/fill/clear",
				GeneratedMacroType::FnName("clear_title_format_fill")
			),
			MethodMacroModifier::fn_name_and_path(
				"set_solid_color", "/charts/{{RID}}/title/format/fill/setSolidColor",
				GeneratedMacroType::FnName("set_title_format_fill_solid_color")
			),
			MethodMacroModifier::fn_name_and_path(
				"delete_font", "/charts/{{RID}}/title/format/font",
				GeneratedMacroType::FnName("delete_title_format_font")
			),
			MethodMacroModifier::fn_name_and_path(
				"get_font", "/charts/{{RID}}/title/format/font",
				GeneratedMacroType::FnName("get_title_format_font")
			),
			MethodMacroModifier::fn_name_and_path(
				"update_font", "/charts/{{RID}}/title/format/font",
				GeneratedMacroType::FnName("update_title_format_font")
			),
			MethodMacroModifier::fn_name_and_path(
				"workbook_chart",  "/charts/{{RID}}/image()",
				GeneratedMacroType::FnName("get_image")
			),
			MethodMacroModifier::fn_name_and_path(
				"workbook_chart",  "/charts/{{RID}}/image(width={{id}})",
				GeneratedMacroType::FnName("get_image_with_width")
			),
			MethodMacroModifier::fn_name_and_path(
				"workbook_chart",  "/charts/{{RID}}/image(width={{id}},height={{id2}})",
				GeneratedMacroType::FnName("get_image_with_width_and_height")
			),
			MethodMacroModifier::fn_name_and_path(
				"workbook_chart",  "/charts/{{RID}}/image(width={{id}},height={{id2}},fittingMode='{{id3}}')",
				GeneratedMacroType::FnName("get_image_with_width_and_height_and_fitting_mode")
			),
		],
		ResourceIdentity::WorkbookFunctions => vec![
			MethodMacroModifier::fn_name_and_path(
				"false",    "/functions/false",
				GeneratedMacroType::FnName("_false")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",    "/functions/t_Inv_2T",
				GeneratedMacroType::FnName("t_inv_2t")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",    "/functions/t_Dist_2T",
				GeneratedMacroType::FnName("t_dist_2t")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",    "/functions/oct2Bin",
				GeneratedMacroType::FnName("oct_2_bin")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",    "/functions/oct2Dec",
				GeneratedMacroType::FnName("oct_2_dec")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",    "/functions/oct2Hex",
				GeneratedMacroType::FnName("oct_2_hex")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",    "/functions/log10",
				GeneratedMacroType::FnName("log_10")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",    "/functions/imLog2",
				GeneratedMacroType::FnName("im_log_2")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",   "/functions/imLog10",
				GeneratedMacroType::FnName("im_log_10")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",    "/functions/hex2Bin",
				GeneratedMacroType::FnName("hex_2b_in")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",   "/functions/hex2Dec",
				GeneratedMacroType::FnName("hex_2_dec")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",  "/functions/hex2Oct",
				GeneratedMacroType::FnName("hex_20_ct")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",   "/functions/days360",
				GeneratedMacroType::FnName("days_360")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",   "/functions/dec2Bin",
				GeneratedMacroType::FnName("dec_2b_in")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",  "/functions/dec2Hex",
				GeneratedMacroType::FnName("dec_2_hex")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",  "/functions/dec2Oct",
				GeneratedMacroType::FnName("dec_20_ct")
			),

			MethodMacroModifier::fn_name_and_path(
				"if",  "/functions/if",
				GeneratedMacroType::FnName("_if")
			),
			MethodMacroModifier::fn_name_and_path(
				"match",  "/functions/match",
				GeneratedMacroType::FnName("_match")
			),
			MethodMacroModifier::fn_name_and_path(
				"mod",  "/functions/mod",
				GeneratedMacroType::FnName("_mod")
			),
			MethodMacroModifier::fn_name_and_path(
				"true",  "/functions/true",
				GeneratedMacroType::FnName("_true")
			),
			MethodMacroModifier::fn_name_and_path(
				"type",  "/functions/type",
				GeneratedMacroType::FnName("_type")
			),
			MethodMacroModifier::fn_name_and_path(
				"yield",  "/functions/yield",
				GeneratedMacroType::FnName("_yield")
			),

			MethodMacroModifier::fn_name_and_path(
				"functions",  "/functions/bin2Dec",
				GeneratedMacroType::FnName("bin_2_dec")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",  "/functions/bin2Hex",
				GeneratedMacroType::FnName("bin_2_hex")
			),
			MethodMacroModifier::fn_name_and_path(
				"functions",  "/functions/bin2Oct",
				GeneratedMacroType::FnName("bin_20_ct")
			),
		],
		_ => vec![],
	}
}
