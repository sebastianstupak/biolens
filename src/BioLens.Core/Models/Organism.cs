namespace BioLens.Core.Models;

/// <summary>
/// Represents an organism (species) with taxonomic information.
/// </summary>
public class Organism
{
    /// <summary>
    /// Scientific name of the organism (e.g., "Homo sapiens").
    /// </summary>
    public required string ScientificName { get; init; }
    
    /// <summary>
    /// Common name of the organism (e.g., "Human").
    /// </summary>
    public string? CommonName { get; init; }
    
    /// <summary>
    /// NCBI Taxonomy database identifier.
    /// </summary>
    public int? TaxonId { get; init; }
    
    /// <summary>
    /// Taxonomic lineage from domain to species.
    /// </summary>
    public List<string> Lineage { get; init; } = [];
    
    /// <summary>
    /// Returns the display name, preferring common name if available.
    /// </summary>
    public string DisplayName => CommonName ?? ScientificName;
}