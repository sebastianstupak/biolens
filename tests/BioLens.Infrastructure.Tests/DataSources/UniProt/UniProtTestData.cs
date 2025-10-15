using System.Runtime.CompilerServices;

namespace BioLens.Infrastructure.Tests.DataSources.UniProt;

internal static class UniProtTestData
{
    private static string? _cached;
    private static readonly HttpClient _httpClient = new() { BaseAddress = new Uri("https://rest.uniprot.org") };

    private static string GetProjectDirectory([CallerFilePath] string? callerFilePath = null)
        => Path.GetDirectoryName(callerFilePath)!;

    private static string GetSnapshotPath()
        => Path.Combine(GetProjectDirectory(), "Snapshots", "p04637_snapshot.json");

    public static async Task<string> GetSnapshotAsync()
    {
        if (_cached != null)
            return _cached;

        var snapshotPath = GetSnapshotPath();

        if (!File.Exists(snapshotPath))
        {
            var liveJson = await FetchLiveAsync().ConfigureAwait(false);
            await UpdateSnapshotAsync(liveJson).ConfigureAwait(false);
            _cached = liveJson;
            return _cached;
        }

        _cached = await File.ReadAllTextAsync(snapshotPath).ConfigureAwait(false);
        return _cached;
    }

    public static async Task<string> FetchLiveAsync()
    {
        var uri = new Uri("uniprotkb/P04637.json", UriKind.Relative);
        var response = await _httpClient.GetAsync(uri).ConfigureAwait(false);
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync().ConfigureAwait(false);
    }

    public static async Task UpdateSnapshotAsync(string json)
    {
        var snapshotPath = GetSnapshotPath();
        var directory = Path.GetDirectoryName(snapshotPath);
        if (!string.IsNullOrEmpty(directory))
            Directory.CreateDirectory(directory);

        await File.WriteAllTextAsync(snapshotPath, json).ConfigureAwait(false);
    }
}
