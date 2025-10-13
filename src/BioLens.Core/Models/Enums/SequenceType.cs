namespace BioLens.Core.Models.Enums;

/// <summary>
/// Type of biological sequence.
/// </summary>
public enum SequenceType
{
    /// <summary>
    /// Protein amino acid sequence.
    /// </summary>
    Protein,
    
    /// <summary>
    /// DNA nucleotide sequence.
    /// </summary>
    Dna,
    
    /// <summary>
    /// RNA nucleotide sequence.
    /// </summary>
    Rna,
    
    /// <summary>
    /// Unknown or unspecified sequence type.
    /// </summary>
    Unknown
}