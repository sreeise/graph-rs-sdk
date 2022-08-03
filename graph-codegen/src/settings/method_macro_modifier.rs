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
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub struct MethodMacroModifier {
    pub matching: Vec<MacroModifierType>,
    pub update: MacroModifierType,
}

pub fn get_method_macro_modifiers(resource_identity: ResourceIdentity) -> Vec<MethodMacroModifier> {
    match resource_identity {
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
        ],
        _ => vec![],
    }
}
