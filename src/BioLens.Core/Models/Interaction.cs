using BioLens.Core.Models.Enums;

namespace BioLens.Core.Models;

/// <summary>
/// Represents a protein-protein interaction between two proteins.
/// </summary>
public class Interaction
{
    /// <summary>
    /// Identifier of the first protein in the interaction.
    /// </summary>
    public required string ProteinA { get; init; }
    
    /// <summary>
    /// Identifier of the second protein in the interaction.
    /// </summary>
    public required string ProteinB { get; init; }
    
    /// <summary>
    /// Confidence score for the interaction, typically between 0 and 1.
    /// Higher values indicate more confident interactions.
    /// </summary>
    public double Score { get; init; }
    
    /// <summary>
    /// Type of interaction between the proteins.
    /// </summary>
    public InteractionType Type { get; init; }
    
    /// <summary>
    /// Types of evidence supporting this interaction.
    /// </summary>
    public List<EvidenceType> Evidence { get; init; } = [];
    
    /// <summary>
    /// Database or source where this interaction was reported.
    /// </summary>
    public DataSource? Source { get; init; }
    
    /// <summary>
    /// Publications (PubMed IDs) that report this interaction.
    /// </summary>
    public List<int> PubMedIds { get; init; } = [];
    
    /// <summary>
    /// Helper method to check if interaction has experimental evidence.
    /// </summary>
    /// <returns>True if at least one experimental evidence type exists</returns>
    public bool HasExperimentalEvidence() =>
        Evidence.Contains(EvidenceType.Experimental) || 
        Evidence.Contains(EvidenceType.HighThroughput) ||
        Evidence.Contains(EvidenceType.Biochemical) ||
        Evidence.Contains(EvidenceType.Genetic);
}