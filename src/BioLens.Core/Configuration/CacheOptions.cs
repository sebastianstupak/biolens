namespace BioLens.Core.Configuration;

/// <summary>
/// Configuration options for caching.
/// </summary>
public class CacheOptions
{
    /// <summary>
    /// Configuration section name.
    /// </summary>
    public const string SectionName = "Cache";

    /// <summary>
    /// Cache provider type: InMemory, Redis, or Sqlite.
    /// </summary>
    public string Provider { get; set; } = "InMemory";

    /// <summary>
    /// Default expiration time in minutes.
    /// </summary>
    public int DefaultExpirationMinutes { get; set; } = 60;

    /// <summary>
    /// Connection string for external cache providers (Redis, etc).
    /// </summary>
    public string? ConnectionString { get; set; }

    /// <summary>
    /// Maximum cache size in MB for in-memory provider.
    /// </summary>
    public int MaxSizeMegabytes { get; set; } = 100;
}
