using BioLens.Core.Configuration;
using BioLens.Infrastructure.DataSources.UniProt;
using BioLens.Infrastructure.Http;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;

namespace BioLens.Infrastructure.Extensions;

/// <summary>
/// Extension methods for configuring BioLens services.
/// </summary>
public static class ServiceCollectionExtensions
{
    /// <summary>
    /// Adds all BioLens data source clients.
    /// </summary>
    public static IServiceCollection AddBioLensClients(
        this IServiceCollection services,
        IConfiguration configuration)
    {
        services.AddUniProtClient(configuration);

        return services;
    }

    /// <summary>
    /// Adds UniProt client with configuration.
    /// </summary>
    public static IServiceCollection AddUniProtClient(
        this IServiceCollection services,
        IConfiguration configuration)
    {
        ArgumentNullException.ThrowIfNull(configuration);

        // Bind configuration - correct syntax
        var uniprotSection = configuration.GetSection(UniProtOptions.SectionName);
        services.Configure<UniProtOptions>(uniprotSection);

        // Add typed HttpClient with configuration
        services.AddHttpClient<UniProtClient>((serviceProvider, client) =>
        {
            var options = serviceProvider
                .GetRequiredService<IOptions<UniProtOptions>>()
                .Value;

            ConfigureHttpClient(client, options.BaseUrl, options.TimeoutSeconds, options.UserAgent);
        })
        .AddHttpMessageHandler(serviceProvider =>
        {
            var options = serviceProvider
                .GetRequiredService<IOptions<UniProtOptions>>()
                .Value;
            var logger = serviceProvider
                .GetRequiredService<ILogger<RetryHandler>>();

            return new RetryHandler(
                logger,
                options.MaxRetryAttempts,
                options.RetryDelayMilliseconds);
        });

        return services;
    }

    /// <summary>
    /// Configures common HttpClient settings.
    /// </summary>
    private static void ConfigureHttpClient(
        HttpClient client,
        Uri baseUrl,
        int timeoutSeconds,
        string userAgent)
    {
        client.BaseAddress = baseUrl;
        client.Timeout = TimeSpan.FromSeconds(timeoutSeconds);
        client.DefaultRequestHeaders.Add("User-Agent", userAgent);
        client.DefaultRequestHeaders.Add("Accept", "application/json");
    }
}
