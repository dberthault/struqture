// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use super::{OperateOnSpins, SingleDecoherenceOperator, ToSparseMatrixSuperOperator};
use crate::fermions::FermionLindbladNoiseOperator;
use crate::mappings::JordanWignerSpinToFermion;
use crate::spins::{DecoherenceOperator, DecoherenceProduct};
use crate::{OperateOnDensityMatrix, SpinIndex, StruqtureError, SymmetricIndex};
use itertools::Itertools;
use num_complex::Complex64;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use std::iter::{FromIterator, IntoIterator};
use std::ops;

use indexmap::map::{Entry, Iter};
use indexmap::IndexMap;
use std::collections::HashMap;

/// PauliLindbladNoiseOperators represent noise interactions in the Lindblad equation.
///
/// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::spins::DecoherenceProduct] style operators.
/// We use ([crate::spins::DecoherenceProduct], [crate::spins::DecoherenceProduct]) as a unique basis.
///
/// # Example
///
/// ```
/// use struqture::prelude::*;
/// use qoqo_calculator::CalculatorComplex;
/// use struqture::spins::{DecoherenceProduct, PauliLindbladNoiseOperator};
///
/// let mut system = PauliLindbladNoiseOperator::new();
///
/// // Set noise terms:
/// let pp_01 = DecoherenceProduct::new().x(0).x(1);
/// let pp_0 = DecoherenceProduct::new().z(0);
/// system.set((pp_01.clone(), pp_01.clone()), CalculatorComplex::from(0.5)).unwrap();
/// system.set((pp_0.clone(), pp_0.clone()), CalculatorComplex::from(0.2)).unwrap();
///
/// // Access what you set:
/// assert_eq!(system.current_number_spins(), 2_usize);
/// assert_eq!(system.get(&(pp_01.clone(), pp_01.clone())), &CalculatorComplex::from(0.5));
/// assert_eq!(system.get(&(pp_0.clone(), pp_0.clone())), &CalculatorComplex::from(0.2));
/// ```
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "PauliLindbladNoiseOperatorSerialize")]
#[serde(into = "PauliLindbladNoiseOperatorSerialize")]
pub struct PauliLindbladNoiseOperator {
    // The internal map representing the noise terms
    internal_map: IndexMap<(DecoherenceProduct, DecoherenceProduct), CalculatorComplex>,
}

impl crate::SerializationSupport for PauliLindbladNoiseOperator {
    fn struqture_type() -> crate::StruqtureType {
        crate::StruqtureType::PauliLindbladNoiseOperator
    }
}
#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for PauliLindbladNoiseOperator {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "PlusMinusOperator".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <PauliLindbladNoiseOperatorSerialize>::json_schema(generator)
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(deny_unknown_fields))]
struct PauliLindbladNoiseOperatorSerialize {
    /// The vector representing the internal map of the PauliLindbladNoiseOperator
    items: Vec<(
        DecoherenceProduct,
        DecoherenceProduct,
        CalculatorFloat,
        CalculatorFloat,
    )>,
    serialisation_meta: crate::StruqtureSerialisationMeta,
}

impl TryFrom<PauliLindbladNoiseOperatorSerialize> for PauliLindbladNoiseOperator {
    type Error = StruqtureError;
    fn try_from(value: PauliLindbladNoiseOperatorSerialize) -> Result<Self, Self::Error> {
        let target_serialisation_meta =
            <Self as crate::SerializationSupport>::target_serialisation_meta();
        crate::check_can_be_deserialised(&target_serialisation_meta, &value.serialisation_meta)?;
        let new_noise_op: PauliLindbladNoiseOperator = value
            .items
            .into_iter()
            .map(|(left, right, real, imag)| {
                ((left, right), CalculatorComplex { re: real, im: imag })
            })
            .collect();
        Ok(new_noise_op)
    }
}

impl From<PauliLindbladNoiseOperator> for PauliLindbladNoiseOperatorSerialize {
    fn from(value: PauliLindbladNoiseOperator) -> Self {
        let serialisation_meta = crate::SerializationSupport::struqture_serialisation_meta(&value);

        let new_noise_op: Vec<(
            DecoherenceProduct,
            DecoherenceProduct,
            CalculatorFloat,
            CalculatorFloat,
        )> = value
            .into_iter()
            .map(|((left, right), val)| (left, right, val.re, val.im))
            .collect();
        Self {
            items: new_noise_op,
            serialisation_meta,
        }
    }
}

impl<'a> OperateOnDensityMatrix<'a> for PauliLindbladNoiseOperator {
    type Index = (DecoherenceProduct, DecoherenceProduct);
    type Value = CalculatorComplex;

    // From trait
    fn get(&self, key: &Self::Index) -> &Self::Value {
        match self.internal_map.get(key) {
            Some(value) => value,
            None => &CalculatorComplex::ZERO,
        }
    }

    // From trait
    fn iter(&'a self) -> impl ExactSizeIterator<Item = (&'a Self::Index, &'a Self::Value)> {
        self.internal_map.iter()
    }

    // From trait
    fn keys(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Index> {
        self.internal_map.keys()
    }

    // From trait
    fn values(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Value> {
        self.internal_map.values()
    }

    // From trait
    fn remove(&mut self, key: &Self::Index) -> Option<Self::Value> {
        self.internal_map.shift_remove(key)
    }

    // From trait
    fn empty_clone(&self, capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(cap),
            None => Self::new(),
        }
    }

    /// Overwrites an existing entry or sets a new entry in the PauliLindbladNoiseOperator with the given ((DecoherenceProduct, DecoherenceProduct) key, CalculatorComplex value) pair.
    ///
    /// # Arguments
    ///
    /// * `key` - The (DecoherenceProduct, DecoherenceProduct) key to set in the PauliLindbladNoiseOperator.
    /// * `value` - The corresponding CalculatorComplex value to set for the key in the PauliLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(CalculatorComplex))` - The key existed, this is the value it had before it was set with the value input.
    /// * `Ok(None)` - The key did not exist, it has been set with its corresponding value.
    /// * `Err(StruqtureError::InvalidLindbladTerms)` - The input contained identities, which are not allowed as Lindblad operators.
    fn set(
        &mut self,
        key: Self::Index,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, StruqtureError> {
        if key.0.is_empty() || key.1.is_empty() {
            return Err(StruqtureError::InvalidLindbladTerms);
        }

        if value != CalculatorComplex::ZERO {
            Ok(self.internal_map.insert(key, value))
        } else {
            match self.internal_map.entry(key) {
                Entry::Occupied(val) => Ok(Some(val.shift_remove())),
                Entry::Vacant(_) => Ok(None),
            }
        }
    }
}

impl OperateOnSpins<'_> for PauliLindbladNoiseOperator {
    /// Gets the maximum index of the PauliLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of spins in the PauliLindbladNoiseOperator.
    fn current_number_spins(&self) -> usize {
        let mut max_mode: usize = 0;
        if !self.internal_map.is_empty() {
            for key in self.internal_map.keys() {
                let maxk = (key.0.current_number_spins()).max(key.1.current_number_spins());
                if maxk > max_mode {
                    max_mode = maxk
                }
            }
        }
        max_mode
    }
}

impl ToSparseMatrixSuperOperator<'_> for PauliLindbladNoiseOperator {
    // From trait
    fn sparse_matrix_superoperator_entries_on_row(
        &self,
        row: usize,
        number_spins: usize,
    ) -> Result<HashMap<usize, Complex64>, StruqtureError> {
        let mut entries: HashMap<usize, Complex64> = HashMap::new();
        let dimension = 2_usize.pow(number_spins as u32);
        for ((left, right), value) in self.iter() {
            add_lindblad_terms(
                left,
                right,
                row,
                dimension,
                number_spins,
                &mut entries,
                value,
            )?;
            // iterate over terms corresponding to - 1/2 right^dagger * left p => -1/2 (right^dagger * left).kron(I) flatten(p)
            // and - 1/2 p right^dagger * left  => - 1/2 I.kron((right^dagger * left).T) flatten(p)
            add_anti_commutator(
                left,
                right,
                row,
                dimension,
                number_spins,
                &mut entries,
                value,
            )?;
        }
        Ok(entries)
    }
}

/// Implements the default function (Default trait) of PauliLindbladNoiseOperator (an empty PauliLindbladNoiseOperator).
///
impl Default for PauliLindbladNoiseOperator {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for the PauliLindbladNoiseOperator
///
impl PauliLindbladNoiseOperator {
    /// Creates a new PauliLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) PauliLindbladNoiseOperator.
    pub fn new() -> Self {
        PauliLindbladNoiseOperator {
            internal_map: IndexMap::new(),
        }
    }

    /// Creates a new PauliLindbladNoiseOperator with pre-allocated capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The pre-allocated capacity of the system.
    ///
    /// # Returns
    ///
    /// * `Self` - The new (empty) PauliLindbladNoiseOperator.
    pub fn with_capacity(capacity: usize) -> Self {
        PauliLindbladNoiseOperator {
            internal_map: IndexMap::with_capacity(capacity),
        }
    }

    /// Adds all noise entries corresponding to a ((DecoherenceOperator, DecoherenceOperator), CalculatorFloat).
    ///
    /// In the Lindblad equation, Linblad noise operator L_i are not limited to [crate::spins::DecoherenceProduct] style operators.
    /// We use ([crate::spins::DecoherenceProduct], [crate::spins::DecoherenceProduct]) as a unique basis.
    /// This function adds a Linblad-Term defined by a combination of Lindblad operators given as general [crate::spins::DecoherenceOperator]
    ///
    /// # Arguments
    ///
    /// * `left` - DecoherenceOperator that acts on the density matrix from the left in the Lindblad equation.
    /// * `right` -  DecoherenceOperator that acts on the density matrix from the right and in hermitian conjugated form in the Lindblad equation.
    /// * `value` - CalculatorComplex value representing the global coefficient of the noise term.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The noise was correctly added.
    /// * `Err(StruqtureError::InvalidLindbladTerms)` - The input contained identities, which are not allowed as Lindblad operators.
    pub fn add_noise_from_full_operators(
        &mut self,
        left: &DecoherenceOperator,
        right: &DecoherenceOperator,
        value: CalculatorComplex,
    ) -> Result<(), StruqtureError> {
        if left.is_empty() || right.is_empty() {
            return Err(StruqtureError::InvalidLindbladTerms);
        }

        for ((decoherence_product_left, value_left), (decoherence_product_right, value_right)) in
            left.iter().cartesian_product(right.into_iter())
        {
            if !decoherence_product_left.is_empty() && !decoherence_product_right.is_empty() {
                let value_complex = value_right.conj() * value_left;
                self.add_operator_product(
                    (
                        decoherence_product_left.clone(),
                        decoherence_product_right.clone(),
                    ),
                    value_complex * value.clone(),
                )?;
            }
        }
        Ok(())
    }

    /// Remaps the qubits in the PauliLindbladNoiseOperator.
    ///
    /// # Arguments
    ///
    /// * `mapping` - HashMap containing the qubit remapping.
    ///
    /// # Returns
    ///
    /// * `Self` - The remapped PauliLindbladNoiseOperator.
    pub fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Self {
        let mut new_noise = PauliLindbladNoiseOperator::new();
        for ((left, right), rate) in self.iter() {
            let new_left = left.remap_qubits(mapping);
            let new_right = right.remap_qubits(mapping);
            new_noise
                .add_operator_product((new_left, new_right), rate.clone())
                .expect("Internal bug in add_operator_product");
        }
        new_noise
    }

    /// Separate self into an operator with the terms of given number of spins and an operator with the remaining operations
    ///
    /// # Arguments
    ///
    /// * `number_spins_left` - Number of spins to filter for in the left term of the keys.
    /// * `number_spins_right` - Number of spins to filter for in the right term of the keys.
    ///
    /// # Returns
    ///
    /// `Ok((separated, remainder))` - Operator with the noise terms where number_spins_left and number_spins_right match the number of spins the left and right noise operator product acts on and Operator with all other contributions.
    pub fn separate_into_n_terms(
        &self,
        number_spins_left: usize,
        number_spins_right: usize,
    ) -> Result<(Self, Self), StruqtureError> {
        let mut separated = Self::default();
        let mut remainder = Self::default();
        for ((prod_l, prod_r), val) in self.iter() {
            if prod_l.iter().len() == number_spins_left && prod_r.iter().len() == number_spins_right
            {
                separated.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
            } else {
                remainder.add_operator_product((prod_l.clone(), prod_r.clone()), val.clone())?;
            }
        }
        Ok((separated, remainder))
    }

    /// Export to struqture_1 format.
    #[cfg(feature = "struqture_1_export")]
    pub fn to_struqture_1(
        &self,
    ) -> Result<struqture_1::spins::SpinLindbladNoiseSystem, StruqtureError> {
        let mut new_system = struqture_1::spins::SpinLindbladNoiseSystem::new(None);
        for (key, val) in self.iter() {
            let one_key_left = key.0.to_struqture_1()?;
            let one_key_right = key.1.to_struqture_1()?;
            let _ = struqture_1::OperateOnDensityMatrix::set(
                &mut new_system,
                (one_key_left, one_key_right),
                val.clone(),
            );
        }
        Ok(new_system)
    }

    /// Import from struqture_1 format.
    #[cfg(feature = "struqture_1_import")]
    pub fn from_struqture_1(
        value: &struqture_1::spins::SpinLindbladNoiseSystem,
    ) -> Result<Self, StruqtureError> {
        let mut new_operator = Self::new();
        for (key, val) in struqture_1::OperateOnDensityMatrix::iter(value) {
            let self_key_left = DecoherenceProduct::from_struqture_1(&key.0)?;
            let self_key_right = DecoherenceProduct::from_struqture_1(&key.1)?;
            let _ = new_operator.set((self_key_left, self_key_right), val.clone());
        }
        Ok(new_operator)
    }
}

/// Implements the negative sign function of PauliLindbladNoiseOperator.
///
impl ops::Neg for PauliLindbladNoiseOperator {
    type Output = PauliLindbladNoiseOperator;
    /// Implement minus sign for PauliLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self` - The PauliLindbladNoiseOperator * -1.
    fn neg(self) -> Self {
        let mut internal = IndexMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key.clone(), val.neg());
        }
        PauliLindbladNoiseOperator {
            internal_map: internal,
        }
    }
}

/// Implements the plus function of PauliLindbladNoiseOperator by PauliLindbladNoiseOperator.
///
impl<T, V> ops::Add<T> for PauliLindbladNoiseOperator
where
    T: IntoIterator<Item = ((DecoherenceProduct, DecoherenceProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `+` (add) for two PauliLindbladNoiseOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The PauliLindbladNoiseOperator to be added.
    ///
    /// # Returns
    ///
    /// * `Self` - The two PauliLindbladNoiseOperators added together.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn add(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value))
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the minus function of PauliLindbladNoiseOperator by PauliLindbladNoiseOperator.
///
impl<T, V> ops::Sub<T> for PauliLindbladNoiseOperator
where
    T: IntoIterator<Item = ((DecoherenceProduct, DecoherenceProduct), V)>,
    V: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implements `-` (subtract) for two PauliLindbladNoiseOperators.
    ///
    /// # Arguments
    ///
    /// * `other` - The PauliLindbladNoiseOperator to be subtracted.
    ///
    /// # Returns
    ///
    /// * `Self` - The two PauliLindbladNoiseOperators subtracted.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn sub(mut self, other: T) -> Self {
        for (key, value) in other.into_iter() {
            self.add_operator_product(key.clone(), Into::<CalculatorComplex>::into(value) * -1.0)
                .expect("Internal bug in add_operator_product");
        }
        self
    }
}

/// Implements the multiplication function of PauliLindbladNoiseOperator by CalculatorComplex/CalculatorFloat.
///
impl<T> ops::Mul<T> for PauliLindbladNoiseOperator
where
    T: Into<CalculatorComplex>,
{
    type Output = Self;
    /// Implement `*` for PauliLindbladNoiseOperator and CalculatorComplex/CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `other` - The CalculatorComplex or CalculatorFloat by which to multiply.
    ///
    /// # Returns
    ///
    /// * `Self` - The PauliLindbladNoiseOperator multiplied by the CalculatorComplex/CalculatorFloat.
    fn mul(self, other: T) -> Self {
        let other_cc = Into::<CalculatorComplex>::into(other);
        let mut internal = IndexMap::with_capacity(self.len());
        for (key, val) in self {
            internal.insert(key, val * other_cc.clone());
        }
        PauliLindbladNoiseOperator {
            internal_map: internal,
        }
    }
}

/// Implements the into_iter function (IntoIterator trait) of PauliLindbladNoiseOperator.
///
impl IntoIterator for PauliLindbladNoiseOperator {
    type Item = ((DecoherenceProduct, DecoherenceProduct), CalculatorComplex);
    type IntoIter =
        indexmap::map::IntoIter<(DecoherenceProduct, DecoherenceProduct), CalculatorComplex>;

    /// Returns the PauliLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The PauliLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.into_iter()
    }
}

/// Implements the into_iter function (IntoIterator trait) of reference PauliLindbladNoiseOperator.
///
impl<'a> IntoIterator for &'a PauliLindbladNoiseOperator {
    type Item = (
        &'a (DecoherenceProduct, DecoherenceProduct),
        &'a CalculatorComplex,
    );
    type IntoIter = Iter<'a, (DecoherenceProduct, DecoherenceProduct), CalculatorComplex>;

    /// Returns the reference PauliLindbladNoiseOperator in Iterator form.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The reference PauliLindbladNoiseOperator in Iterator form.
    fn into_iter(self) -> Self::IntoIter {
        self.internal_map.iter()
    }
}

/// Implements the from_iter function (FromIterator trait) of PauliLindbladNoiseOperator.
///
impl FromIterator<((DecoherenceProduct, DecoherenceProduct), CalculatorComplex)>
    for PauliLindbladNoiseOperator
{
    /// Returns the object in PauliLindbladNoiseOperator form, from an Iterator form of the object.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the information from which to create the PauliLindbladNoiseOperator.
    ///
    /// # Returns
    ///
    /// * `Self::IntoIter` - The iterator in PauliLindbladNoiseOperator form.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn from_iter<
        I: IntoIterator<Item = ((DecoherenceProduct, DecoherenceProduct), CalculatorComplex)>,
    >(
        iter: I,
    ) -> Self {
        let mut slno = PauliLindbladNoiseOperator::new();
        for (pair, cc) in iter {
            slno.add_operator_product(pair, cc)
                .expect("Internal bug in add_operator_product");
        }
        slno
    }
}

/// Implements the extend function (Extend trait) of PauliLindbladNoiseOperator.
///
impl Extend<((DecoherenceProduct, DecoherenceProduct), CalculatorComplex)>
    for PauliLindbladNoiseOperator
{
    /// Extends the PauliLindbladNoiseOperator by the specified operations (in Iterator form).
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator containing the operations by which to extend the PauliLindbladNoiseOperator.
    ///
    /// # Panics
    ///
    /// * Internal error in add_operator_product.
    fn extend<
        I: IntoIterator<Item = ((DecoherenceProduct, DecoherenceProduct), CalculatorComplex)>,
    >(
        &mut self,
        iter: I,
    ) {
        for (pair, cc) in iter {
            self.add_operator_product(pair, cc)
                .expect("Internal bug in add_operator_product");
        }
    }
}

/// Implements the format function (Display trait) of PauliLindbladNoiseOperator.
///
impl fmt::Display for PauliLindbladNoiseOperator {
    /// Formats the PauliLindbladNoiseOperator using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The formatted PauliLindbladNoiseOperator.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "PauliLindbladNoiseOperator{\n".to_string();
        for (key, val) in self.iter() {
            writeln!(output, "({}, {}): {},", key.0, key.1, val)?;
        }
        output.push('}');

        write!(f, "{output}")
    }
}

/// Add anti-commutator Lindblad contributions
fn add_anti_commutator(
    left: &DecoherenceProduct,
    right: &DecoherenceProduct,
    row: usize,
    dimension: usize,
    number_spins: usize,
    entries: &mut HashMap<usize, Complex64>,
    value: &CalculatorComplex,
) -> Result<(), StruqtureError> {
    let constant_prefactor = -0.5;
    let (right_conj, conjugate_prefactor) = right.hermitian_conjugate();
    let (product, product_prefactor) = DecoherenceProduct::multiply(right_conj, left.clone());
    for (row_adjusted, shift, (operator, transpose_prefactor)) in [
        (
            row.div_euclid(dimension),
            number_spins,
            (product.clone(), 1.0),
        ),
        (row % dimension, 0, product.hermitian_conjugate()),
    ] {
        let mut column = row;
        let mut prefac = Complex64::new(1.0, 0.0);
        // iterate over Lindblad terms
        for (spin_op_index, dec_op) in operator.iter() {
            match dec_op {
                SingleDecoherenceOperator::X => {
                    match row_adjusted.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                        0 => column += 2usize.pow((*spin_op_index + shift) as u32),
                        1 => column -= 2usize.pow((*spin_op_index + shift) as u32),
                        _ => panic!("Internal error in constructing matrix"),
                    }
                }
                SingleDecoherenceOperator::IY => {
                    match row_adjusted.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                        0 => {
                            column += 2usize.pow((*spin_op_index + shift) as u32);
                            // due to the transpose in i p H => i I.kron(H.T) only the Y Pauli operator picks up an extra
                            // sign equal to the commutato_prefactor
                            prefac *= 1.0;
                        }
                        1 => {
                            column -= 2usize.pow((*spin_op_index + shift) as u32);
                            prefac *= -1.0;
                        }
                        _ => panic!("Internal error in constructing matrix"),
                    };
                }
                SingleDecoherenceOperator::Z => {
                    match row_adjusted.div_euclid(2usize.pow(*spin_op_index as u32)) % 2 {
                        0 => {
                            prefac *= 1.0;
                        }
                        1 => {
                            prefac *= -1.0;
                        }
                        _ => panic!("Internal error in constructing matrix"),
                    };
                }
                SingleDecoherenceOperator::Identity => (),
            }
        }
        prefac *=
            transpose_prefactor * conjugate_prefactor * product_prefactor * constant_prefactor;
        let mut_value = entries.get_mut(&column);
        let value = Complex64 {
            re: *value.re.float()?,
            im: *value.im.float()?,
        };
        match mut_value {
            Some(x) => *x += value * prefac,
            None => {
                entries.insert(column, value * prefac);
            }
        }
    }
    Ok(())
}

/// Add Lindblad terms that are not part of the anti-commutator
fn add_lindblad_terms(
    left: &DecoherenceProduct,
    right: &DecoherenceProduct,
    row: usize,
    dimension: usize,
    number_spins: usize,
    entries: &mut HashMap<usize, Complex64>,
    value: &CalculatorComplex,
) -> Result<(), StruqtureError> {
    let mut column = row;
    let mut prefac = 1.0;
    // first the terms corresponding to -i H p => -i H.kron(I) flatten(p)
    for (index_operator_iter, shift, div_euclid) in
        [(left.iter(), number_spins, true), (right.iter(), 0, false)]
    {
        for (index, operator) in index_operator_iter {
            let row_adjusted = if div_euclid {
                row.div_euclid(dimension)
            } else {
                row % dimension
            };

            match operator {
                SingleDecoherenceOperator::X => {
                    match row_adjusted.div_euclid(2usize.pow(*index as u32)) % 2 {
                        0 => column += 2usize.pow((*index + shift) as u32),
                        1 => column -= 2usize.pow((*index + shift) as u32),
                        _ => panic!("Internal error in constructing matrix"),
                    }
                }
                SingleDecoherenceOperator::IY => {
                    match row_adjusted.div_euclid(2usize.pow(*index as u32)) % 2 {
                        0 => {
                            column += 2usize.pow((*index + shift) as u32);
                            // due to the transpose in i p H => i I.kron(H.T) only the Y Pauli operator picks up an extra
                            // sign equal to the commutator_prefactor
                            prefac *= 1.0;
                        }
                        1 => {
                            column -= 2usize.pow((*index + shift) as u32);
                            prefac *= -1.0;
                        }
                        _ => panic!("Internal error in constructing matrix"),
                    };
                }
                SingleDecoherenceOperator::Z => {
                    match row_adjusted.div_euclid(2usize.pow(*index as u32)) % 2 {
                        0 => {
                            prefac *= 1.0;
                        }
                        1 => {
                            prefac *= -1.0;
                        }
                        _ => panic!("Internal error in constructing matrix"),
                    };
                }
                SingleDecoherenceOperator::Identity => (),
            }
        }
    }
    let mut_value = entries.get_mut(&column);
    let value = Complex64 {
        re: *value.re.float()?,
        im: *value.im.float()?,
    };
    match mut_value {
        Some(x) => *x += value * prefac,
        None => {
            entries.insert(column, value * prefac);
        }
    }
    Ok(())
}

impl JordanWignerSpinToFermion for PauliLindbladNoiseOperator {
    type Output = FermionLindbladNoiseOperator;

    /// Implements JordanWignerSpinToFermion for a PauliLindbladNoiseOperator.
    ///
    /// The convention used is that |0> represents an empty fermionic state (spin-orbital),
    /// and |1> represents an occupied fermionic state.
    ///
    /// # Returns
    ///
    /// `FermionLindbladNoiseOperator` - The fermionic noise operator that results from the transformation.
    fn jordan_wigner(&self) -> Self::Output {
        let mut out = FermionLindbladNoiseOperator::new();

        for key in self.keys() {
            let fermion_operator_left = key.0.jordan_wigner();
            let fermion_operator_right = key.1.jordan_wigner();

            out.add_noise_from_full_operators(
                &fermion_operator_left,
                &fermion_operator_right,
                self.get(key).into(),
            )
            .expect("Internal bug in add_noise_from_full_operators");
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::STRUQTURE_VERSION;
    use serde_test::{assert_tokens, Configure, Token};

    // Test the Clone and PartialEq traits of PauliLindbladNoiseOperator
    #[test]
    fn so_from_sos() {
        let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
        let sos = PauliLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp.clone(), 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PauliLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: STRUQTURE_VERSION.to_string(),
            },
        };
        let mut so = PauliLindbladNoiseOperator::new();
        so.set((pp.clone(), pp), CalculatorComplex::from(0.5))
            .unwrap();

        assert_eq!(
            PauliLindbladNoiseOperator::try_from(sos.clone()).unwrap(),
            so
        );
        assert_eq!(PauliLindbladNoiseOperatorSerialize::from(so), sos);
    }
    // Test the Clone and PartialEq traits of PauliLindbladNoiseOperator
    #[test]
    fn clone_partial_eq() {
        let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
        let sos = PauliLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PauliLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        // Test Clone trait
        assert_eq!(sos.clone(), sos);

        // Test PartialEq trait
        let pp_1: DecoherenceProduct = DecoherenceProduct::new().z(0);
        let sos_1 = PauliLindbladNoiseOperatorSerialize {
            items: vec![(pp_1.clone(), pp_1, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PauliLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        let pp_2: DecoherenceProduct = DecoherenceProduct::new().z(2);
        let sos_2 = PauliLindbladNoiseOperatorSerialize {
            items: vec![(pp_2.clone(), pp_2, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PauliLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };
        assert!(sos_1 == sos);
        assert!(sos == sos_1);
        assert!(sos_2 != sos);
        assert!(sos != sos_2);
    }

    // Test the Debug trait of PauliLindbladNoiseOperator
    #[test]
    fn debug() {
        let pp: DecoherenceProduct = DecoherenceProduct::new().z(0);
        let sos = PauliLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PauliLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_eq!(
            format!("{sos:?}"),
            "PauliLindbladNoiseOperatorSerialize { items: [(DecoherenceProduct { items: [(0, Z)] }, DecoherenceProduct { items: [(0, Z)] }, Float(0.5), Float(0.0))], serialisation_meta: StruqtureSerialisationMeta { type_name: \"PauliLindbladNoiseOperator\", min_version: (2, 0, 0), version: \"2.0.0\" } }"
        );
    }

    /// Test PauliLindbladNoiseOperator Serialization and Deserialization traits (readable)
    #[test]
    fn serde_readable() {
        let pp = DecoherenceProduct::new().x(0);
        let sos = PauliLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PauliLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.readable(),
            &[
                Token::Struct {
                    name: "PauliLindbladNoiseOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 4 },
                Token::Str("0X"),
                Token::Str("0X"),
                Token::F64(0.5),
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("PauliLindbladNoiseOperator"),
                Token::Str("min_version"),
                Token::Tuple { len: 3 },
                Token::U64(2),
                Token::U64(0),
                Token::U64(0),
                Token::TupleEnd,
                Token::Str("version"),
                Token::Str("2.0.0"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    /// Test PauliLindbladNoiseOperator Serialization and Deserialization traits (compact)
    #[test]
    fn serde_compact() {
        let pp = DecoherenceProduct::new().x(0);
        let sos = PauliLindbladNoiseOperatorSerialize {
            items: vec![(pp.clone(), pp, 0.5.into(), 0.0.into())],
            serialisation_meta: crate::StruqtureSerialisationMeta {
                type_name: "PauliLindbladNoiseOperator".to_string(),
                min_version: (2, 0, 0),
                version: "2.0.0".to_string(),
            },
        };

        assert_tokens(
            &sos.compact(),
            &[
                Token::Struct {
                    name: "PauliLindbladNoiseOperatorSerialize",
                    len: 2,
                },
                Token::Str("items"),
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 4 },
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SingleDecoherenceOperator",
                    variant: "X",
                },
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Seq { len: Some(1) },
                Token::Tuple { len: 2 },
                Token::U64(0),
                Token::UnitVariant {
                    name: "SingleDecoherenceOperator",
                    variant: "X",
                },
                Token::TupleEnd,
                Token::SeqEnd,
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Float",
                },
                Token::F64(0.5),
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Float",
                },
                Token::F64(0.0),
                Token::TupleEnd,
                Token::SeqEnd,
                Token::Str("serialisation_meta"),
                Token::Struct {
                    name: "StruqtureSerialisationMeta",
                    len: 3,
                },
                Token::Str("type_name"),
                Token::Str("PauliLindbladNoiseOperator"),
                Token::Str("min_version"),
                Token::Tuple { len: 3 },
                Token::U64(2),
                Token::U64(0),
                Token::U64(0),
                Token::TupleEnd,
                Token::Str("version"),
                Token::Str("2.0.0"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
