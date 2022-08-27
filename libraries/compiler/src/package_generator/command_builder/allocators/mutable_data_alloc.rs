use crate::package_generator::utils::{align_array_width, convert_to_u8_array};
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::error::pkg_gen_issue::PackageGenerationIssue;
use crate::shared::package_generation::data_descriptor::DataAccessDescriptor;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

/// dac : Data access command
/// # Command scheme:
/// ```text
/// [0] :           0x00 instant value
///                 0x01 local variable
///                 0x02 string from static string heap
/// [from 1] :      data slot (when [0] is 0x00, it is the instant data binary)
/// ```
pub fn dac_builder(data: DataAccessDescriptor, metadata: &PackageMetadata) -> Result<RelocatableCommandList, GeneralIssue<PackageGenerationIssue>> {
    let mut result = vec![];

    if data.identifier.is_some() {
        let identifier = data.identifier.unwrap();
        result.push(0x01);

        // Push identifier slot
        result.extend(align_array_width(&identifier.slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));
    } else if data.instant_value.is_some() {
        let value = data.instant_value.unwrap();
        result.push(0x00);

        // Push value
        result.extend(align_array_width(&convert_to_u8_array(value), metadata.data_alignment));
    } else if data.string_constant.is_some() {
        let string_value = data.string_constant.unwrap();
        result.push(0x02);

        // Push slot id on string heap
        result.extend(align_array_width(&string_value.slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));
    } else {
        return Err(GeneralIssue {
            issues: vec![IssueBase {
                level: IssueLevel::Error,
                position: IssuePosition::CodeGeneration,
                code: "-1".to_string(),
                detail: PackageGenerationIssue {},
            }]
        });
    }

    return Ok(RelocatableCommandList::new_no_relocation(result));
}
