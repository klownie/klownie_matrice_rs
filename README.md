# klownie_matrix_rs

A matrice implementation that I made to learn rust

# exemple of usage with diferent types of numbers

```rust
use klownie_matrice::*;
use num::complex::Complex;
use std::num::Wrapping;

fn main() {
    let now = std::time::Instant::now();

    let a = mat!(
        4,
        4,
        [
            (4, 1),
            (20, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (0, 0),
            (0, 1),
            (5, 1),
            (2, 2),
            (1, 1),
            (3, 1),
            (0, 1),
            (4, 9),
            (1, 50)
        ]
        .into_iter()
        .map(|(re, im)| Complex::new(Wrapping(re), Wrapping(im)))
        .collect::<Vec<Complex<Wrapping<u16>>>>()
    );
    for i in 0..445 {
        println!("puissance {} :\n{}", i, a.pow(i))
    }

    let elapsed = now.elapsed();
    println!("Éxecuté en : {:.2?}", elapsed);
}
```
