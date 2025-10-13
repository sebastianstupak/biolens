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
        
        var normalized = value.Replace(" ", "").Replace("-", "").ToLowerInvariant();
        
        return normalized switch
        {
            _ when normalized.Contains("dnabinding") || normalized.Contains("dna-binding") => DomainType.DnaBinding,
            _ when normalized.Contains("kinase") => DomainType.Kinase,
            _ when normalized.Contains("transactivation") => DomainType.Transactivation,
            _ when normalized.Contains("tetramerization") => DomainType.Tetramerization,
            _ when normalized.Contains("dimerization") => DomainType.Dimerization,
            _ when normalized.Contains("transmembrane") => DomainType.Transmembrane,
            _ when normalized.Contains("signal") => DomainType.SignalPeptide,
            _ when normalized.Contains("zincfinger") || normalized.Contains("znf") => DomainType.ZincFinger,
            _ when normalized.Contains("immunoglobulin") || normalized.Contains("ig") => DomainType.Immunoglobulin,
            _ when normalized.Contains("sh2") => DomainType.Sh2,
            _ when normalized.Contains("sh3") => DomainType.Sh3,
            _ when normalized.Contains("catalytic") => DomainType.Catalytic,
            _ when normalized.Contains("regulatory") => DomainType.Regulatory,
            _ when normalized.Contains("motif") => DomainType.Motif,
            _ when normalized.Contains("repeat") => DomainType.Repeat,
            _ when normalized.Contains("coiledcoil") || normalized.Contains("coiled-coil") => DomainType.CoiledCoil,
            _ when normalized.Contains("region") => DomainType.Region,
            "domain" => DomainType.Domain,
            _ => DomainType.Other
        };
    }
}