namespace BioLens.Core.Models.Enums;

/// <summary>
/// Types of protein domains and functional regions.
/// </summary>
public enum DomainType
{
    /// <summary>
    /// DNA-binding domain that interacts with DNA sequences.
    /// </summary>
    DnaBinding,

    /// <summary>
    /// Protein kinase domain that phosphorylates other proteins.
    /// </summary>
    Kinase,

    /// <summary>
    /// Transcriptional activation domain.
    /// </summary>
    Transactivation,

    /// <summary>
    /// Domain responsible for protein oligomerization.
    /// </summary>
    Tetramerization,

    /// <summary>
    /// Domain responsible for protein dimerization.
    /// </summary>
    Dimerization,

    /// <summary>
    /// Transmembrane domain that spans cellular membranes.
    /// </summary>
    Transmembrane,

    /// <summary>
    /// Signal peptide for protein targeting and secretion.
    /// </summary>
    SignalPeptide,

    /// <summary>
    /// Zinc finger DNA-binding domain.
    /// </summary>
    ZincFinger,

    /// <summary>
    /// Immunoglobulin-like domain.
    /// </summary>
    Immunoglobulin,

    /// <summary>
    /// SH2 domain (Src Homology 2) for phosphotyrosine binding.
    /// </summary>
    Sh2,

    /// <summary>
    /// SH3 domain (Src Homology 3) for proline-rich sequence binding.
    /// </summary>
    Sh3,

    /// <summary>
    /// Catalytic domain containing enzymatic active site.
    /// </summary>
    Catalytic,

    /// <summary>
    /// Regulatory domain that modulates protein activity.
    /// </summary>
    Regulatory,

    /// <summary>
    /// Generic structural domain.
    /// </summary>
    Domain,

    /// <summary>
    /// Functional region that doesn't constitute a complete domain.
    /// </summary>
    Region,

    /// <summary>
    /// Short conserved sequence motif.
    /// </summary>
    Motif,

    /// <summary>
    /// Repeat sequence element.
    /// </summary>
    Repeat,

    /// <summary>
    /// Coiled coil structural element.
    /// </summary>
    CoiledCoil,

    /// <summary>
    /// Compositionally biased region (e.g., low complexity).
    /// </summary>
    CompositionallyBiased,

    /// <summary>
    /// Domain type not categorized or unknown.
    /// </summary>
    Other
}

public static class EnumDomainTypeExtension
{
    /// <summary>
    /// Parses a domain type string to DomainType enum with fuzzy matching.
    /// </summary>
    /// <param name="value">Domain type string from source database</param>
    /// <returns>Corresponding DomainType enum value</returns>
    public static DomainType ParseDomainType(this string value)
    {
        if (string.IsNullOrWhiteSpace(value))
            return DomainType.Other;

        var normalized = value
            .Replace(" ", "", StringComparison.Ordinal)
            .Replace("-", "", StringComparison.Ordinal)
            .ToUpperInvariant(); // Changed from ToLowerInvariant

        return normalized switch
        {
            _ when normalized.Contains("DNABINDING", StringComparison.Ordinal) ||
                   normalized.Contains("DNA-BINDING", StringComparison.Ordinal) => DomainType.DnaBinding,
            _ when normalized.Contains("KINASE", StringComparison.Ordinal) => DomainType.Kinase,
            _ when normalized.Contains("TRANSACTIVATION", StringComparison.Ordinal) => DomainType.Transactivation,
            _ when normalized.Contains("TETRAMERIZATION", StringComparison.Ordinal) => DomainType.Tetramerization,
            _ when normalized.Contains("DIMERIZATION", StringComparison.Ordinal) => DomainType.Dimerization,
            _ when normalized.Contains("TRANSMEMBRANE", StringComparison.Ordinal) => DomainType.Transmembrane,
            _ when normalized.Contains("SIGNAL", StringComparison.Ordinal) => DomainType.SignalPeptide,
            _ when normalized.Contains("ZINCFINGER", StringComparison.Ordinal) ||
                   normalized.Contains("ZNF", StringComparison.Ordinal) => DomainType.ZincFinger,
            _ when normalized.Contains("IMMUNOGLOBULIN", StringComparison.Ordinal) ||
                   normalized.Contains("IG", StringComparison.Ordinal) => DomainType.Immunoglobulin,
            _ when normalized.Contains("SH2", StringComparison.Ordinal) => DomainType.Sh2,
            _ when normalized.Contains("SH3", StringComparison.Ordinal) => DomainType.Sh3,
            _ when normalized.Contains("CATALYTIC", StringComparison.Ordinal) => DomainType.Catalytic,
            _ when normalized.Contains("REGULATORY", StringComparison.Ordinal) => DomainType.Regulatory,
            _ when normalized.Contains("MOTIF", StringComparison.Ordinal) => DomainType.Motif,
            _ when normalized.Contains("REPEAT", StringComparison.Ordinal) => DomainType.Repeat,
            _ when normalized.Contains("COILEDCOIL", StringComparison.Ordinal) ||
                   normalized.Contains("COILED-COIL", StringComparison.Ordinal) => DomainType.CoiledCoil,
            _ when normalized.Contains("REGION", StringComparison.Ordinal) => DomainType.Region,
            "DOMAIN" => DomainType.Domain,
            _ => DomainType.Other
        };
    }
}
