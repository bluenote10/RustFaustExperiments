#!/usr/bin/env python

import json
from pathlib import Path
from statistics import median

import pandas as pd
import tabulate


def load(benchmark: str, entry: str) -> float:
    path = Path(__file__).parent / "results" / benchmark / f"{entry}.json"
    with path.open() as f:
        data = json.load(f)
        data = [x / 1024 / 1024 for x in data]
        return median(data)


benchmarks = [
    "copy1",
    "copy2",
    "math",
    "delay",
    "karplus32",
    "reverb",
]

entries = [
    "rust",
    "cpp_no_fastmath",
    "cpp_fastmath",
]
entries_to_names = {
    "rust": "Rust",
    "cpp_no_fastmath": "C++ (no fastmath)",
    "cpp_fastmath": "C++ (fastmath)",
}

records = [
    {benchmark: load(benchmark, entry) for benchmark in benchmarks} for entry in entries
]

df = pd.DataFrame.from_records(
    records, index=[entries_to_names[entry] for entry in entries]
).transpose()


table_str = tabulate.tabulate(
    df,  # pyright: ignore
    tablefmt="pipe",
    headers="keys",
    floatfmt=".1f",
)
table_str = table_str.replace("nan  ", "    -")
print(table_str)
