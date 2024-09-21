// use nalgebra::{
//     allocator::{Allocator, SameShapeAllocator},
//     constraint::{AreMultipliable, SameNumberOfRows, ShapeConstraint},
//     ArrayStorage, Const, DefaultAllocator, DimMin, DimMinimum, SMatrix,
// };

// Works cited:
//
// [1] E. K.-W. Chu, H.-Y. Fan, W.-W. Lin & C.-S. Wang "Structure-Preserving
//     Algorithms for Periodic Discrete-Time Algebraic Riccati Equations",
//     International Journal of Control, 77:8, 767-788, 2004.
//     DOI: 10.1080/00207170410001714988

// use thiserror::Error;
// #[derive(Debug, Clone, Copy, Error)]
// pub enum DareError {
//     #[error("The matrix is not positive definite.")]
//     MatrixNotDefinitePositive,
//     #[error("W_solver not invertible.")]
//     WNotInvertible
// }
// type DareResult<T> = Result<T, DareError>;

// /// Discrete Algebraic Riccati Equation
// /// 
// /// # Errors
// /// - [`DareError::MatrixNotDefinitePositive`] if the R matrix is not positive definite.
// /// - [`DareError::WNotInvertible`] if the W matrix is not invertible.
// #[allow(clippy::many_single_char_names, clippy::type_repetition_in_bounds, non_snake_case)]
// pub fn dare<const STATES: usize, const INPUTS: usize>(
//     A: &SMatrix<f64, STATES, STATES>,
//     B: &SMatrix<f64, STATES, INPUTS>,
//     Q: &SMatrix<f64, STATES, STATES>,
//     R: &SMatrix<f64, INPUTS, INPUTS>,
// ) -> DareResult<SMatrix<f64, STATES, STATES>>
// where
//     DefaultAllocator:
//         Allocator<f64, Const<STATES>, Const<STATES>, Buffer = ArrayStorage<f64, STATES, STATES>>,
//     DefaultAllocator:
//         Allocator<f64, Const<STATES>, Const<INPUTS>, Buffer = ArrayStorage<f64, STATES, INPUTS>>,
//     DefaultAllocator:
//         SameShapeAllocator<f64, Const<STATES>, Const<STATES>, Const<STATES>, Const<STATES>>,
//     DefaultAllocator: Allocator<(usize, usize), DimMinimum<Const<STATES>, Const<STATES>>>,

//     ShapeConstraint: AreMultipliable<Const<STATES>, Const<INPUTS>, Const<INPUTS>, Const<STATES>>,
//     ShapeConstraint: SameNumberOfRows<Const<INPUTS>, Const<INPUTS>>,
//     ShapeConstraint: SameNumberOfRows<Const<STATES>, Const<STATES>, Representative = Const<STATES>>,

//     Const<STATES>: DimMin<Const<STATES>, Output = Const<STATES>>,
// {
//     // Implements the SDA algorithm on page 5 of [1].

//     // A₀ = A
//     let mut A_k = *A; //implicit copy

//     // G₀ = BR⁻¹Bᵀ
//     //
//     // See equation (4) of [1].
//     let mut G_k = B * R.cholesky()
//         .ok_or(DareError::MatrixNotDefinitePositive)?.solve(&B.transpose());

//     // H₀ = Q
//     //
//     // See equation (4) of [1].
//     let mut H_k;
//     let mut H_k1 = *Q; //implicit copy

//     loop {
//         H_k = H_k1;

//         // W = I + GₖHₖ
//         let W = SMatrix::<f64, STATES, STATES>::identity() + G_k * H_k;

//         let W_solver = W.lu();

//         // Solve WV₁ = Aₖ for V₁
//         let V_1 = W_solver.solve(&A_k).ok_or(DareError::WNotInvertible)?;

//         // Solve V₂Wᵀ = Gₖ for V₂
//         //
//         // We want to put V₂Wᵀ = Gₖ into Ax = b form so we can solve it more
//         // efficiently.
//         //
//         // V₂Wᵀ = Gₖ
//         // (V₂Wᵀ)ᵀ = Gₖᵀ
//         // WV₂ᵀ = Gₖᵀ
//         //
//         // The solution of Ax = b can be found via x = A.solve(b).
//         //
//         // V₂ᵀ = W.solve(Gₖᵀ)
//         // V₂ = W.solve(Gₖᵀ)ᵀ
//         let V_2 = W_solver.solve(&G_k.transpose())
//             .ok_or(DareError::WNotInvertible)?.transpose();

//         // Gₖ₊₁ = Gₖ + AₖV₂Aₖᵀ
//         G_k += A_k * V_2 * A_k.transpose();

//         // Hₖ₊₁ = Hₖ + V₁ᵀHₖAₖ
//         H_k1 = H_k + V_1.transpose() * H_k * A_k;

//         // Aₖ₊₁ = AₖV₁
//         A_k *= V_1;

//         // while |Hₖ₊₁ − Hₖ| > ε |Hₖ₊₁|
//         if (H_k1 - H_k).norm() <= 1e-10f64 * H_k1.norm() {
//             break;
//         }
//     }

//     Ok(H_k1)
// }