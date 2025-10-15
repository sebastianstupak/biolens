using System.Collections.ObjectModel;
using BioLens.Core.Models;
using BioLens.Core.Models.Enums;

namespace BioLens.Infrastructure.DataSources.UniProt;

/// <summary>
/// Maps UniProt API response models to BioLens core domain models.
/// </summary>
public static class UniProtMapper
{
    /// <summary>
    /// Maps a UniProt protein response to a BioLens Protein model.
    /// </summary>
    public static Protein MapToProtein(UniProtProteinResponse response)
    {
        ArgumentNullException.ThrowIfNull(response);

        return new Protein
        {
            Id = response.PrimaryAccession,
            EntryName = response.UniProtkbId,
            Name = ExtractProteinName(response),
            FullName = response.ProteinDescription?.RecommendedName?.FullName?.Value,
            AlternativeNames = new ReadOnlyCollection<string>(ExtractAlternativeNames(response.ProteinDescription)),
            ShortNames = new ReadOnlyCollection<string>(ExtractShortNames(response.ProteinDescription)),
            Organism = MapOrganism(response.Organism),
            Sequence = MapSequence(response.Sequence),
            Functions = new ReadOnlyCollection<string>(ExtractFunctions(response.Comments)),
            SubunitStructure = new ReadOnlyCollection<string>(ExtractSubunitStructure(response.Comments)),
            Domains = new ReadOnlyCollection<Domain>(ExtractDomains(response.Features)),
            CrossReferences = new ReadOnlyCollection<CrossReference>(ExtractCrossReferences(response.UniProtKBCrossReferences)),
            SubcellularLocations = new ReadOnlyCollection<Core.Models.Enums.SubcellularLocation>(ExtractSubcellularLocations(response.Comments)),
            Keywords = new ReadOnlyCollection<string>(ExtractKeywords(response.Keywords)),
            GeneNames = new ReadOnlyCollection<string>(ExtractGeneNames(response.Genes)),
            LastUpdated = response.EntryAudit?.LastAnnotationUpdateDate,
            SequenceLastUpdated = response.EntryAudit?.LastSequenceUpdateDate,
            EntryVersion = response.EntryAudit?.EntryVersion,
            SequenceVersion = response.EntryAudit?.SequenceVersion,
            Source = DataSource.UniProt
        };
    }

    private static string ExtractProteinName(UniProtProteinResponse response)
    {
        return response.ProteinDescription?.RecommendedName?.FullName?.Value
            ?? response.UniProtkbId
            ?? response.PrimaryAccession;
    }

    private static List<string> ExtractAlternativeNames(ProteinDescription? description)
    {
        if (description?.AlternativeNames == null)
            return new List<string>();

        return description.AlternativeNames
            .Select(altName => altName.FullName?.Value)
            .Where(name => !string.IsNullOrWhiteSpace(name))
            .Cast<string>()
            .ToList();
    }

    private static List<string> ExtractShortNames(ProteinDescription? description)
    {
        if (description?.RecommendedName?.ShortNames == null)
            return new List<string>();

        return description.RecommendedName.ShortNames
            .Select(shortName => shortName.Value)
            .Where(name => !string.IsNullOrWhiteSpace(name))
            .ToList();
    }

    private static Organism MapOrganism(UniProtOrganism? organism)
    {
        if (organism == null)
        {
            return new Organism
            {
                ScientificName = "Unknown",
                CommonName = null,
                TaxonId = null,
                Lineage = new ReadOnlyCollection<string>(Array.Empty<string>())
            };
        }

        List<string> lineage;
        if (organism.Lineage != null)
            lineage = new List<string>(organism.Lineage);
        else
            lineage = new List<string>();

        return new Organism
        {
            ScientificName = organism.ScientificName,
            CommonName = organism.CommonName,
            TaxonId = organism.TaxonId,
            Lineage = new ReadOnlyCollection<string>(lineage)
        };
    }

    private static Sequence? MapSequence(SequenceInfo? sequenceInfo)
    {
        if (sequenceInfo == null)
            return null;

        return new Sequence
        {
            Value = sequenceInfo.Value,
            Length = sequenceInfo.Length,
            Type = SequenceType.Protein,
            MolecularWeight = sequenceInfo.MolWeight,
            Crc64 = sequenceInfo.Crc64,
            Md5 = sequenceInfo.Md5
        };
    }

    private static List<string> ExtractFunctions(IEnumerable<Comment>? comments)
    {
        if (comments == null)
            return new List<string>();

#pragma warning disable S3267
        var functions = new List<string>();
        foreach (var comment in comments.Where(c => c.CommentType == "FUNCTION" && c.Texts != null))
        {
            if (comment.Texts != null)
            {
                foreach (var text in comment.Texts)
                {
                    if (text != null && !string.IsNullOrWhiteSpace(text.Value))
                        functions.Add(text.Value);
                }
            }
        }
        return functions;
#pragma warning restore S3267
    }

    private static List<string> ExtractSubunitStructure(IEnumerable<Comment>? comments)
    {
        if (comments == null)
            return new List<string>();

#pragma warning disable S3267
        var subunits = new List<string>();
        foreach (var comment in comments.Where(c => c.CommentType == "SUBUNIT" && c.Texts != null))
        {
            if (comment.Texts != null)
            {
                foreach (var text in comment.Texts)
                {
                    if (text != null && !string.IsNullOrWhiteSpace(text.Value))
                        subunits.Add(text.Value);
                }
            }
        }
        return subunits;
#pragma warning restore S3267
    }

    private static List<Domain> ExtractDomains(IEnumerable<Feature>? features)
    {
        if (features == null)
            return new List<Domain>();

        return features
            .Where(f => IsDomainFeature(f.Type) &&
                       f.Location?.Start?.Value != null &&
                       f.Location?.End?.Value != null)
            .Select(f => new Domain
            {
                Type = (f.Description ?? f.Type).ParseDomainType(),
                Start = f.Location!.Start!.Value!.Value,
                End = f.Location!.End!.Value!.Value,
                Description = f.Description,
                OriginalName = f.Description ?? f.Type
            })
            .ToList();
    }

    private static bool IsDomainFeature(string type)
    {
        var domainTypes = new[]
        {
            "Domain",
            "Region",
            "Motif",
            "Repeat",
            "Zinc finger",
            "DNA-binding region",
            "Coiled coil",
            "Compositional bias",
            "Transmembrane",
            "Signal peptide"
        };

        return domainTypes.Any(dt =>
            type.Contains(dt, StringComparison.OrdinalIgnoreCase));
    }

    private static List<CrossReference> ExtractCrossReferences(
        IEnumerable<UniProtCrossReference>? crossRefs)
    {
        if (crossRefs == null)
            return new List<CrossReference>();

        return crossRefs
            .GroupBy(x => x.Database)
            .Select(g => g.First())
            .Select(xref => new CrossReference
            {
                Database = xref.Database,
                Id = xref.Id,
                Properties = xref.Properties?.ToDictionary(p => p.Key, p => p.Value)
                            ?? new Dictionary<string, string>()
            })
            .ToList();
    }

    private static List<Core.Models.Enums.SubcellularLocation> ExtractSubcellularLocations(
        IEnumerable<Comment>? comments)
    {
        if (comments == null)
            return new List<Core.Models.Enums.SubcellularLocation>();

#pragma warning disable S3267
        var uniqueLocations = new HashSet<Core.Models.Enums.SubcellularLocation>();
        foreach (var comment in comments.Where(c => c.CommentType == "SUBCELLULAR LOCATION" && c.SubcellularLocations != null))
        {
            if (comment.SubcellularLocations != null)
            {
                foreach (var location in comment.SubcellularLocations)
                {
                    if (location != null && location.Location?.Value != null && !string.IsNullOrWhiteSpace(location.Location.Value))
                    {
                        uniqueLocations.Add(location.Location.Value.ParseSubcellularLocation());
                    }
                }
            }
        }
        return uniqueLocations.ToList();
#pragma warning restore S3267
    }

    private static List<string> ExtractKeywords(IEnumerable<Keyword>? keywords)
    {
        if (keywords == null)
            return new List<string>();

        return keywords
            .Select(keyword => keyword.Name)
            .Where(name => !string.IsNullOrWhiteSpace(name))
            .ToList();
    }

    private static List<string> ExtractGeneNames(IEnumerable<Gene>? genes)
    {
        if (genes == null)
            return new List<string>();

        var geneNames = new List<string>();

        foreach (var gene in genes)
        {
            if (gene.GeneName?.Value != null)
                geneNames.Add(gene.GeneName.Value);

            if (gene.Synonyms != null)
            {
                geneNames.AddRange(gene.Synonyms
                    .Where(synonym => synonym != null && !string.IsNullOrWhiteSpace(synonym.Value))
                    .Select(synonym => synonym.Value));
            }
        }

        return geneNames.Distinct().ToList();
    }
}
