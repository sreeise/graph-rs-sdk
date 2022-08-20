use crate::api_types::RequestTask;
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
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub struct MethodMacroModifier {
    pub matching: Vec<MacroModifierType>,
    pub update: MacroModifierType,
}
/*
   get!({
       doc: "Get the number of the resource",
       name: internal_sponsors_count,
       response: serde_json::Value,
       path: "/connectedOrganizations/{{RID}}/internalSponsors/$count",
       has_body: false
   });

       get!({
       doc: "Get the number of the resource",
       name: count,
       response: serde_json::Value,
       path: "/connectedOrganizations/{{RID}}/externalSponsors/$count",
       has_body: false
   });

       post!({
       doc: "Invoke action validateProperties",
       name: validate_properties,
       response: NoContent,
       path: "/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.validateProperties",
       has_body: true
   });
*/
pub fn get_method_macro_modifiers(resource_identity: ResourceIdentity) -> Vec<MethodMacroModifier> {
    match resource_identity {
        ResourceIdentity::AccessReviews => vec![
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/accessReviews/historyDefinitions/$count".into()),
                ],
                update: MacroModifierType::FnName("get_history_definitions_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/accessReviews/historyDefinitions/{{id}}/instances/$count".into()),
                ],
                update: MacroModifierType::FnName("get_history_definitions_instances_count".into()),
            },
        ],
        ResourceIdentity::Agreements => vec![
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/acceptances/$count".into()),
                ],
                update: MacroModifierType::FnName("get_acceptances_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("list_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions".into()),
                ],
                update: MacroModifierType::FnName("list_localizations_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("get_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}".into()),
                ],
                update: MacroModifierType::FnName("get_localizations_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("create_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions".into()),
                ],
                update: MacroModifierType::FnName("create_localizations_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("list_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions".into()),
                ],
                update: MacroModifierType::FnName("list_files_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("update_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}".into()),
                ],
                update: MacroModifierType::FnName("update_localizations_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("delete_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}".into()),
                ],
                update: MacroModifierType::FnName("delete_localizations_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("update_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}".into()),
                ],
                update: MacroModifierType::FnName("update_files_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("get_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}".into()),
                ],
                update: MacroModifierType::FnName("get_files_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("create_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions".into()),
                ],
                update: MacroModifierType::FnName("create_files_versions".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("delete_versions".into()),
                    MacroModifierType::Path("/agreements/{{RID}}/files/{{id}}/versions/{{id2}}".into()),
                ],
                update: MacroModifierType::FnName("delete_files_versions".into()),
            }
        ],
        ResourceIdentity::AppConsent => vec![
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/appConsent/appConsentRequests/$count".into()),
                ],
                update: MacroModifierType::FnName("get_app_consent_requests_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/appConsent/appConsentRequests/{{id}}/userConsentRequests/$count".into()),
                ],
                update: MacroModifierType::FnName("get_user_consent_requests_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/$count".into()),
                ],
                update: MacroModifierType::FnName("get_user_consent_requests_approval_stages_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("filter_by_current_user".into()),
                    MacroModifierType::Path("/appConsent/appConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')".into()),
                ],
                update: MacroModifierType::FnName("filter_app_consent_requests_by_current_user".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("filter_by_current_user".into()),
                    MacroModifierType::Path("/appConsent/appConsentRequests/{{id}}/userConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id2}}')".into()),
                ],
                update: MacroModifierType::FnName("filter_user_consent_requests_by_current_user".into()),
            },
        ],
        ResourceIdentity::ConnectedOrganizations => vec![
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/connectedOrganizations/{{RID}}/internalSponsors/$count".into()),
                ],
                update: MacroModifierType::FnName("get_internal_sponsors_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/connectedOrganizations/{{RID}}/externalSponsors/$count".into()),
                ],
                update: MacroModifierType::FnName("get_external_sponsors_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("validate_properties".into()),
                    MacroModifierType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.validateProperties".into()),
                ],
                update: MacroModifierType::FnName("validate_external_sponsors_properties".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("validate_properties".into()),
                    MacroModifierType::Path("/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.validateProperties".into()),
                ],
                update: MacroModifierType::FnName("validate_internal_sponsors_properties".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("get_by_ids".into()),
                    MacroModifierType::Path("/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.getByIds".into()),
                ],
                update: MacroModifierType::FnName("get_internal_sponsors_by_ids".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("get_by_ids".into()),
                    MacroModifierType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.getByIds".into()),
                ],
                update: MacroModifierType::FnName("get_external_sponsors_by_ids".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("get_available_extension_properties".into()),
                    MacroModifierType::Path("/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.getAvailableExtensionProperties".into()),
                ],
                update: MacroModifierType::FnName("get_internal_sponsors_available_extension_properties".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("get_available_extension_properties".into()),
                    MacroModifierType::Path("/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.getByIds".into()),
                ],
                update: MacroModifierType::FnName("get_external_sponsors_available_extension_properties".into()),
            },
        ],
        ResourceIdentity::DefinitionInstances => vec![
          MethodMacroModifier {
              matching: vec![
                  MacroModifierType::FnName("count".into()),
                  MacroModifierType::Path("/instances/{{RID}}/decisions/$count".into()),
              ],
              update: MacroModifierType::FnName("get_decisions_count".into()),
          },
          MethodMacroModifier {
              matching: vec![
                  MacroModifierType::FnName("count".into()),
                  MacroModifierType::Path("/instances/{{RID}}/contactedReviewers/$count".into()),
              ],
              update: MacroModifierType::FnName("get_contacted_reviewers_count".into()),
          },
        ],
        ResourceIdentity::Teams => vec![
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/teams/{{RID}}/allChannels/$count".into()),
                ],
                update: MacroModifierType::FnName("get_all_channels_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/teams/{{RID}}/incomingChannels/$count".into()),
                ],
                update: MacroModifierType::FnName("get_incoming_channels_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/teams/{{RID}}/installedApps/$count".into()),
                ],
                update: MacroModifierType::FnName("get_installed_apps_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("count".into()),
                    MacroModifierType::Path("/teams/{{RID}}/operations/$count".into()),
                ],
                update: MacroModifierType::FnName("get_operations_count".into()),
            },
            MethodMacroModifier {
                matching: vec![
                    MacroModifierType::FnName("create_team".into()),
                    MacroModifierType::Path("/teams".into()),
                ],
                update: MacroModifierType::RequestTask(RequestTask::NoContent),
            },
        ],
        _ => vec![],
    }
}
