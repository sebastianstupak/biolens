using BioLens.Core.Models.Enums;

namespace BioLens.Core.Models;

/// <summary>
/// Represents detailed information about a biological sequence.
/// </summary>
public class Sequence
{
    /// <summary>
    /// The actual sequence string (amino acids or nucleotides).
    /// </summary>
    public required string Value { get; init; }
    
    /// <summary>
    /// Length of the sequence.
    /// </summary>
    public required int Length { get; init; }
    
    /// <summary>
    /// Type of biological sequence.
    /// </summary>
    public SequenceType Type { get; init; }
    
    /// <summary>
    /// Molecular weight in Daltons.
    /// </summary>
    public int? MolecularWeight { get; init; }
    
    /// <summary>
    /// CRC64 checksum for sequence integrity verification.
    /// </summary>
    public string? Crc64 { get; init; }
    
    /// <summary>
    /// MD5 hash of the sequence.
    /// </summary>
    public string? Md5 { get; init; }
}