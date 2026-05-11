import polars as pl

from example_polars import pig_latinnify


def main() -> None:

    df = pl.DataFrame(
        {
            "convert": ["pig", "latin", "is", "silly" ]
        }
    )
    out = df.with_columns(pg_latin=pig_latinnify(pl.col("convert")))
    print(out)


if __name__ == "__main__":
    main()
    