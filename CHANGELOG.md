# Changelog

This changelog track changes to the struqture project starting at version v1.0.0

## Unreleased

## 2.0.0-alpha.14

* Changed the `cmp` method of `PauliProduct` to use the size of the product, then the qubit index and then the Pauli terms.

## 2.0.0-alpha.13

* Added matrix information to Migration Guide.
* Made the `number_spins` field in the `sparse_matrix` methods not optional.

## 2.0.0-alpha.12

* Renamed `to_qubit`/`from_qubit` functions to `to_pauli`/`from_pauli`.

## 2.0.0-alpha.11

* Updated the struqture_1 dependency.

## 2.0.0-alpha.10

* Updated to qoqo-calculator 1.5.0.
* Updated to pyo3 0.23.

## 2.0.0-alpha.9

* Updated dependencies.
* Removed `sparse_lindblad_entries` and `unitary_sparse_matrix_coo` functions.
* Removed `from_struqture_1` and `from_pyany_to_struqture_1` functions from the python interface.
* Renamed all `Qubit` objects to `Pauli`.
* Renamed `to_mixed_system` and `from_mixed_system` to `to_mixed_operator` and `from_mixed_operator`.
* Added qoqo/.cargo/config file with aarch64 and x86_64 targets for macos.

## 2.0.0-alpha.8

* Added the `separate_into_n_terms` function into the FermionHamiltonian methods.

## 2.0.0-alpha.6 - 2.0.0-alpha.7

* Updated to qoqo-calculator 1.4.

## 2.0.0-alpha.5

* Added links to examples in container types of the user documentation
* Updated dependencies: jsonschema (0.18 -> 0.28), ndarray (0.15 -> 0.16), thiserror (1.0 -> 2.0), itertools (0.13 -> 0.14), qoqo-calculator (1.2 -> 1.3).
* Updated minimum supported Rust version from 1.57 to 1.76.
* Updated minimum supported Python version from 3.8 to 3.9.
* Fixed a bug when creating a Product from a bad JSON.

## 2.0.0-alpha.4

* Updated to pyo3 0.22 and python 3.13.

## 2.0.0-alpha.1 - 2.0.0-alpha.3

* Additional changes from feedback regarding struqture 2.0.

## 2.0.0-alpha.0

* First draft of the changes for struqture 2.0.

## 1.11.1

* Updated to struqture 2.0.0-alpha.7.

## 1.11.0

* Updated dependencies: jsonschema (0.18 -> 0.28), ndarray (0.15 -> 0.16), thiserror (1.0 -> 2.0), itertools (0.13 -> 0.14).
* Updated minimum supported Rust version from 1.57 to 1.76.
* Updated minimum supported Python version from 3.8 to 3.9.

## 1.10.1

* Fixed a build issue in 1.10.0.

## 1.10.0

* Updated to pyo3 0.22 and python 3.13.

## 1.9.2

* Fixed a bug when creating a Product from a bad JSON.

## 1.9.0 - 1.9.1

* Added methods to convert from struqture 2.0.0-alpha.3

## 1.8.0

* Added IDE hint support.

## 1.7.1

* Fixed versioning bug.

## 1.7.0

* Updated to pyo3 0.21.

## 1.6.2

* Updated VersionMissmatch error message.

## 1.6.1

* Updated Cargo.lock (particularly mio 0.8.10->0.8.11).

## 1.6.0

* Add optional feature `indexed_map_iterators` switching internal HashMaps to `indexmap` implementation. Using this feature will change the type of iterators returned by `keys`, `values` and `iter` methods.
* Switching Python interface to using `indexed_map_iterators` by default. This emulates the usual Python behavior of returning the elements of dictionary-like objects in the order of insertion.

## 1.5.2

* Updated to pyo3 0.20.

## 1.5.1

* Removed print statement from __init__.py file.

## 1.5.0

* Added remap_modes function to fermionic and bosonic indices for the pyo3 interface.

## 1.4.1

* Added remap_modes function to fermionic and bosonic indices in pure Rust.

## 1.4.0

* Fixed bug in Jordan-Wigner transformation for FermionHamiltonian and FermionHamiltonian.
* Added MixedPlusMinusProduct, MixedPlusMinusOperator to mod.rs in struqture-py/src/mixed_systems (fixed import error).
* Added conversion from SpinHamiltonian(System) to PlusMinusOperator.
* Added support for jsonschema in struqture and struqture-py.

## 1.3.1

* Fixed bug allowing the construction of Hermitian operator products with annihilator index lower than creator index when there are leading equal indices.
* Updated pyo3 dependency to 0.19.

## 1.3.0

* Added Jordan-Wigner transform to both struqture and struqture-py.

## 1.2.0

* Added MixedPlusMinusProduct and MixedPlusMinusOperator to both struqture and struqture-py.

## 1.1.1

* Fixed failing group when system and noise have the same number of current spins or modes put one of them has not fixed number of spins/modes.

## 1.1.0

* Added support for sigma +, sigma - and sigma z spin basis.

## 1.0.1

* Updated to pyo3 0.18 and test-case 3.0.

## 1.0.0

* Added `noise_get` and `system_get` getters for all OpenSystems in python interface.
* Added a number of particles check to MixedHamiltonian, MixedSystem and MixedLindbladNoiseSystem.
