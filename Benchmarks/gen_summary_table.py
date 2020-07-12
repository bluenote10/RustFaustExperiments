#!/usr/bin/env python

import pandas as pd
import tabulate

rust = {
    "copy1": 5052.425,
    "copy2": 5040.145,
    "math": 951.463,
    "karplus32": 80.531,
    "reverb": 81.858,
}

cpp_no_fastmath = {
    "copy1": 19449.679,
    "copy2": 65471.413,
    "math": 6688.054,
    "karplus32": 77.956,
    "reverb": 86.474,
}

cpp_fastmath = {
    "copy1": 19261.800,
    "copy2": 65031.774,
    "math": 7156.849,
    "karplus32": 80.897,
    "reverb": 105.350,
}

df = pd.DataFrame(
    [
        rust,
        cpp_no_fastmath,
        cpp_fastmath
    ],
    index=[
        "Rust",
        "C++ (no fastmath)",
        "C++ (fastmath)",
    ],
).transpose()

print(tabulate.tabulate(df, tablefmt="pipe", headers="keys", floatfmt=".1f"))

# import IPython; IPython.embed()
