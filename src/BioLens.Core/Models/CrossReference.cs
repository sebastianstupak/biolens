namespace BioLens.Core.Models;

/// <summary>
/// Represents a cross-reference to an external database with additional properties.
/// </summary>
public class CrossReference
{
    /// <summary>
    /// Name of the external database (e.g., "PDB", "RefSeq", "STRING").
    /// </summary>
    public required string Database { get; init; }
    
    /// <summary>
    /// Identifier in the external database.
    /// </summary>
    public required string Id { get; init; }
    
    /// <summary>
    /// Additional properties specific to this cross-reference.
    /// For example, PDB might include resolution, method, chains.
    /// </summary>
    public Dictionary<string, string> Properties { get; init; } = [];
    
    /// <summary>
    /// Returns a formatted string representation of the cross-reference.
    /// </summary>
    public override string ToString() => $"{Database}:{Id}";
}