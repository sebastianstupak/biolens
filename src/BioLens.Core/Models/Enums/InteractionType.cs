namespace BioLens.Core.Models.Enums;

/// <summary>
/// Types of protein-protein interactions.
/// </summary>
public enum InteractionType
{
    /// <summary>
    /// Direct physical binding between proteins.
    /// </summary>
    DirectBinding,

    /// <summary>
    /// Functional association without direct binding.
    /// </summary>
    FunctionalAssociation,

    /// <summary>
    /// Co-expression in same tissues or conditions.
    /// </summary>
    CoExpression,

    /// <summary>
    /// Proteins in same biochemical pathway.
    /// </summary>
    Pathway,

    /// <summary>
    /// Co-occurrence in same protein complex.
    /// </summary>
    Complex,

    /// <summary>
    /// Interaction type not specified.
    /// </summary>
    Unknown
}
