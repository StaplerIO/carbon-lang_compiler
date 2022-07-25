use crate::shared::package_generation::relocation_reference::{RelocationReference, RelocationReferenceType};

pub fn is_domain_create_command(reloc_ref: &RelocationReference) -> bool {
    return reloc_ref.ref_type == RelocationReferenceType::WhileEntrance ||
        reloc_ref.ref_type == RelocationReferenceType::IfEntrance ||
        reloc_ref.ref_type == RelocationReferenceType::ElifEntrance ||
        reloc_ref.ref_type == RelocationReferenceType::ElseEntrance ||
        reloc_ref.ref_type == RelocationReferenceType::LoopEntrance ||
        reloc_ref.ref_type == RelocationReferenceType::FunctionEntrance;
}

pub fn is_domain_destroy_command(reloc_ref: &RelocationReference) -> bool {
    return reloc_ref.ref_type == RelocationReferenceType::EndWhile ||
        reloc_ref.ref_type == RelocationReferenceType::EndIf ||
        reloc_ref.ref_type == RelocationReferenceType::EndElif ||
        reloc_ref.ref_type == RelocationReferenceType::EndElse ||
        reloc_ref.ref_type == RelocationReferenceType::EndLoop ||
        reloc_ref.ref_type == RelocationReferenceType::EndFunction;
}
