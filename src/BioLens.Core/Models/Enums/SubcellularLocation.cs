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
        
        var normalized = value.Replace(" ", "").ToLowerInvariant();
        
        return normalized switch
        {
            _ when normalized.Contains("nucleus") => SubcellularLocation.Nucleus,
            _ when normalized.Contains("cytoplasm") => SubcellularLocation.Cytoplasm,
            _ when normalized.Contains("membrane") => SubcellularLocation.Membrane,
            _ when normalized.Contains("mitochondri") => SubcellularLocation.Mitochondrion,
            _ when normalized.Contains("endoplasmic") || normalized.Contains("er") => SubcellularLocation.EndoplasmicReticulum,
            _ when normalized.Contains("golgi") => SubcellularLocation.Golgi,
            _ when normalized.Contains("lysosome") => SubcellularLocation.Lysosome,
            _ when normalized.Contains("peroxisome") => SubcellularLocation.Peroxisome,
            _ when normalized.Contains("secret") => SubcellularLocation.Secreted,
            _ when normalized.Contains("extracellular") => SubcellularLocation.ExtracellularMatrix,
            _ when normalized.Contains("cytoskeleton") => SubcellularLocation.Cytoskeleton,
            _ => SubcellularLocation.Unknown
        };
    }
}