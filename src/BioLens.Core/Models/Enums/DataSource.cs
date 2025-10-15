namespace BioLens.Core.Models.Enums;

/// <summary>
/// Represents the source database from which biological data was retrieved.
/// </summary>
public enum DataSource
{
    /// <summary>
    /// Universal Protein Resource - comprehensive protein sequence and annotation database.
    /// </summary>
    UniProt,

    /// <summary>
    /// STRING - protein-protein interaction networks database.
    /// </summary>
    StringDb,

    /// <summary>
    /// Protein Data Bank - 3D structural data of biological macromolecules.
    /// </summary>
    Pdb,

    /// <summary>
    /// NCBI Reference Sequence database - comprehensive, integrated, non-redundant sequence database.
    /// </summary>
    RefSeq,

    /// <summary>
    /// Ensembl - genome annotation database.
    /// </summary>
    Ensembl,

    /// <summary>
    /// Gene Ontology - structured vocabulary for gene and protein functions.
    /// </summary>
    GeneOntology,

    /// <summary>
    /// AlphaFold Database - AI-predicted protein structures.
    /// </summary>
    AlphaFoldDb,

    /// <summary>
    /// Multiple sources aggregated together.
    /// </summary>
    Aggregated,

    /// <summary>
    /// Unknown or unspecified data source.
    /// </summary>
    Unknown
}
