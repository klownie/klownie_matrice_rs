# klownie_matrix_rs
A matrice implementation that I made to learn rust

# exemple of usage with diferent types of numbers

```rust
use klownie_matrix::Matrice;
use num::complex::Complex;
use num::BigInt;
use num::BigRational;
use std::num::Wrapping;

fn main() {
    // crée une nouvelle matrice
    let mut a = Matrice::<u128>::new(4, 4);
    println!("la Matrice a ressemble a :\n{}", a);
    // crée une matrice junk
    let b = Matrice::junk(4, 10, 5..11);
    println!("B matrice au hazard :\n{}", b);
    // remplir la matrice
    a.remplir(vec![1, 2, 3, 4, 56, 8, 3, 9, 11, 99, 9, 7, 89, 43, 2, 0]);
    println!("matrice A remplie :\n{}", a);
    // multiplication de matrice avec nombre entier
    println!("La multiplication de A par B fait :\n{}", &a * &b);
    // puissance
    for i in 0..6 {
        println!("Matrice A à la puissance {}:\n{}", i, &a.pow(i))
    }
    // crée une matrice avec des nombre complexe
    let mut c = Matrice::<Complex<i128>>::new(2, 5);
    c.remplir(
        [
            (1, 4),
            (2, 5),
            (2, 5),
            (2, 5),
            (2, 5),
            (2, 5),
            (2, 5),
            (2, 5),
            (2, 5),
            (2, 5),
        ]
        .into_iter()
        .map(|(re, im)| Complex::new(re, im))
        .collect(),
    );
    println!("La matrice C avec des nombre ccomplexe :\n{}", c);
    // matrice junk complexe
    let d = Matrice::junk_complex(5, 5, 0..2, 0..2);
    println!("D matrice au hazard avec des nombre complexe : \n{}", d);
    // multiplication de matrice avec des nombre complexe
    println!(
        "Multiplication de matrice avec des nombre complexe :\n{}",
        &c * &d
    );
    // puissance complexe
    for i in 0..8 {
        println!("Matrice D à la puissance {}:\n{}", i, &d.pow(i))
    }
    // matrice avec fraction
    let mut e = Matrice::<BigRational>::new(2, 2);
    e.remplir(
        [3, 4, 7, 1]
            .into_iter()
            .map(|numer| BigRational::new(BigInt::from(numer), BigInt::from(1)))
            .collect(),
    );
    println!("Matrice E fractionel:\n{}", e);
    // matrice BigRational puissance
    for i in 0..5 {
        println!("Matrice E a la pussance {} :\n{}", i, e.pow(i))
    }
    // matrice BigRational complexe
    let mut f = Matrice::<Complex<BigRational>>::new(3, 3);
    f.remplir(
        [
            (1, 4),
            (2, 5),
            (-1, 5),
            (2, 5),
            (2, 5),
            (2, 5),
            (2, -5),
            (0, 0),
            (1, 1),
        ]
        .into_iter()
        .map(|(re, im)| {
            Complex::new(
                BigRational::new(BigInt::from(re), BigInt::from(1)),
                BigRational::new(BigInt::from(im), BigInt::from(1)),
            )
        })
        .collect(),
    );
    println!("Matrice F fractionel:\n{}", f);
    // matrice BigRational puissance
    for i in 0..10 {
        println!("Matrice E a la puissance {} :\n{}", i, f.pow(i))
    }
    // matrice wraping numbers
    let mut g = Matrice::<Wrapping<u32>>::new(5, 5);
    g.remplir(
        [
            4, 76, 1, 2, 9, 1, 4, 5, 8, 10, 4, 5, 4, 4, 4, 4, 1, 0, 3, 67, 0, 0, 4, 5, 5,
        ]
        .into_iter()
        .map(|nb| Wrapping(nb))
        .collect(),
    );
    for i in 0..5 {
        println!(
            "Matrice G with wrapping type puissance {} :\n{}",
            i,
            g.pow(i)
        )
    }
}
```
