namespace BioLens.Core.Models.Enums;

/// <summary>
/// Types of experimental or computational evidence.
/// </summary>
public enum EvidenceType
{
    /// <summary>
    /// Evidence from laboratory experiments.
    /// </summary>
    Experimental,

    /// <summary>
    /// Evidence from high-throughput experimental methods.
    /// </summary>
    HighThroughput,

    /// <summary>
    /// Evidence from computational prediction or inference.
    /// </summary>
    Predicted,

    /// <summary>
    /// Evidence from text mining of scientific literature.
    /// </summary>
    TextMining,

    /// <summary>
    /// Evidence from database cross-references.
    /// </summary>
    Database,

    /// <summary>
    /// Evidence from genomic context analysis.
    /// </summary>
    GenomicContext,

    /// <summary>
    /// Evidence from gene co-expression analysis.
    /// </summary>
    CoExpression,

    /// <summary>
    /// Evidence from evolutionary analysis.
    /// </summary>
    Phylogenetic,

    /// <summary>
    /// Evidence from biochemical assays.
    /// </summary>
    Biochemical,

    /// <summary>
    /// Evidence from genetic interaction studies.
    /// </summary>
    Genetic,

    /// <summary>
    /// Evidence type not specified.
    /// </summary>
    Unknown
}
