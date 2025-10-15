namespace BioLens.Core.Models.Enums;

/// <summary>
/// Subcellular locations where proteins can be found.
/// </summary>
public enum SubcellularLocation
{
    /// <summary>
    /// Cell nucleus - contains genetic material.
    /// </summary>
    Nucleus,

    /// <summary>
    /// Cytoplasm - fluid inside the cell.
    /// </summary>
    Cytoplasm,

    /// <summary>
    /// Cell membrane - outer boundary of the cell.
    /// </summary>
    Membrane,

    /// <summary>
    /// Mitochondrion - cellular energy production.
    /// </summary>
    Mitochondrion,

    /// <summary>
    /// Endoplasmic reticulum - protein synthesis and processing.
    /// </summary>
    EndoplasmicReticulum,

    /// <summary>
    /// Golgi apparatus - protein modification and sorting.
    /// </summary>
    Golgi,

    /// <summary>
    /// Lysosome - cellular digestion.
    /// </summary>
    Lysosome,

    /// <summary>
    /// Peroxisome - fatty acid metabolism.
    /// </summary>
    Peroxisome,

    /// <summary>
    /// Secreted outside the cell.
    /// </summary>
    Secreted,

    /// <summary>
    /// Extracellular matrix - outside cells in tissues.
    /// </summary>
    ExtracellularMatrix,

    /// <summary>
    /// Cell projection (e.g., flagellum, cilium).
    /// </summary>
    CellProjection,

    /// <summary>
    /// Cytoskeleton - structural framework of the cell.
    /// </summary>
    Cytoskeleton,

    /// <summary>
    /// Multiple or various locations.
    /// </summary>
    Multiple,

    /// <summary>
    /// Location not specified or unknown.
    /// </summary>
    Unknown
}

public static class EnumSubcelullarLocationExtension
{
    /// <summary>
    /// Parses a subcellular location string to SubcellularLocation enum.
    /// </summary>
    /// <param name="value">Location string from source database</param>
    /// <returns>Corresponding SubcellularLocation enum value</returns>
    public static SubcellularLocation ParseSubcellularLocation(this string value)
    {
        if (string.IsNullOrWhiteSpace(value))
            return SubcellularLocation.Unknown;

        var normalized = value
            .Replace(" ", "", StringComparison.Ordinal)
            .ToUpperInvariant();

        return normalized switch
        {
            _ when normalized.Contains("NUCLEUS", StringComparison.Ordinal) => SubcellularLocation.Nucleus,
            _ when normalized.Contains("CYTOPLASM", StringComparison.Ordinal) => SubcellularLocation.Cytoplasm,
            _ when normalized.Contains("MEMBRANE", StringComparison.Ordinal) => SubcellularLocation.Membrane,
            _ when normalized.Contains("MITOCHONDRI", StringComparison.Ordinal) => SubcellularLocation.Mitochondrion,
            _ when normalized.Contains("ENDOPLASMIC", StringComparison.Ordinal) ||
                   normalized.Contains("ER", StringComparison.Ordinal) => SubcellularLocation.EndoplasmicReticulum,
            _ when normalized.Contains("GOLGI", StringComparison.Ordinal) => SubcellularLocation.Golgi,
            _ when normalized.Contains("LYSOSOME", StringComparison.Ordinal) => SubcellularLocation.Lysosome,
            _ when normalized.Contains("PEROXISOME", StringComparison.Ordinal) => SubcellularLocation.Peroxisome,
            _ when normalized.Contains("SECRET", StringComparison.Ordinal) => SubcellularLocation.Secreted,
            _ when normalized.Contains("EXTRACELLULAR", StringComparison.Ordinal) => SubcellularLocation.ExtracellularMatrix,
            _ when normalized.Contains("CYTOSKELETON", StringComparison.Ordinal) => SubcellularLocation.Cytoskeleton,
            _ => SubcellularLocation.Unknown
        };
    }
}
