use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_protein_all_amino_acids() {
    // Create a protein sequence with all valid amino acid codes
    // Standard 20 amino acids plus B, Z, X, J, O, U and *
    let content = ">Complete_Protein\nACDEFGHIKLMNPQRSTVWYBZXJOU*";
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Protein, // Force protein validation
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Debug output if any issues
    for issue in &result.issues {
        println!("Issue: {:?} - {}", issue.level, issue.message);
    }

    // All valid amino acid codes should be accepted
    assert!(
        result.is_valid,
        "FASTA with all valid amino acid codes should be valid"
    );

    // Check that sequence type is detected as protein
    assert_eq!(
        result.detected_sequence_type,
        Some(SequenceType::Protein),
        "Sequence type should be detected as protein"
    );
}
