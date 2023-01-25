import polars as pl
from extend_polars import parallel_jaccard, unit

df = pl.DataFrame({
    "list_a": [[1, 2, 3], [5, 5]],
    "list_b": [[1, 2, 3, 8], [5, 1, 1]]
})
df2 = pl.DataFrame({
    "a": [1,2,3],
    "b": [4,5,6]
})

print("Leaving python world", df2)
# print(parallel_jaccard(df, "list_a", "list_b"))
print("Return to python world", unit(df2))
