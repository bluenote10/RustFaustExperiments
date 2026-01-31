#!/usr/bin/env python

import json
from pathlib import Path
from statistics import median
from typing import List

import pandas as pd
import tabulate


def load(benchmark: str, entry: str) -> float:
    path = Path(__file__).parent / "results" / benchmark / f"{entry}.json"
    with path.open() as f:
        data = json.load(f)
        data = [x / 1024 / 1024 for x in data]
        return median(data)


def load_binary_info(benchmark: str, entry: str) -> dict:
    path = Path(__file__).parent / "results" / benchmark / f"{entry}_binary_info.json"
    if not path.exists():
        return {}
    with path.open() as f:
        return json.load(f)


def print_table(benchmarks: List[str], entries: List[str]):
    records = [
        {benchmark: load(benchmark, entry) for benchmark in benchmarks}
        for entry in entries
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


def print_binary_info_table(benchmarks: List[str], entry: str):
    records = []
    for benchmark in benchmarks:
        info = load_binary_info(benchmark, entry)
        if info:
            records.append(
                {
                    "benchmark": benchmark,
                    "sha256": info["sha256"][:16] + "…",
                    "asm_sha256": info["asm_sha256"][:16] + "…",
                    "size (bytes)": info["size"],
                }
            )
    if not records:
        return
    df = pd.DataFrame.from_records(records).set_index("benchmark")
    table_str = tabulate.tabulate(
        df,  # pyright: ignore
        tablefmt="pipe",
        headers="keys",
    )
    print(f"\nBinary info ({entry}):\n")
    print(table_str)
    (Path(__file__).parent / "binary_info.md").write_text(table_str)


entries_to_names = {
    "rust": "Rust (latest)",
    "rust_1_66_0": "Rust (1.66.0)",
    "rust_1_67_0": "Rust (1.67.0)",
    "cpp_no_fastmath": "C++ (no fastmath)",
    "cpp_fastmath": "C++ (fastmath)",
}

benchmarks = [
    "copy1",
    "copy2",
    "delay",
    "math",
    "karplus32",
    "reverb",
    "osci",
]

print_table(
    benchmarks,
    entries=[
        "rust",
        "cpp_no_fastmath",
        "cpp_fastmath",
    ],
)

print_binary_info_table(benchmarks, "rust")

print_performance_regression_table = False

if print_performance_regression_table:
    print_table(
        benchmarks,
        entries=[
            "rust_1_67_0",
            "rust_1_66_0",
            "cpp_no_fastmath",
            "cpp_fastmath",
        ],
    )
