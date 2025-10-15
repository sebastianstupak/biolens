using System.Text.Json;
using BioLens.Infrastructure.DataSources.UniProt;

namespace BioLens.Infrastructure.Tests.DataSources.UniProt;

public class UniProtMapperTests
{
    [Fact]
    public async Task MapToProteinValidResponseSuccess()
    {
        var json = await UniProtTestData.GetSnapshotAsync();
        var response = JsonSerializer.Deserialize<UniProtProteinResponse>(json);
        var protein = UniProtMapper.MapToProtein(response!);

        Assert.Equal("P04637", protein.Id);
        Assert.NotNull(protein.Organism);
        Assert.NotNull(protein.Sequence);
    }
}
