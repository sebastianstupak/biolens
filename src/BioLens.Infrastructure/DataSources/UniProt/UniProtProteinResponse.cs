using System.Text.Json.Serialization;

namespace BioLens.Infrastructure.DataSources.UniProt;

/// <summary>
/// Root response model for UniProt KB entry.
/// </summary>
public record UniProtProteinResponse
{
    [JsonPropertyName("entryType")]
    public string? EntryType { get; init; }

    [JsonPropertyName("primaryAccession")]
    public required string PrimaryAccession { get; init; }

    [JsonPropertyName("secondaryAccessions")]
    public IReadOnlyCollection<string>? SecondaryAccessions { get; init; }

    [JsonPropertyName("uniProtkbId")]
    public string? UniProtkbId { get; init; }

    [JsonPropertyName("entryAudit")]
    public EntryAudit? EntryAudit { get; init; }

    [JsonPropertyName("annotationScore")]
    public double? AnnotationScore { get; init; }

    [JsonPropertyName("organism")]
    public UniProtOrganism? Organism { get; init; }

    [JsonPropertyName("proteinExistence")]
    public string? ProteinExistence { get; init; }

    [JsonPropertyName("proteinDescription")]
    public ProteinDescription? ProteinDescription { get; init; }

    [JsonPropertyName("genes")]
    public IReadOnlyCollection<Gene>? Genes { get; init; }

    [JsonPropertyName("comments")]
    public IReadOnlyCollection<Comment>? Comments { get; init; }

    [JsonPropertyName("features")]
    public IReadOnlyCollection<Feature>? Features { get; init; }

    [JsonPropertyName("keywords")]
    public IReadOnlyCollection<Keyword>? Keywords { get; init; }

    [JsonPropertyName("references")]
    public IReadOnlyCollection<Reference>? References { get; init; }

    [JsonPropertyName("uniProtKBCrossReferences")]
    public IReadOnlyCollection<UniProtCrossReference>? UniProtKBCrossReferences { get; init; }

    [JsonPropertyName("sequence")]
    public SequenceInfo? Sequence { get; init; }
}

public record EntryAudit
{
    [JsonPropertyName("firstPublicDate")]
    public DateTime? FirstPublicDate { get; init; }

    [JsonPropertyName("lastAnnotationUpdateDate")]
    public DateTime? LastAnnotationUpdateDate { get; init; }

    [JsonPropertyName("lastSequenceUpdateDate")]
    public DateTime? LastSequenceUpdateDate { get; init; }

    [JsonPropertyName("entryVersion")]
    public int? EntryVersion { get; init; }

    [JsonPropertyName("sequenceVersion")]
    public int? SequenceVersion { get; init; }
}

public record UniProtOrganism
{
    [JsonPropertyName("scientificName")]
    public required string ScientificName { get; init; }

    [JsonPropertyName("commonName")]
    public string? CommonName { get; init; }

    [JsonPropertyName("taxonId")]
    public int TaxonId { get; init; }

    [JsonPropertyName("lineage")]
    public IReadOnlyCollection<string>? Lineage { get; init; }
}

public record ProteinDescription
{
    [JsonPropertyName("recommendedName")]
    public RecommendedName? RecommendedName { get; init; }

    [JsonPropertyName("alternativeNames")]
    public IReadOnlyCollection<AlternativeName>? AlternativeNames { get; init; }

    [JsonPropertyName("submissionNames")]
    public IReadOnlyCollection<SubmissionName>? SubmissionNames { get; init; }
}

public record RecommendedName
{
    [JsonPropertyName("fullName")]
    public NameValue? FullName { get; init; }

    [JsonPropertyName("shortNames")]
    public IReadOnlyCollection<NameValue>? ShortNames { get; init; }

    [JsonPropertyName("ecNumbers")]
    public IReadOnlyCollection<EcNumber>? EcNumbers { get; init; }
}

public record AlternativeName
{
    [JsonPropertyName("fullName")]
    public NameValue? FullName { get; init; }

    [JsonPropertyName("shortNames")]
    public IReadOnlyCollection<NameValue>? ShortNames { get; init; }
}

public record SubmissionName
{
    [JsonPropertyName("fullName")]
    public NameValue? FullName { get; init; }
}

public record NameValue
{
    [JsonPropertyName("value")]
    public required string Value { get; init; }
}

public record EcNumber
{
    [JsonPropertyName("value")]
    public required string Value { get; init; }
}

public record Gene
{
    [JsonPropertyName("geneName")]
    public GeneName? GeneName { get; init; }

    [JsonPropertyName("synonyms")]
    public IReadOnlyCollection<GeneName>? Synonyms { get; init; }

    [JsonPropertyName("orderedLocusNames")]
    public IReadOnlyCollection<OrderedLocusName>? OrderedLocusNames { get; init; }

    [JsonPropertyName("orfNames")]
    public IReadOnlyCollection<OrfName>? OrfNames { get; init; }
}

public record GeneName
{
    [JsonPropertyName("value")]
    public required string Value { get; init; }
}

public record OrderedLocusName
{
    [JsonPropertyName("value")]
    public required string Value { get; init; }
}

public record OrfName
{
    [JsonPropertyName("value")]
    public required string Value { get; init; }
}

public record Comment
{
    [JsonPropertyName("commentType")]
    public required string CommentType { get; init; }

    [JsonPropertyName("texts")]
    public IReadOnlyCollection<CommentText>? Texts { get; init; }

    [JsonPropertyName("subcellularLocations")]
    public IReadOnlyCollection<SubcellularLocation>? SubcellularLocations { get; init; }

    [JsonPropertyName("interactions")]
    public IReadOnlyCollection<Interaction>? Interactions { get; init; }
}

public record CommentText
{
    [JsonPropertyName("value")]
    public required string Value { get; init; }
}

public record SubcellularLocation
{
    [JsonPropertyName("location")]
    public LocationValue? Location { get; init; }

    [JsonPropertyName("topology")]
    public LocationValue? Topology { get; init; }
}

public record LocationValue
{
    [JsonPropertyName("value")]
    public required string Value { get; init; }
}

public record Interaction
{
    [JsonPropertyName("interactantOne")]
    public Interactant? InteractantOne { get; init; }

    [JsonPropertyName("interactantTwo")]
    public Interactant? InteractantTwo { get; init; }

    [JsonPropertyName("numberOfExperiments")]
    public int? NumberOfExperiments { get; init; }

    [JsonPropertyName("organismDiffer")]
    public bool? OrganismDiffer { get; init; }
}

public record Interactant
{
    [JsonPropertyName("uniProtkbAccession")]
    public string? UniProtkbAccession { get; init; }

    [JsonPropertyName("intActId")]
    public string? IntActId { get; init; }

    [JsonPropertyName("geneName")]
    public string? GeneName { get; init; }
}

public record Feature
{
    [JsonPropertyName("type")]
    public required string Type { get; init; }

    [JsonPropertyName("location")]
    public FeatureLocation? Location { get; init; }

    [JsonPropertyName("description")]
    public string? Description { get; init; }

    [JsonPropertyName("featureId")]
    public string? FeatureId { get; init; }

    [JsonPropertyName("evidences")]
    public IReadOnlyCollection<Evidence>? Evidences { get; init; }
}

public record FeatureLocation
{
    [JsonPropertyName("start")]
    public Position? Start { get; init; }

    [JsonPropertyName("end")]
    public Position? End { get; init; }

    [JsonPropertyName("sequence")]
    public string? Sequence { get; init; }
}

public record Position
{
    [JsonPropertyName("value")]
    public int? Value { get; init; }

    [JsonPropertyName("modifier")]
    public string? Modifier { get; init; }
}

public record Evidence
{
    [JsonPropertyName("evidenceCode")]
    public string? EvidenceCode { get; init; }

    [JsonPropertyName("source")]
    public string? Source { get; init; }

    [JsonPropertyName("id")]
    public string? Id { get; init; }
}

public record Keyword
{
    [JsonPropertyName("id")]
    public required string Id { get; init; }

    [JsonPropertyName("category")]
    public string? Category { get; init; }

    [JsonPropertyName("name")]
    public required string Name { get; init; }
}

public record Reference
{
    [JsonPropertyName("citation")]
    public Citation? Citation { get; init; }

    [JsonPropertyName("referencePositions")]
    public IReadOnlyCollection<string>? ReferencePositions { get; init; }
}

public record Citation
{
    [JsonPropertyName("id")]
    public string? Id { get; init; }

    [JsonPropertyName("citationType")]
    public string? CitationType { get; init; }

    [JsonPropertyName("authors")]
    public IReadOnlyCollection<string>? Authors { get; init; }

    [JsonPropertyName("citationCrossReferences")]
    public IReadOnlyCollection<CitationCrossReference>? CitationCrossReferences { get; init; }

    [JsonPropertyName("title")]
    public string? Title { get; init; }

    [JsonPropertyName("publicationDate")]
    public string? PublicationDate { get; init; }

    [JsonPropertyName("journal")]
    public string? Journal { get; init; }

    [JsonPropertyName("firstPage")]
    public string? FirstPage { get; init; }

    [JsonPropertyName("lastPage")]
    public string? LastPage { get; init; }

    [JsonPropertyName("volume")]
    public string? Volume { get; init; }
}

public record CitationCrossReference
{
    [JsonPropertyName("database")]
    public required string Database { get; init; }

    [JsonPropertyName("id")]
    public required string Id { get; init; }
}

public record UniProtCrossReference
{
    [JsonPropertyName("database")]
    public required string Database { get; init; }

    [JsonPropertyName("id")]
    public required string Id { get; init; }

    [JsonPropertyName("properties")]
    public IReadOnlyCollection<CrossReferenceProperty>? Properties { get; init; }

    [JsonPropertyName("isoformId")]
    public string? IsoformId { get; init; }
}

public record CrossReferenceProperty
{
    [JsonPropertyName("key")]
    public required string Key { get; init; }

    [JsonPropertyName("value")]
    public required string Value { get; init; }
}

public record SequenceInfo
{
    [JsonPropertyName("value")]
    public required string Value { get; init; }

    [JsonPropertyName("length")]
    public int Length { get; init; }

    [JsonPropertyName("molWeight")]
    public int? MolWeight { get; init; }

    [JsonPropertyName("crc64")]
    public string? Crc64 { get; init; }

    [JsonPropertyName("md5")]
    public string? Md5 { get; init; }
}
