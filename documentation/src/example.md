# Applied example

In this example, we will create the qubit-boson Hamiltonian we have used for open-system research in our [paper](https://arxiv.org/abs/2210.12138), for 1 qubit and 3 bosonic modes.

The Hamiltonian is as follows:
\\[
    \hat{H} = \hat{H}_S + \hat{H}_B + \hat{H}_C
\\]

with the qubit (system) Hamiltonian \\(\hat{H}_S\\) :

\\[
    \hat{H} = \frac {\hbar \Delta} {2} \sigma^z_0,
\\]

the bosonic bath Hamiltonian \\(\hat{H}_B\\) :

\\[ 
    \hat{H} = \sum_{k=0}^2 \hbar \omega_k c_k^{\dagger} c_k,
\\]

and the coupling between system and bath \\(\hat{H}_C\\) :

\\[ 
    \hat{H} = \sigma_0^x \sum_{k=0}^2 \frac {v_k} {2} \left( c_k + c_k^{\dagger} \right)
\\]

For simplicity, we will set \\(\hbar\\) to 1.0 for this example.

Implementation:
```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py.bosons import BosonProduct
from struqture_py.mixed_systems import (
    HermitianMixedProduct, HermitianMixedProduct, MixedHamiltonian,
)
from struqture_py.spins import (PauliProduct, PauliProduct)


operator = MixedHamiltonian(1, 1, 0)

# Setting up constants:
delta = 1.0
omega_k = [2.0, 3.0, 4.0]
v_k = [5.0, 6.0, 7.0]

# First, H_S:
pp = PauliProduct().z(1)
hmp = HermitianMixedProduct([pp], [BosonProduct([], [])], [])
operator.add_operator_product(
    hmp, CalculatorComplex.from_pair(delta / 2.0, 0.0)
)

# Second, H_B:
for k in range(3):
    bp = BosonProduct([k], [k])
    hmp = HermitianMixedProduct([PauliProduct()], [bp], [])
    operator.add_operator_product(
        hmp, CalculatorComplex.from_pair(v_k[k] / 2.0, 0.0)
    )

# Third, H_C: the hermitian conjugate is implicitly stored,
# we don't need to add it manually
pp = PauliProduct().x(0)
for k in range(3):
    bp = BosonProduct([], [k])
    hmp = HermitianMixedProduct([pp], [bp], [])
    operator.add_operator_product(
        hmp, CalculatorComplex.from_pair(omega_k[k], 0.0)
    )


# Our resulting H:
print(operator)
```
