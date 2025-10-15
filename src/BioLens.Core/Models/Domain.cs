using BioLens.Core.Models.Enums;

namespace BioLens.Core.Models;

/// <summary>
/// Represents a structural or functional domain within a protein sequence.
/// </summary>
public class Domain
{
    /// <summary>
    /// Type of the domain (e.g., DNA-binding, Kinase).
    /// </summary>
    public required DomainType Type { get; init; }

    /// <summary>
    /// Starting position of the domain in the protein sequence (1-indexed).
    /// </summary>
    public required int Start { get; init; }

    /// <summary>
    /// Ending position of the domain in the protein sequence (1-indexed, inclusive).
    /// </summary>
    public required int End { get; init; }

    /// <summary>
    /// Detailed description of the domain's function or characteristics.
    /// </summary>
    public string? Description { get; init; }

    /// <summary>
    /// Original name from source database if it doesn't map to a DomainType enum value.
    /// </summary>
    public string? OriginalName { get; init; }

    /// <summary>
    /// Length of the domain in amino acids.
    /// </summary>
    public int Length => End - Start + 1;
}
