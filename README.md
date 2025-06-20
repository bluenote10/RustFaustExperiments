# Benchmarks

DSP throughput comparison of Rust vs C++:

|           |   Rust (latest) |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|----------------:|--------------------:|-----------------:|
| copy1     |         38842.1 |             22208.2 |          23093.6 |
| copy2     |         36039.6 |             51440.7 |          53407.6 |
| delay     |          5427.2 |              4926.0 |           4838.6 |
| math      |          6498.1 |              7229.7 |           7268.5 |
| karplus32 |            99.2 |                61.2 |             69.9 |
| reverb    |            95.1 |                92.9 |             98.3 |
| osci      |           556.6 |               532.7 |            536.9 |

Numbers are output throughput in MiB/s, i.e., higher is better.

Versions:
- Faust version: 2.81.1 (902f4ebab)
- Rust version: 1.87.0
- g++ version: 11.4.0

OS: Ubuntu 22.04

<details>
<summary>CPU info</summary>

```
Architecture:            x86_64
  CPU op-mode(s):        32-bit, 64-bit
  Address sizes:         39 bits physical, 48 bits virtual
  Byte Order:            Little Endian
CPU(s):                  4
  On-line CPU(s) list:   0-3
Vendor ID:               GenuineIntel
  Model name:            Intel(R) Core(TM) i5-4670 CPU @ 3.40GHz
    CPU family:          6
    Model:               60
    Thread(s) per core:  1
    Core(s) per socket:  4
    Socket(s):           1
    Stepping:            3
    CPU max MHz:         3800,0000
    CPU min MHz:         800,0000
    BogoMIPS:            6800.55
    Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl vmx smx est t
                         m2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid xsa
                         veopt dtherm ida arat pln pts md_clear flush_l1d
Virtualization features:
  Virtualization:        VT-x
Caches (sum of all):
  L1d:                   128 KiB (4 instances)
  L1i:                   128 KiB (4 instances)
  L2:                    1 MiB (4 instances)
  L3:                    6 MiB (1 instance)
NUMA:
  NUMA node(s):          1
  NUMA node0 CPU(s):     0-3
Vulnerabilities:
  Itlb multihit:         KVM: Mitigation: VMX disabled
  L1tf:                  Mitigation; PTE Inversion; VMX conditional cache flushes, SMT disabled
  Mds:                   Mitigation; Clear CPU buffers; SMT disabled
  Meltdown:              Mitigation; PTI
  Mmio stale data:       Unknown: No mitigations
  Retbleed:              Not affected
  Spec store bypass:     Mitigation; Speculative Store Bypass disabled via prctl and seccomp
  Spectre v1:            Mitigation; usercopy/swapgs barriers and __user pointer sanitization
  Spectre v2:            Mitigation; Retpolines, IBPB conditional, IBRS_FW, STIBP disabled, RSB filling, PBRSB-eIBRS Not affected
  Srbds:                 Mitigation; Microcode
  Tsx async abort:       Not affected
```
</details>

**Note about fastmath:**
In the long term this could be solved by using [fastmath intrinsics](https://doc.rust-lang.org/core/intrinsics/fn.fadd_fast.html)
(currently a nightly feature) or [future higher-level solutions](https://github.com/rust-lang/rust/issues/21690).
