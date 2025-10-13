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
        _logger = logger;
        _maxRetryAttempts = maxRetryAttempts;
        _retryDelayMilliseconds = retryDelayMilliseconds;
    }

    protected override async Task<HttpResponseMessage> SendAsync(
        HttpRequestMessage request,
        CancellationToken cancellationToken)
    {
        HttpResponseMessage? response = null;
        
        for (int i = 0; i <= _maxRetryAttempts; ++i)
        {
            try
            {
                response = await base.SendAsync(request, cancellationToken);
                
                if (response.IsSuccessStatusCode || !IsTransientError(response.StatusCode))
                {
                    return response;
                }
                
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
                    
                    await Task.Delay(delay, cancellationToken);
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
                
                await Task.Delay(delay, cancellationToken);
            }
        }
        
        return response ?? throw new HttpRequestException("All retry attempts failed");
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