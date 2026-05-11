import example_polars


def test_sum_as_string() -> None:
    assert example_polars.sum_as_string(5, 20) == "25"
    