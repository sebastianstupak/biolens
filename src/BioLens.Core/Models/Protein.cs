using System.Collections.ObjectModel;
using BioLens.Core.Models.Enums;

namespace BioLens.Core.Models;

/// <summary>
/// Represents a protein with its primary attributes and associated data.
/// </summary>
public class Protein
{
    /// <summary>
    /// Primary protein identifier (e.g., UniProt accession number like P04637).
    /// </summary>
    public required string Id { get; init; }

    /// <summary>
    /// UniProtKB entry name (e.g., "P53_HUMAN").
    /// </summary>
    public string? EntryName { get; init; }

    /// <summary>
    /// Primary recommended name of the protein.
    /// </summary>
    public required string Name { get; init; }

    /// <summary>
    /// Full descriptive name of the protein, if different from Name.
    /// </summary>
    public string? FullName { get; init; }

    /// <summary>
    /// Alternative names or synonyms for the protein.
    /// </summary>
    public ReadOnlyCollection<string> AlternativeNames { get; init; } = new(Array.Empty<string>());

    /// <summary>
    /// Short names or abbreviations for the protein.
    /// </summary>
    public ReadOnlyCollection<string> ShortNames { get; init; } = new(Array.Empty<string>());

    /// <summary>
    /// The organism this protein belongs to.
    /// </summary>
    public required Organism Organism { get; init; }

    /// <summary>
    /// Detailed sequence information for the protein.
    /// </summary>
    public Sequence? Sequence { get; init; }

    /// <summary>
    /// Biological functions of the protein as described in the source database.
    /// </summary>
    public ReadOnlyCollection<string> Functions { get; init; } = new(Array.Empty<string>());

    /// <summary>
    /// Information about protein subunit composition and interactions.
    /// </summary>
    public ReadOnlyCollection<string> SubunitStructure { get; init; } = new(Array.Empty<string>());

    /// <summary>
    /// Structural and functional domains within the protein.
    /// </summary>
    public ReadOnlyCollection<Domain> Domains { get; init; } = new(Array.Empty<Domain>());

    /// <summary>
    /// Cross-references to other databases with detailed properties.
    /// </summary>
    public ReadOnlyCollection<CrossReference> CrossReferences { get; init; } = new(Array.Empty<CrossReference>());

    /// <summary>
    /// Subcellular locations where the protein is found.
    /// </summary>
    public ReadOnlyCollection<SubcellularLocation> SubcellularLocations { get; init; } = new(Array.Empty<SubcellularLocation>());

    /// <summary>
    /// Keywords or classifications associated with the protein.
    /// </summary>
    public ReadOnlyCollection<string> Keywords { get; init; } = new(Array.Empty<string>());

    /// <summary>
    /// Gene names associated with this protein.
    /// </summary>
    public ReadOnlyCollection<string> GeneNames { get; init; } = new(Array.Empty<string>());

    /// <summary>
    /// Date when the protein entry was last updated in the source database.
    /// </summary>
    public DateTime? LastUpdated { get; init; }

    /// <summary>
    /// Date when the sequence was last modified.
    /// </summary>
    public DateTime? SequenceLastUpdated { get; init; }

    /// <summary>
    /// Version number of the entry.
    /// </summary>
    public int? EntryVersion { get; init; }

    /// <summary>
    /// Version number of the sequence.
    /// </summary>
    public int? SequenceVersion { get; init; }

    /// <summary>
    /// Source database this protein data was retrieved from.
    /// </summary>
    public required DataSource Source { get; init; }

    /// <summary>
    /// Helper method to get a specific cross-reference by database name.
    /// </summary>
    /// <param name="database">Name of the database</param>
    /// <returns>The cross-reference if found, otherwise null</returns>
    public CrossReference? GetCrossReference(string database) =>
        CrossReferences.FirstOrDefault(x =>
            x.Database.Equals(database, StringComparison.OrdinalIgnoreCase));

    /// <summary>
    /// Helper method to check if protein has a specific keyword.
    /// </summary>
    /// <param name="keyword">Keyword to search for</param>
    /// <returns>True if keyword exists, case-insensitive</returns>
    public bool HasKeyword(string keyword) =>
        Keywords.Any(k => k.Equals(keyword, StringComparison.OrdinalIgnoreCase));

    /// <summary>
    /// Helper method to check if protein is located in a specific subcellular location.
    /// </summary>
    /// <param name="location">Subcellular location to check</param>
    /// <returns>True if protein is found in that location</returns>
    public bool IsLocatedIn(SubcellularLocation location) =>
        SubcellularLocations.Contains(location);
}
