// 01_rust_embedding_design.rs

/// Represents our 1746-bit Gödel number as an array of 28 u64 integers.
/// 28 * 64 = 1792 bits, which is enough to hold our 1746-bit number.
pub type GodelNumber = [u64; 28];

/// Represents the 4x7 matrix form of the Gödel number.
/// 4 * 7 = 28, so the number of u64 elements is the same.
pub type GodelMatrix = [[u64; 7]; 4];

/// The Univalent Embedding for a single term in our system.
/// It holds the term's harmonic property and its unique Gödel number.
#[derive(Debug, PartialEq, Eq)]
pub struct UnivalentEmbedding {
    /// The reciprocal of the term's harmonic prime (1/p).
    pub reciprocal_harmonic: f64,
    /// The unique, 1746-bit Gödel number for the term.
    pub godel_number: GodelNumber,
}

impl UnivalentEmbedding {
    /// Creates a new UnivalentEmbedding.
    pub fn new(reciprocal_harmonic: f64, godel_number: GodelNumber) -> Self {
        Self {
            reciprocal_harmonic,
            godel_number,
        }
    }

    /// Resamples the Gödel number into a 4x7 matrix representation.
    /// This demonstrates how the linear number can be viewed as a 2D structure.
    pub fn to_matrix(&self) -> GodelMatrix {
        let mut matrix: GodelMatrix = [[0; 7]; 4];
        for i in 0..4 {
            for j in 0..7 {
                matrix[i][j] = self.godel_number[i * 7 + j];
            }
        }
        matrix
    }

    /// Constructs a UnivalentEmbedding from a 4x7 matrix,
    /// demonstrating the equivalence is two-way.
    pub fn from_matrix(reciprocal_harmonic: f64, matrix: GodelMatrix) -> Self {
        let mut godel_number: GodelNumber = [0; 28];
        for i in 0..4 {
            for j in 0..7 {
                godel_number[i * 7 + j] = matrix[i][j];
            }
        }
        Self {
            reciprocal_harmonic,
            godel_number,
        }
    }
}

// --- Example Usage ---
// let original_embedding = UnivalentEmbedding::new(0.5, [1,2,3,...,28]);
// let matrix = original_embedding.to_matrix();
// let reconstructed_embedding = UnivalentEmbedding::from_matrix(0.5, matrix);
// assert_eq!(original_embedding, reconstructed_embedding);
