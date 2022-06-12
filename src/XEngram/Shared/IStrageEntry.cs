namespace XEngram.Shared;

interface IStrageEntry
{
    public string Name { get; }
    public abstract IEnumerable<IStrageEntry> Children { get; }
}
