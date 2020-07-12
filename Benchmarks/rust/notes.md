## Relevant resources

- [Taking Advantage of Auto-Vectorization in Rust](https://www.nickwilcox.com/blog/autovec/)
- [Does rustc have auto-vectorization?](https://www.reddit.com/r/rust/comments/8uccla/does_rustc_have_autovectorization/)
- [Is there a clear syntax for iterating multiple iterators at once?](https://users.rust-lang.org/t/is-there-a-clear-syntax-for-iterating-multiple-iterators-at-once/4024)
- [How to “zip” two slices efficiently](https://users.rust-lang.org/t/how-to-zip-two-slices-efficiently/2048)
- [Iterator and compiler auto-vectorization issue](https://github.com/rust-lang/rust/issues/66268#issuecomment-552193374)
- [Auto-vectorization in Rust](https://users.rust-lang.org/t/auto-vectorization-in-rust/24379/4)
- [faster](https://github.com/AdamNiederer/faster)
- [Towards fearless SIMD](https://raphlinus.github.io/rust/simd/2018/10/19/fearless-simd.html)
- [fearless_simd](https://github.com/raphlinus/fearless_simd)
- [synthesizer-io](https://github.com/raphlinus/synthesizer-io/tree/master/synthesizer-io-core)
- [RFC portable packed SIMD](https://github.com/gnzlbg/rfcs/blob/ppv/text/0000-ppv.md)

## Test on Compiler Explorer

- [`math`](https://rust.godbolt.org/z/z5EW8j)


## Variations on `copy1`:

This is slightly faster:

```rust
    let input0 = &inputs[0];
    let output0 = &mut outputs[0];
    // assert!(input0.len() >= count as usize);
    // assert!(output0.len() >= count as usize);
    for i in 0..count {
        //outputs[0][i as usize] = (inputs[0][i as usize] as f32);
        unsafe {
            *output0.get_unchecked_mut(i as usize) = (*input0.get_unchecked(i as usize) as f32);
        }
    }
```

## Variantions on `math`:

With unchecked gets:

```rust
        unsafe {
            for i in 0..count {
                let i = i as usize;
                *outputs.get_unchecked_mut(0).get_unchecked_mut(i as usize) = (((((*inputs.get_unchecked(2).get_unchecked(i as usize) as f32)
                    + (*inputs.get_unchecked(3).get_unchecked(i as usize) as f32))
                    * ((*inputs.get_unchecked(0).get_unchecked(i as usize) as f32) + (*inputs.get_unchecked(1).get_unchecked(i as usize) as f32)))
                    / (((*inputs.get_unchecked(6).get_unchecked(i as usize) as f32) + (*inputs.get_unchecked(7).get_unchecked(i as usize) as f32))
                        * ((*inputs.get_unchecked(4).get_unchecked(i as usize) as f32) + (*inputs.get_unchecked(5).get_unchecked(i as usize) as f32))))
                    as f32);
            }
        }
```

With explictly sized slices:

```
        let input0 = &inputs[0][..count as usize];
        let input1 = &inputs[1][..count as usize];
        let input2 = &inputs[2][..count as usize];
        let input3 = &inputs[3][..count as usize];
        let input4 = &inputs[4][..count as usize];
        let input5 = &inputs[5][..count as usize];
        let input6 = &inputs[6][..count as usize];
        let input7 = &inputs[7][..count as usize];
        let output0 = &mut outputs[0][..count as usize];

        for i in 0..count {
            output0[i as usize] = (((((input2[i as usize] as f32)
                + (input3[i as usize] as f32))
                * ((input0[i as usize] as f32) + (input1[i as usize] as f32)))
                / (((input6[i as usize] as f32) + (input7[i as usize] as f32))
                    * ((input4[i as usize] as f32) + (input5[i as usize] as f32))))
                as f32);
        }
```

## Variantions on `reverb`:

```rust

        let inputs0 = &inputs[0][..count as usize];
        let inputs1 = &inputs[1][..count as usize];
        assert!(outputs.len() >= 2);
        let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = &mut outputs[0..2] {
            let outputs0 = &mut outputs0[..count as usize];
            let outputs1 = &mut outputs1[..count as usize];
            (outputs0, outputs1)
        } else {
            panic!("wrong number of outputs");
        };

        for (((input0, input1), output0), output1) in inputs0.iter().zip(inputs1.iter()).zip(outputs0.iter_mut()).zip(outputs1.iter_mut()) {
            // ...
            let mut fTemp0: f32 = (*input0 as f32);
            let mut fTemp3: f32 = (*input1 as f32);
            // same with output
        }

```