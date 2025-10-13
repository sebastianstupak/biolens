using BioLens.Infrastructure.DataSources.UniProt;
using BioLens.Infrastructure.Extensions;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddOpenApi();

builder.Services.AddBioLensClients(builder.Configuration);

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.MapOpenApi();
}

app.UseHttpsRedirection();

app.MapGet("/api/v1/proteins/{id}", async (string id, UniProtClient client) =>
    {
        var protein = await client.GetProteinAsync(id);
        return protein is not null ? Results.Ok(protein) : Results.NotFound();
    })
    .WithName("GetProtein")
    .WithOpenApi();

app.Run();