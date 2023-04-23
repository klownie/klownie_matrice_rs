use num::complex::Complex;
use num::Num;
use num::One;
use num::Zero;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::{self, Display, Formatter};
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrice<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrice<T>
where
    T: Zero + Clone,
{
    pub fn new(hauteur: usize, longueur: usize) -> Self {
        let data = vec![vec![T::zero(); longueur]; hauteur];
        Self { data }
    }
}

impl<T> Matrice<T>
where
    T: Clone,
{
    pub fn remplir(&mut self, numbers: Vec<T>) -> &Self {
        // Verify qu'il y a assez d'element pour remplir la matrice
        assert!(
            numbers.len() == self.data.len() * self.data[0].len(),
            "le vecteur est trop grand ou pas assez grand"
        );

        for (i, row) in self.data.iter_mut().enumerate() {
            for j in 0..row.len() {
                row[j] = numbers[i * row.len() + j].clone();
            }
        }

        self
    }
}

impl<T> Matrice<T>
where
    Standard: Distribution<T>,
    T: Zero + Clone + SampleUniform + PartialOrd + One,
{
    pub fn junk(hauteur: usize, longueur: usize, range: std::ops::Range<T>) -> Self {
        let mut data = vec![vec![T::zero(); longueur]; hauteur];
        let mut rng = rand::thread_rng();
        data.iter_mut().for_each(|row| {
            row.iter_mut()
                .for_each(|element| *element = rng.gen_range(range.clone()))
        });
        Self { data }
    }
}

impl<T> Matrice<Complex<T>>
where
    T: Clone + Zero + Num + SampleUniform + PartialOrd,
    Standard: Distribution<T>,
{
    pub fn junk_complex(
        hauteur: usize,
        longueur: usize,
        range_re: std::ops::Range<T>,
        range_im: std::ops::Range<T>,
    ) -> Self {
        let mut data = vec![vec![Complex::zero(); longueur]; hauteur];
        let mut rng = rand::thread_rng();
        data.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|element| {
                *element = Complex::new(
                    rng.gen_range(range_re.clone()),
                    rng.gen_range(range_im.clone()),
                )
            })
        });
        Self { data }
    }
}

impl<T> Matrice<T>
where
    T: Zero + Clone + One,
{
    pub fn pow(&self, exp: u32) -> Matrice<T> {
        assert_eq!(
            self.data.len(),
            self.data[0].len(),
            "La matrice doit être carrée pour être élevée à une puissance"
        );

        if exp == 0 {
            // Retourn la matrice identité
            let size = self.data.len();
            let mut result = Matrice::<T>::new(size, size);
            for i in 0..size {
                result.data[i][i] = T::one();
            }
            return result;
        }

        let mut result = self.clone();
        for _ in 1..exp {
            result = &result * self;
        }
        result
    }
}

impl<T> Mul for &Matrice<T>
where
    T: Zero + Clone + Mul<Output = T>,
{
    type Output = Matrice<T>;

    fn mul(self, other: Self) -> Matrice<T> {
        // Verifie si les deux matrice son multiplicatable entre elle
        assert_eq!(
            self.data[0].len(),
            other.data.len(),
            "C'est matrice ne peuvent pas etre multiplier entre elle"
        );

        // Create a new matrix to hold the result
        let mut result = Matrice::<T>::new(self.data.len(), other.data[0].len());

        // Compute the matrix product
        for i in 0..self.data.len() {
            for j in 0..other.data[0].len() {
                let mut sum = T::zero();
                for k in 0..other.data.len() {
                    sum = sum + self.data[i][k].clone() * other.data[k][j].clone();
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}

impl<T> Display for Matrice<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut max_widths = vec![0; self.data[0].len()];
        for row in &self.data {
            for (i, elem) in row.iter().enumerate() {
                let width = format!("{}", elem).len();
                if width > max_widths[i] {
                    max_widths[i] = width;
                }
            }
        }

        let sum: usize = max_widths.iter().sum();

        write!(f, "{:<with$}  ┐", "┌", with = sum + max_widths.len())?;
        writeln!(f)?;
        for row in &self.data {
            write!(f, "│ ")?;
            for (i, elem) in row.iter().enumerate() {
                write!(f, "{:<width$} ", elem, width = max_widths[i])?;
            }
            write!(f, "│")?;
            writeln!(f)?;
        }
        write!(f, "{:<with$}  ┘", "└", with = sum + max_widths.len())?;
        Ok(())
    }
}
