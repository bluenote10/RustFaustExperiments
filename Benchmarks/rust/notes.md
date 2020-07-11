## Variations on `copy1` generated code:

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