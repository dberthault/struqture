// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use num_complex::Complex64;
use pyo3::prelude::*;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use qoqo_calculator_pyo3::{CalculatorComplexWrapper, CalculatorFloatWrapper};
use struqture_py::spins::{
    PlusMinusLindbladNoiseOperatorWrapper, PlusMinusProductWrapper, SpinLindbladNoiseSystemWrapper,
};
use test_case::test_case;

// helper functions
fn new_noisesystem(py: Python) -> &PyCell<PlusMinusLindbladNoiseOperatorWrapper> {
    let system_type = py.get_type::<PlusMinusLindbladNoiseOperatorWrapper>();
    system_type
        .call0()
        .unwrap()
        .downcast::<PyCell<PlusMinusLindbladNoiseOperatorWrapper>>()
        .unwrap()
}

// helper function to convert CalculatorFloat into a python object
fn convert_cf_to_pyobject(
    py: Python,
    parameter: CalculatorFloat,
) -> &PyCell<CalculatorFloatWrapper> {
    let parameter_type = py.get_type::<CalculatorFloatWrapper>();
    match parameter {
        CalculatorFloat::Float(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
        CalculatorFloat::Str(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
    }
}

/// Test default function of PlusMinusLindbladNoiseOperatorWrapper
#[test]
fn test_default_partialeq_debug_clone() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_system = new_noisesystem(py);
        new_system
            .call_method1("add_operator_product", (("0+", "0+"), 0.1))
            .unwrap();
        let system_wrapper = new_system
            .extract::<PlusMinusLindbladNoiseOperatorWrapper>()
            .unwrap();

        // PartialEq
        let helper_ne: bool = PlusMinusLindbladNoiseOperatorWrapper::new() != system_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = PlusMinusLindbladNoiseOperatorWrapper::default()
            == PlusMinusLindbladNoiseOperatorWrapper::new();
        assert!(helper_eq);

        // Clone
        assert_eq!(system_wrapper.clone(), system_wrapper);

        // Debug
        assert_eq!(
            format!("{:?}", PlusMinusLindbladNoiseOperatorWrapper::new()),
            "PlusMinusLindbladNoiseOperatorWrapper { internal: PlusMinusLindbladNoiseOperator { internal_map: {} } }"
        );
    })
}

/// Test empty_clone function of SpinSystem
#[test]
fn test_empty_clone() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let number_spins: Option<usize> = None;
        let system = new_noisesystem(py);
        let none_system = system.call_method1("empty_clone", (number_spins,)).unwrap();
        let comparison =
            bool::extract(none_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);

        let number_spins: Option<usize> = Some(3);
        let system = new_noisesystem(py);
        let some_system = system.call_method1("empty_clone", (number_spins,)).unwrap();
        let comparison =
            bool::extract(some_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add_operator_product and remove functions of SpinSystem
#[test]
fn spin_system_test_add_operator_product_remove() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1("add_operator_product", (("0+", "0+"), 0.1))
            .unwrap();
        system
            .call_method1("add_operator_product", (("0+", "1Z"), 0.2))
            .unwrap();
        system
            .call_method1("add_operator_product", (("0+", "3-"), 0.05))
            .unwrap();

        // test access at index 0
        let comp_op = system.call_method1("get", (("0+", "0+"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.1,)).unwrap()).unwrap();
        assert!(comparison);
        system.call_method1("remove", (("0+", "0+"),)).unwrap();
        let comp_op = system.call_method1("get", (("0+", "0+"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 1
        let comp_op = system.call_method1("get", (("0+", "1Z"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.2,)).unwrap()).unwrap();
        assert!(comparison);
        // test access at index 3
        let comp_op = system.call_method1("get", (("0+", "3-"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.05,)).unwrap()).unwrap();
        assert!(comparison);

        // Get zero
        let comp_op = system.call_method1("get", (("0+", "2+"),)).unwrap();
        let comparison = bool::extract(comp_op.call_method1("__eq__", (0.0,)).unwrap()).unwrap();
        assert!(comparison);

        // Get error
        let error = system.call_method1("get", (("2J", "0+"),));
        assert!(error.is_err());

        // Try_set error 1: Key (PauliProduct) cannot be converted from string
        let error = system.call_method1("add_operator_product", (("1J", "0+"), 0.5));
        assert!(error.is_err());

        // Try_set error 2: Value cannot be converted to CalculatorComplex
        let error = system.call_method1("add_operator_product", (("0+", "1Z"), vec![0.0]));
        assert!(error.is_err());

        // Try_set error 4: Generic error
        let error = system.call_method1("add_operator_product", (("0+", "1J"), 0.5));
        assert!(error.is_err());
    });
}

/// Test keys function of SpinSystem
#[test]
fn test_keys_values() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract(len_system.call_method1("__eq__", (0_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract(system.call_method0("is_empty").unwrap()).unwrap();
        assert!(empty_system);

        system
            .call_method1("add_operator_product", (("0+", "0+"), 0.1))
            .unwrap();

        let keys_system = system.call_method0("keys").unwrap();
        let comparison = bool::extract(
            keys_system
                .call_method1("__eq__", (vec![("0+", "0+")],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let values_system = system.call_method0("values").unwrap();
        let comparison =
            bool::extract(values_system.call_method1("__eq__", (vec![0.1],)).unwrap()).unwrap();
        assert!(comparison);

        let len_system = system.call_method0("__len__").unwrap();
        let comparison =
            bool::extract(len_system.call_method1("__eq__", (1_u64,)).unwrap()).unwrap();
        assert!(comparison);
        let empty_system = bool::extract(system.call_method0("is_empty").unwrap()).unwrap();
        assert!(!empty_system);
    });
}

#[test_case(1.0,0.0;"real")]
#[test_case(0.0,1.0;"imag")]
#[test_case(0.7,0.7;"mixed")]
fn test_truncate(re: f64, im: f64) {
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "1-"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(10.0 * re, 10.0 * im),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "2Z"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(re, im),
                    },
                ),
            )
            .unwrap();
        system
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+1Z"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system1 = new_noisesystem(py);
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "1-"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(10.0 * re, 10.0 * im),
                    },
                ),
            )
            .unwrap();
        test_system1
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+1Z"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let test_system2 = new_noisesystem(py);
        test_system2
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::new(100.0 * re, 100.0 * im),
                    },
                ),
            )
            .unwrap();
        test_system2
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+1Z"),
                    CalculatorComplexWrapper {
                        internal: CalculatorComplex::from("test"),
                    },
                ),
            )
            .unwrap();

        let comparison_system1 = system.call_method1("truncate", (5.0_f64,)).unwrap();
        let comparison = bool::extract(
            comparison_system1
                .call_method1("__eq__", (test_system1,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison_system2 = system.call_method1("truncate", (50.0_f64,)).unwrap();
        let comparison = bool::extract(
            comparison_system2
                .call_method1("__eq__", (test_system2,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_neg() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("0+", "0+"), 0.1))
            .unwrap();
        let system_1 = new_noisesystem(py);
        system_1
            .call_method1("add_operator_product", (("0+", "0+"), -0.1))
            .unwrap();

        let negated = system_0.call_method0("__neg__").unwrap();
        let comparison =
            bool::extract(negated.call_method1("__eq__", (system_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_add() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("0+", "0+"), 0.1))
            .unwrap();
        let system_1 = new_noisesystem(py);
        system_1
            .call_method1("add_operator_product", (("1Z", "0+"), 0.2))
            .unwrap();
        let system_0_1 = new_noisesystem(py);
        system_0_1
            .call_method1("add_operator_product", (("0+", "0+"), 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", (("1Z", "0+"), 0.2))
            .unwrap();

        let added = system_0.call_method1("__add__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_sub() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("0+", "0+"), 0.1))
            .unwrap();
        let system_1 = new_noisesystem(py);
        system_1
            .call_method1("add_operator_product", (("1Z", "0+"), 0.2))
            .unwrap();
        let system_0_1 = new_noisesystem(py);
        system_0_1
            .call_method1("add_operator_product", (("0+", "0+"), 0.1))
            .unwrap();
        system_0_1
            .call_method1("add_operator_product", (("1Z", "0+"), -0.2))
            .unwrap();

        let added = system_0.call_method1("__sub__", (system_1,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_mul_cf() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("0+", "0+"), 0.1_f64))
            .unwrap();

        let system_0_1 = new_noisesystem(py);
        system_0_1
            .call_method1("add_operator_product", (("0+", "0+"), 0.2))
            .unwrap();

        let added = system_0.call_method1("__mul__", (2.0,)).unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test add magic method function of SpinSystem
#[test]
fn test_mul_cc() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_0 = new_noisesystem(py);
        system_0
            .call_method1("add_operator_product", (("0+", "0+"), 0.1_f64))
            .unwrap();

        let system_0_1 = new_noisesystem(py);
        system_0_1
            .call_method1(
                "add_operator_product",
                (("0+", "0+"), Complex64::new(0.0, 0.5)),
            )
            .unwrap();

        let added = system_0
            .call_method1(
                "__mul__",
                (CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(0.0, 5.0),
                },),
            )
            .unwrap();
        let comparison =
            bool::extract(added.call_method1("__eq__", (system_0_1,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test copy and deepcopy functions of PlusMinusLindbladNoiseOperator
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let copy_system = system.call_method0("__copy__").unwrap();
        let deepcopy_system = system.call_method1("__deepcopy__", ("",)).unwrap();

        let comparison_copy =
            bool::extract(copy_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy =
            bool::extract(deepcopy_system.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison_deepcopy);
    });
}

/// Test to_bincode and from_bincode functions of PlusMinusLindbladNoiseOperator
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let serialised = system.call_method0("to_bincode").unwrap();
        let new = new_noisesystem(py);
        let deserialised = new.call_method1("from_bincode", (serialised,)).unwrap();

        let deserialised_error =
            new.call_method1("from_bincode", (bincode::serialize("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new.call_method1("from_bincode", (bincode::serialize(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let comparison =
            bool::extract(deserialised.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

#[test]
fn test_value_error_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new = new_noisesystem(py);
        let deserialised_error = new.call_method1("from_bincode", ("J",));
        assert!(deserialised_error.is_err());
    });
}

/// Test to_ and from_json functions of PlusMinusLindbladNoiseOperator
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let serialised = system.call_method0("to_json").unwrap();
        let new = new_noisesystem(py);
        let deserialised = new.call_method1("from_json", (serialised,)).unwrap();

        let deserialised_error =
            new.call_method1("from_json", (serde_json::to_string("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new.call_method1("from_json", (serde_json::to_string(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let comparison =
            bool::extract(deserialised.call_method1("__eq__", (system,)).unwrap()).unwrap();
        assert!(comparison)
    });
}

/// Test the __repr__ and __format__ functions
#[test]
fn test_format_repr() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system = new_noisesystem(py);
        system
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "0+"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let mut rust_system = PlusMinusLindbladNoiseOperatorWrapper::new();
        let pp_type = py.get_type::<PlusMinusProductWrapper>();
        let new_pp = pp_type
            .call0()
            .unwrap()
            .downcast::<PyCell<PlusMinusProductWrapper>>()
            .unwrap();
        let pp = new_pp.call_method1("set_pauli", (1_u64, "Z")).unwrap();

        rust_system
            .add_operator_product(
                (pp.into(), pp.into()),
                convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
            )
            .unwrap();

        let to_format = system.call_method1("__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format).unwrap();

        let to_repr = system.call_method0("__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr).unwrap();

        let to_str = system.call_method0("__str__").unwrap();
        let str_op: &str = <&str>::extract(to_str).unwrap();

        assert_eq!(
            format_op,
            "PlusMinusLindbladNoiseOperator{\n(0+, 0+): (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            repr_op,
            "PlusMinusLindbladNoiseOperator{\n(0+, 0+): (1e-1 + i * 0e0),\n}".to_string()
        );
        assert_eq!(
            str_op,
            "PlusMinusLindbladNoiseOperator{\n(0+, 0+): (1e-1 + i * 0e0),\n}".to_string()
        );
    });
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let system_one = new_noisesystem(py);
        system_one
            .call_method1(
                "add_operator_product",
                (
                    ("1+", "1Z"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();
        let system_two = new_noisesystem(py);
        system_two
            .call_method1(
                "add_operator_product",
                (
                    ("0+", "1Z"),
                    convert_cf_to_pyobject(py, CalculatorFloat::from(0.1)),
                ),
            )
            .unwrap();

        let comparison =
            bool::extract(system_one.call_method1("__eq__", (system_two,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison =
            bool::extract(system_one.call_method1("__eq__", ("0+",)).unwrap()).unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract(system_one.call_method1("__ne__", (system_two,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison =
            bool::extract(system_one.call_method1("__ne__", ("0+",)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = system_one.call_method1("__ge__", ("0+",));
        assert!(comparison.is_err());
    });
}

#[test]
fn test_separate() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pmp = new_noisesystem(py);
        pmp.call_method1("add_operator_product", (("0Z", "0Z"), 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", (("0Z1Z", "0Z"), 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", (("0Z1Z", "0Z1Z"), 1.0))
            .unwrap();
        pmp.call_method1("add_operator_product", (("0Z1+", "0Z1Z"), 1.0))
            .unwrap();

        let pmp_rem = new_noisesystem(py);
        pmp_rem
            .call_method1("add_operator_product", (("0Z", "0Z"), 1.0))
            .unwrap();
        pmp_rem
            .call_method1("add_operator_product", (("0Z1Z", "0Z"), 1.0))
            .unwrap();

        let pmp_sys = new_noisesystem(py);
        pmp_sys
            .call_method1("add_operator_product", (("0Z1Z", "0Z1Z"), 1.0))
            .unwrap();
        pmp_sys
            .call_method1("add_operator_product", (("0Z1+", "0Z1Z"), 1.0))
            .unwrap();

        let result = pmp.call_method1("separate_into_n_terms", (2, 2)).unwrap();
        let equal = bool::extract(
            result
                .call_method1("__eq__", ((pmp_sys, pmp_rem),))
                .unwrap(),
        )
        .unwrap();
        assert!(equal);
    })
}

#[test]
fn test_from_spin_sys() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pmp = new_noisesystem(py);
        pmp.call_method1(
            "add_operator_product",
            (
                ("0+", "0+"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(1.0, 0.0),
                },
            ),
        )
        .unwrap();
        pmp.call_method1(
            "add_operator_product",
            (
                ("0-", "0+"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(-1.0, 0.0),
                },
            ),
        )
        .unwrap();
        pmp.call_method1(
            "add_operator_product",
            (
                ("0+", "0-"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(-1.0, 0.0),
                },
            ),
        )
        .unwrap();
        pmp.call_method1(
            "add_operator_product",
            (
                ("0-", "0-"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(1.0, 0.0),
                },
            ),
        )
        .unwrap();

        let number_spins: Option<usize> = Some(1);
        let pp_type = py.get_type::<SpinLindbladNoiseSystemWrapper>();
        let pp = pp_type
            .call1((number_spins,))
            .unwrap()
            .downcast::<PyCell<SpinLindbladNoiseSystemWrapper>>()
            .unwrap();
        pp.call_method1(
            "add_operator_product",
            (
                ("0iY", "0iY"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(1.0, 0.0),
                },
            ),
        )
        .unwrap();

        let result = py
            .get_type::<PlusMinusLindbladNoiseOperatorWrapper>()
            .call_method1("from_spin_noise_system", (pp,))
            .unwrap();
        let equal = bool::extract(result.call_method1("__eq__", (pmp,)).unwrap()).unwrap();
        assert!(equal);

        let result = py
            .get_type::<PlusMinusLindbladNoiseOperatorWrapper>()
            .call_method1("from_spin_noise_system", ("No",));
        assert!(result.is_err())
    })
}

#[test]
fn test_to_spin_sys() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pmp = new_noisesystem(py);
        pmp.call_method1(
            "add_operator_product",
            (
                ("0+", "0+"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(1.0, 0.0),
                },
            ),
        )
        .unwrap();

        let number_spins: Option<usize> = Some(1);
        let pp_type = py.get_type::<SpinLindbladNoiseSystemWrapper>();
        let sys = pp_type
            .call1((number_spins,))
            .unwrap()
            .downcast::<PyCell<SpinLindbladNoiseSystemWrapper>>()
            .unwrap();
        sys.call_method1(
            "add_operator_product",
            (
                ("0X", "0X"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(0.25, 0.0),
                },
            ),
        )
        .unwrap();
        sys.call_method1(
            "add_operator_product",
            (
                ("0X", "0iY"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(0.25, 0.0),
                },
            ),
        )
        .unwrap();
        sys.call_method1(
            "add_operator_product",
            (
                ("0iY", "0X"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(0.25, 0.0),
                },
            ),
        )
        .unwrap();
        sys.call_method1(
            "add_operator_product",
            (
                ("0iY", "0iY"),
                CalculatorComplexWrapper {
                    internal: CalculatorComplex::new(0.25, 0.0),
                },
            ),
        )
        .unwrap();

        let result = pmp
            .call_method1("to_spin_noise_system", (number_spins,))
            .unwrap();
        println!("{:?}", result);
        let equal = bool::extract(result.call_method1("__eq__", (sys,)).unwrap()).unwrap();
        assert!(equal);
    })
}