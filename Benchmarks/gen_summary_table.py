#!/usr/bin/env python

import pandas as pd
import tabulate

rust = {
    "copy1": 5052.425,
    "copy2": 5040.145,
    "math": 951.463,
    "delay": None,
    "karplus32": 84.960,
    "reverb": 81.858,
}

rust_iterators = {
    "copy1": 36549.387,
    "copy2": 41046.452,
    "math": 6537.100,
    "delay": 1948.203,
    "karplus32": 85.565,
    "reverb": 82.679,
}

cpp_no_fastmath = {
    "copy1": 19449.679,
    "copy2": 65471.413,
    "math": 6688.054,
    "delay": 4703.031,
    "karplus32": 77.956,
    "reverb": 86.474,
}

cpp_fastmath = {
    "copy1": 19261.800,
    "copy2": 65031.774,
    "math": 7156.849,
    "delay": 4742.733,
    "karplus32": 80.897,
    "reverb": 105.350,
}

full_table = True

if full_table:
    df = pd.DataFrame(
        [
            rust_iterators,
            rust,
            cpp_no_fastmath,
            cpp_fastmath
        ],
        index=[
            "Rust (`rust-wip`)",
            "Rust (`master-dev`)",
            "C++ (no fastmath)",
            "C++ (fastmath)",
        ],
    ).transpose()

else:
    df = pd.DataFrame(
        [
            rust,
            rust_iterators,
            cpp_no_fastmath,
            cpp_fastmath
        ],
        index=[
            "Rust",
            "Rust (iterators)",
            "C++ (no fastmath)",
            "C++ (fastmath)",
        ],
    ).transpose()


table_str = tabulate.tabulate(df, tablefmt="pipe", headers="keys", floatfmt=".1f")
table_str = table_str.replace("nan  ", "    -")
print(table_str)

# import IPython; IPython.embed()
