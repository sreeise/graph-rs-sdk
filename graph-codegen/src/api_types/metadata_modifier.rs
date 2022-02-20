use crate::parser::filter::{MatchTarget, ModifierMap};

pub trait MetadataModifier {
    fn replace_operation_mapping(&mut self, replacement: &str);
    fn replace_operation_id(&mut self, replacement: &str);
    fn replace_operation_mapping_n(&mut self, pat: &str, replacement: &str, count: usize);
    fn replace_operation_id_n(&mut self, pat: &str, replacement: &str, count: usize);
    fn operation_mapping(&self) -> String;
    fn operation_id(&self) -> String;

    fn contains_match_target(&self, match_target: &MatchTarget) -> bool {
        match match_target {
            MatchTarget::OperationId(id) => {
                if self.operation_id().contains(id.as_str()) {
                    return true;
                }
            }
            MatchTarget::OperationMap(mapping) => {
                if self.operation_mapping().contains(mapping.as_str()) {
                    return true;
                }
            }
        }
        false
    }

    fn update_targets(&mut self, modifier_map: &ModifierMap) {
        for (match_target, match_target_vec) in modifier_map.map.iter() {
            for mat_target in match_target_vec.iter() {
                match match_target {
                    MatchTarget::OperationId(id) => {
                        println!(
                            "Match Operation Id: {:#?}\nCurrent operation id: {:#?}",
                            id,
                            self.operation_id()
                        );

                        if self.operation_id().eq(id.as_str()) {
                            match mat_target {
                                MatchTarget::OperationId(replacement) => {
                                    self.replace_operation_id(replacement.as_ref());
                                }
                                MatchTarget::OperationMap(replacement) => {
                                    self.replace_operation_mapping(replacement.as_ref());
                                }
                            }
                        }
                    }
                    MatchTarget::OperationMap(mapping) => {
                        if self.operation_mapping().eq(mapping.as_str()) {
                            match mat_target {
                                MatchTarget::OperationId(replacement) => {
                                    self.replace_operation_id(replacement.as_ref());
                                }
                                MatchTarget::OperationMap(replacement) => {
                                    self.replace_operation_mapping(replacement.as_ref());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
