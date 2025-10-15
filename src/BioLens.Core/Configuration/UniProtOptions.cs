namespace BioLens.Core.Configuration;

/// <summary>
/// Configuration options for UniProt data source.
/// </summary>
public class UniProtOptions
{
    /// <summary>
    /// Configuration section name.
    /// </summary>
    public const string SectionName = "DataSources:UniProt";

    /// <summary>
    /// Base URL for the UniProt REST API.
    /// </summary>
    public Uri BaseUrl { get; set; } = new("https://rest.uniprot.org");

    /// <summary>
    /// Request timeout in seconds.
    /// </summary>
    public int TimeoutSeconds { get; set; } = 30;

    /// <summary>
    /// Maximum number of retry attempts for failed requests.
    /// </summary>
    public int MaxRetryAttempts { get; set; } = 3;

    /// <summary>
    /// Initial retry delay in milliseconds.
    /// </summary>
    public int RetryDelayMilliseconds { get; set; } = 1000;

    /// <summary>
    /// User agent string for API requests.
    /// </summary>
    public string UserAgent { get; set; } = "BioLens/1.0";
}
