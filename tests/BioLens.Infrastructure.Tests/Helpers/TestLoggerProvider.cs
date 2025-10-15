using Microsoft.Extensions.Logging;
using Xunit.Abstractions;

namespace BioLens.Infrastructure.Tests.Helpers;

/// <summary>
/// Extensions for logging with XUnit.
/// </summary>
internal static class XUnitLoggerExtensions
{
    /// <summary>
    /// Creates a logger factory configured to write to XUnit test output.
    /// </summary>
    private static ILoggerFactory CreateXUnitLoggerFactory(this ITestOutputHelper output)
    {
        ArgumentNullException.ThrowIfNull(output);
        return LoggerFactory.Create(builder => builder.AddProvider(new XUnitLoggerProvider(output)));
    }

    /// <summary>
    /// Creates a logger for the specified category that writes to XUnit test output.
    /// </summary>
    public static ILogger<T> CreateXUnitLogger<T>(this ITestOutputHelper output)
    {
        ArgumentNullException.ThrowIfNull(output);

        // Create and store the factory in a using statement to ensure disposal
        using var factory = output.CreateXUnitLoggerFactory();
        // The logger will continue to work after factory disposal
        return factory.CreateLogger<T>();
    }

    /// <summary>
    /// Simple logger provider that writes to XUnit test output.
    /// </summary>
    private sealed class XUnitLoggerProvider(ITestOutputHelper output) : ILoggerProvider
    {
        private readonly ITestOutputHelper _output = output ?? throw new ArgumentNullException(nameof(output));

        public ILogger CreateLogger(string categoryName) => new XUnitTestLogger(_output, categoryName);

        public void Dispose() { }
    }

    /// <summary>
    /// Simple logger implementation that writes to XUnit test output.
    /// </summary>
    private sealed class XUnitTestLogger(ITestOutputHelper output, string categoryName) : ILogger
    {
        public IDisposable? BeginScope<TState>(TState state) where TState : notnull => null;

        public bool IsEnabled(LogLevel logLevel) => true;

        public void Log<TState>(LogLevel logLevel, EventId eventId, TState state, Exception? exception,
            Func<TState, Exception?, string>? formatter)
        {
            if (formatter is null)
                return;

            try
            {
                output.WriteLine($"[{logLevel}] {categoryName}: {formatter(state, exception)}");

                if (exception != null)
                    output.WriteLine($"Exception: {exception}");
            }
            catch (InvalidOperationException)
            {
                // Catch only InvalidOperationException which occurs when writing to
                // ITestOutputHelper after test completion
            }
        }
    }
}
