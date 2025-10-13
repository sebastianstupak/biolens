using System.Text.Json;
using BioLens.Infrastructure.DataSources.UniProt;

namespace BioLens.Infrastructure.Tests.DataSources.UniProt;

[Trait("Category", "Contract")]
[Trait("DataSource", "UniProt")]
public class UniProtContractTests
{
    [Fact]
    public async Task Contract_APIStructureUnchanged()
    {
        var liveJson = await UniProtTestData.FetchLiveAsync();
        var snapshotJson = await UniProtTestData.GetSnapshotAsync();

        var liveStructure = ExtractStructure(JsonDocument.Parse(liveJson).RootElement);
        var snapshotStructure = ExtractStructure(JsonDocument.Parse(snapshotJson).RootElement);

        var liveStructureJson = JsonSerializer.Serialize(liveStructure, new JsonSerializerOptions { WriteIndented = true });
        var snapshotStructureJson = JsonSerializer.Serialize(snapshotStructure, new JsonSerializerOptions { WriteIndented = true });

        if (liveStructureJson != snapshotStructureJson)
        {
            await UniProtTestData.UpdateSnapshotAsync(liveJson);
            Assert.Fail("API structure changed. Snapshot updated. Review structure changes and update DTO if needed.");
        }

        var response = JsonSerializer.Deserialize<UniProtProteinResponse>(liveJson);
        var protein = UniProtMapper.MapToProtein(response!);

        Assert.Equal("P04637", protein.Id);
    }

    private static object ExtractStructure(JsonElement element)
    {
        return element.ValueKind switch
        {
            JsonValueKind.Object => new
            {
                Type = "Object",
                Properties = element.EnumerateObject()
                    .ToDictionary(p => p.Name, p => ExtractStructure(p.Value))
            },
            JsonValueKind.Array => new
            {
                Type = "Array",
                ItemType = element.GetArrayLength() > 0 
                    ? ExtractStructure(element[0]) 
                    : "Empty"
            },
            JsonValueKind.String => "String",
            JsonValueKind.Number => "Number",
            JsonValueKind.True or JsonValueKind.False => "Boolean",
            JsonValueKind.Null => "Null",
            _ => "Unknown"
        };
    }
}