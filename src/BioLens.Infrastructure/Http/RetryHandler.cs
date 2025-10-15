using System.Net;
using Microsoft.Extensions.Logging;

namespace BioLens.Infrastructure.Http;

/// <summary>
/// HTTP message handler that implements retry logic.
/// </summary>
public class RetryHandler : DelegatingHandler
{
    private readonly ILogger<RetryHandler> _logger;
    private readonly int _maxRetryAttempts;
    private readonly int _retryDelayMilliseconds;

    public RetryHandler(
        ILogger<RetryHandler> logger,
        int maxRetryAttempts = 3,
        int retryDelayMilliseconds = 1000)
    {
        _logger = logger ?? throw new ArgumentNullException(nameof(logger));
        _maxRetryAttempts = maxRetryAttempts;
        _retryDelayMilliseconds = retryDelayMilliseconds;
    }

    protected override async Task<HttpResponseMessage> SendAsync(
        HttpRequestMessage request,
        CancellationToken cancellationToken)
    {
        ArgumentNullException.ThrowIfNull(request);
        ArgumentNullException.ThrowIfNull(request.RequestUri);

        HttpResponseMessage? response = null;

        for (int i = 0; i <= _maxRetryAttempts; ++i)
        {
            try
            {
                response = await base.SendAsync(request, cancellationToken).ConfigureAwait(false);

                if (response.IsSuccessStatusCode || !IsTransientError(response.StatusCode))
                    return response;

                if (i < _maxRetryAttempts)
                {
                    var delay = TimeSpan.FromMilliseconds(_retryDelayMilliseconds * Math.Pow(2, i));

                    _logger.LogWarning(
                        "Request to {Uri} failed with status {StatusCode}. Retrying in {Delay}ms. Attempt {Attempt}/{Max}",
                        request.RequestUri,
                        response.StatusCode,
                        delay.TotalMilliseconds,
                        i + 1,
                        _maxRetryAttempts);

                    await Task.Delay(delay, cancellationToken).ConfigureAwait(false);
                }
            }
            catch (HttpRequestException ex) when (i < _maxRetryAttempts)
            {
                var delay = TimeSpan.FromMilliseconds(_retryDelayMilliseconds * Math.Pow(2, i));
                _logger.LogWarning(
                    ex,
                    "Request to {Uri} failed. Retrying in {Delay}ms. Attempt {Attempt}/{Max}",
                    request.RequestUri,
                    delay.TotalMilliseconds,
                    i + 1,
                    _maxRetryAttempts);

                await Task.Delay(delay, cancellationToken).ConfigureAwait(false);
            }
        }

        // If we've exhausted all retries and have a response, return it
        // Otherwise throw a detailed exception
        return response ??
               throw new HttpRequestException(
                   $"All {_maxRetryAttempts} retry attempts failed for {request.RequestUri}");
    }

    private static bool IsTransientError(HttpStatusCode statusCode)
    {
        return statusCode is
            HttpStatusCode.RequestTimeout or
            HttpStatusCode.TooManyRequests or
            HttpStatusCode.InternalServerError or
            HttpStatusCode.BadGateway or
            HttpStatusCode.ServiceUnavailable or
            HttpStatusCode.GatewayTimeout;
    }
}
