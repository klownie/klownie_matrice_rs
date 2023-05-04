use num::One;
use num::Zero;
use std::fmt::{self, Display, Formatter};
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;

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

impl<T> Add for &Matrice<T>
where
    T: Zero + Clone + Add<Output = T> ,
{
    type Output = Matrice<T>;

    fn add(self, other : Self) -> Matrice<T> {
        assert_eq!(
            (self.data.len(),self.data[0].len()),
            (other.data.len(),other.data[0].len()),
            "Ces matrice ne peuvent pas etre additionner entre elle "
        );

        // Cree une nouvelle matrice qui contiendra le resultat
        let mut result = Matrice::new(self.data.len(),self.data[0].len());

        // Calcule d'addition entre matrice
        for i in 0..self.data.len(){
            result.data[i] = self.data[i].iter().zip(other.data[i].clone()).map(|(x1 , x2)| x1.clone() + x2.clone() ).collect()
        }

        result

    }

}

impl<T> Sub for &Matrice<T>
where
    T: Zero + Clone + Sub<Output = T> ,
{
    type Output = Matrice<T>;

    fn sub(self, other : Self) -> Matrice<T> {
        assert_eq!(
            (self.data.len(),self.data[0].len()),
            (other.data.len(),other.data[0].len()),
            "Ces matrice ne peuvent pas etre additionner entre elle "
        );

        // Cree une nouvelle matrice qui contiendra le resultat
        let mut result = Matrice::new(self.data.len(),self.data[0].len());

        // Calcule d'addition entre matrice
        for i in 0..self.data.len(){
            result.data[i] = self.data[i].iter().zip(other.data[i].clone()).map(|(x1 , x2)| x1.clone() - x2.clone() ).collect()
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
            "Ces matrice ne peuvent pas etre multiplier entre elle"
        );

        // Cree une nouvel matrice qui contiendra le produit
        let mut result = Matrice::new(self.data.len(), other.data[0].len());

        // Calcule de produit entre matrice
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

#[macro_export]
macro_rules! mat {
    ($rows:expr, $cols:expr, $v:expr) => {{
        assert_eq!($v.len(), $rows * $cols);

        let mut matrice = Matrice::new($rows, $cols);
        matrice.remplir($v);

        matrice
    }};
}

