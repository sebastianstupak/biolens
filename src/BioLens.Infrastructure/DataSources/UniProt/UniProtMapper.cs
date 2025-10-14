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
            AlternativeNames = ExtractAlternativeNames(response.ProteinDescription),
            ShortNames = ExtractShortNames(response.ProteinDescription),
            Organism = MapOrganism(response.Organism),
            Sequence = MapSequence(response.Sequence),
            Functions = ExtractFunctions(response.Comments),
            SubunitStructure = ExtractSubunitStructure(response.Comments),
            Domains = ExtractDomains(response.Features),
            CrossReferences = ExtractCrossReferences(response.UniProtKBCrossReferences),
            SubcellularLocations = ExtractSubcellularLocations(response.Comments),
            Keywords = ExtractKeywords(response.Keywords),
            GeneNames = ExtractGeneNames(response.Genes),
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
            return [];
        
        return description.AlternativeNames
            .Select(an => an.FullName?.Value)
            .Where(name => !string.IsNullOrWhiteSpace(name))
            .Cast<string>()
            .ToList();
    }
    
    private static List<string> ExtractShortNames(ProteinDescription? description)
    {
        if (description?.RecommendedName?.ShortNames == null)
            return [];
        
        return description.RecommendedName.ShortNames
            .Select(sn => sn.Value)
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
                Lineage = []
            };
        }
        
        return new Organism
        {
            ScientificName = organism.ScientificName,
            CommonName = organism.CommonName,
            TaxonId = organism.TaxonId,
            Lineage = organism.Lineage ?? []
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
    
    private static List<string> ExtractFunctions(List<Comment>? comments)
    {
        if (comments == null)
            return [];
        
        return comments
            .Where(c => c.CommentType == "FUNCTION")
            .SelectMany(c => c.Texts ?? [])
            .Select(t => t.Value)
            .Where(v => !string.IsNullOrWhiteSpace(v))
            .ToList();
    }
    
    private static List<string> ExtractSubunitStructure(List<Comment>? comments)
    {
        if (comments == null)
            return [];
        
        return comments
            .Where(c => c.CommentType == "SUBUNIT")
            .SelectMany(c => c.Texts ?? [])
            .Select(t => t.Value)
            .Where(v => !string.IsNullOrWhiteSpace(v))
            .ToList();
    }
    
    private static List<Domain> ExtractDomains(List<Feature>? features)
    {
        if (features == null)
            return [];
        
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
        List<UniProtCrossReference>? crossRefs)
    {
        if (crossRefs == null)
            return [];
        
        return crossRefs
            .GroupBy(x => x.Database)
            .Select(g => g.First())
            .Select(xref => new CrossReference
            {
                Database = xref.Database,
                Id = xref.Id,
                Properties = xref.Properties?
                    .ToDictionary(p => p.Key, p => p.Value) ?? []
            })
            .ToList();
    }
    
    private static List<Core.Models.Enums.SubcellularLocation> ExtractSubcellularLocations(
        List<Comment>? comments)
    {
        if (comments == null)
            return [];
        
        return comments
            .Where(c => c.CommentType == "SUBCELLULAR LOCATION")
            .SelectMany(c => c.SubcellularLocations ?? [])
            .Select(sl => sl.Location?.Value)
            .Where(v => !string.IsNullOrWhiteSpace(v))
            .Select(v => v!.ParseSubcellularLocation())
            .Distinct()
            .ToList();
    }
    
    private static List<string> ExtractKeywords(List<Keyword>? keywords)
    {
        if (keywords == null)
            return [];
        
        return keywords
            .Select(k => k.Name)
            .Where(n => !string.IsNullOrWhiteSpace(n))
            .ToList();
    }
    
    private static List<string> ExtractGeneNames(List<Gene>? genes)
    {
        if (genes == null)
            return [];
        
        var geneNames = new List<string>();
        
        foreach (var gene in genes)
        {
            if (gene.GeneName?.Value != null)
                geneNames.Add(gene.GeneName.Value);
            
            if (gene.Synonyms != null)
                geneNames.AddRange(gene.Synonyms.Select(s => s.Value));
        }
        
        return geneNames.Distinct().ToList();
    }
}