using System.Net.Http.Json;
using BioLens.Core.Models;
using Microsoft.Extensions.Logging;

namespace BioLens.Infrastructure.DataSources.UniProt;

/// <summary>
/// Client for interacting with the UniProt REST API.
/// </summary>
public class UniProtClient
{
    private readonly HttpClient _httpClient;
    private readonly ILogger<UniProtClient> _logger;

    public UniProtClient(HttpClient httpClient, ILogger<UniProtClient> logger)
    {
        _httpClient = httpClient ?? throw new ArgumentNullException(nameof(httpClient));
        _logger = logger ?? throw new ArgumentNullException(nameof(logger));
        
        if (_httpClient.BaseAddress == null)
        {
            throw new InvalidOperationException(
                "HttpClient.BaseAddress must be configured. " +
                "Configure it via AddUniProtClient or set it manually.");
        }
    }

    /// <summary>
    /// Retrieves a protein by its UniProt accession ID.
    /// </summary>
    public async Task<Protein?> GetProteinAsync(string id, CancellationToken cancellationToken = default)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(id);
        
        try
        {
            _logger.LogInformation("Fetching protein {ProteinId} from UniProt", id);

            var response = await _httpClient.GetAsync($"uniprotkb/{id}.json", cancellationToken);

            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
            {
                _logger.LogWarning("Protein {ProteinId} not found", id);
                return null;
            }

            response.EnsureSuccessStatusCode();

            var uniprotResponse = await response.Content.ReadFromJsonAsync<UniProtProteinResponse>(
                cancellationToken: cancellationToken);

            if (uniprotResponse == null)
            {
                _logger.LogError("Failed to deserialize UniProt response for {ProteinId}", id);
                return null;
            }

            var protein = UniProtMapper.MapToProtein(uniprotResponse);
            _logger.LogInformation("Successfully fetched protein {ProteinId}: {ProteinName}", id, protein.Name);

            return protein;
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error fetching protein {ProteinId}", id);
            throw;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error fetching protein {ProteinId}", id);
            throw;
        }
    }
}