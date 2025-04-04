# Spins

## Building blocks

All spin objects in struqture are expressed based on products of either Pauli operators (X, Y, Z) or operators suited to express decoherence (X, iY, Z). The products are built by setting the operators acting on separate spins.

### PauliProducts

PauliProducts are combinations of SinglePauliOperators on specific qubits. These are the `SinglePauliOperators`, or Pauli matrices, that are available for PauliProducts:

* I: identity matrix
\\[
I = \begin{pmatrix}
1 & 0\\\\
0 & 1
\end{pmatrix}
\\]

* X: Pauli x matrix
\\[
X = \begin{pmatrix}
0 & 1\\\\
1 & 0
\end{pmatrix}
\\]

* Y: Pauli y matrix
\\[
Y = \begin{pmatrix}
0 & -i\\\\
i & 0
\end{pmatrix}
\\]

* Z: Pauli z matrix
\\[
Z = \begin{pmatrix}
1 & 0\\\\
0 & -1
\end{pmatrix}
\\]

### DecoherenceProducts

DecoherenceProducts are products of a decoherence operators acting on single spins. These `SingleDecoherenceOperators`
are almost identical to the `SinglePauliOperators` with the exception of an additional \\(i\\) factor and are well suited to represent decoherence properties

* I: identity matrix
\\[
\begin{pmatrix}
1 & 0\\\\
0 & 1
\end{pmatrix}
\\]
* X: Pauli X matrix
\\[
\begin{pmatrix}
0 & 1\\\\
1 & 0
\end{pmatrix}
\\]
* iY: Pauli Y matrix multiplied by i
\\[
\begin{pmatrix}
0 & 1 \\\\
-1 & 0
\end{pmatrix}
\\]
* Z: Pauli z matrix
\\[
\begin{pmatrix}
1 & 0\\\\
0 & -1
\end{pmatrix}
\\]

### Examples

In Python the separate operators can be set via functions. In the python interface a PauliProduct can often be replaced by its unique string representation.

```python
from struqture_py.spins import PauliProduct, DecoherenceProduct

# A product of a X acting on spin 0, a Y acting on spin 3 and a Z acting on spin 20
pp = PauliProduct().x(0).y(3).z(20)
# Often equivalent the string representation
pp_string = str(pp)


# A product of a X acting on spin 0, a iY acting on spin 3 and a Z acting on spin 20
dp = DecoherenceProduct().x(0).iy(3).z(20)
# Often equivalent the string representation
dp_string = str(dp)
```

## Operators and Hamiltonians

A good example how complex objects are constructed from operator products are `PauliOperators` and `PauliHamiltonians`
(for more information, [see also](../container_types/operators_hamiltonians.md)).

These `PauliOperators` and `PauliHamiltonians` represent operators or Hamiltonians such as:
\\[
\hat{O} = \sum_{j} \alpha_j \prod_{k=0}^N \sigma_{j, k} \\\\
    \sigma_{j, k} \in \\{ X_k, Y_k, Z_k, I_k \\}
\\]
where the \\(\sigma_{j, k}\\) are `SinglePauliOperators`.

From a programming perspective the operators and Hamiltonians are HashMaps or Dictionaries with the `PauliProducts` as keys and the coefficients \\(\alpha_j\\) as values.

In struqture we distinguish between operators and Hamiltonians to avoid introducing unphysical behaviour by accident.
While both are sums over PauliProducts, Hamiltonians are guaranteed to be hermitian. In a spin Hamiltonian, this means that the prefactor of each `PauliProduct` has to be real.

### Examples

Here is an example of how to build a `PauliOperator` and a `PauliHamiltonian`:

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import spins

# Building the term sigma^x_0 * sigma^z_2: sigma_x acting on qubit 0
# and sigma_z acting on qubit 2
pp = spins.PauliProduct().x(0).z(2)

# O = (1 + 1.5 * i) * sigma^x_0 * sigma^z_2
operator = spins.PauliOperator()
operator.add_operator_product(pp, CalculatorComplex.from_pair(1.0, 1.5))
assert operator.get(pp) == complex(1.0, 1.5)
print(operator)

# Or when overwriting the previous value
operator.set(pp, 1.0)
print(operator)

# A complex extry is not valid for a PauliHamiltonian
hamiltonian = spins.PauliHamiltonian()
# This would fail
hamiltonian.add_operator_product(pp, CalculatorComplex.from_pair(1.0, 1.5))
# This is possible
hamiltonian.add_operator_product(pp, 1.0)
print(hamiltonian)
```

## Noise operators

We describe decoherence by representing it with the Lindblad equation.
The Lindblad equation is a master equation determining the time evolution of the density matrix.
It is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
with the rate matrix \\(\Gamma_{j,k}\\) and the Lindblad operator \\(L_{j}\\).

To describe spin noise we use the Lindblad equation with \\(\hat{H}=0\\).

Therefore, to describe the pure noise part of the Lindblad equation one needs the rate matrix in a well defined basis of Lindblad operators.
We use `DecoherenceProducts` as the operator basis.

The rate matrix and with it the Lindblad noise model is saved as a sum over pairs of `DecoherenceProducts`, giving the operators acting from the left and right on the density matrix.
In programming terms the object `PauliLindbladNoiseOperator` is given by a HashMap or Dictionary with the tuple (`DecoherenceProduct`, `DecoherenceProduct`) as keys and the entries in the rate matrix as values.

### Examples

Here, we add the terms \\( L_0 = \sigma_0^{x} \sigma_2^{z} \\) and \\( L_1 = \sigma_0^{x} \sigma_2^{z} \\) with coefficient 1.0: 
\\( 1.0 \left( L_0 \rho L_1^{\dagger} - \frac{1}{2} \\{ L_1^{\dagger} L_0, \rho \\} \right) \\)

```python
from struqture_py import spins

# Constructing the operator and product to be added to it
operator = spins.PauliLindbladNoiseOperator()
dp = spins.DecoherenceProduct().x(0).z(2)

# Adding in the 0X2Z term
operator.add_operator_product((dp, dp), 1.0+1.5*1j)
print(operator)

# In python we can also use the string representation
operator = spins.PauliLindbladNoiseOperator()
operator.add_operator_product(("0X2Z", "0X2Z"), 1.0+1.5*1j)
print(operator)
```

## Open systems

Physically open systems are quantum systems coupled to an environment that can often be described using Lindblad type of noise.
The Lindblad master equation is given by
\\[
    \dot{\rho} = \mathcal{L}(\rho) =-i \[\hat{H}, \rho\] + \sum_{j,k} \Gamma_{j,k} \left( L_{j}\rho L_{k}^{\dagger} - \frac{1}{2} \\{ L_k^{\dagger} L_j, \rho \\} \right)
\\]
In struqture they are composed of a hamiltonian (`PauliHamiltonian`) and noise (`PauliLindbladNoiseOperator`).

### Examples

```python
from qoqo_calculator_pyo3 import CalculatorComplex, CalculatorFloat
from struqture_py import spins

open_system = spins.PauliLindbladOpenSystem()

pp = spins.PauliProduct().z(1)
dp = spins.DecoherenceProduct().x(0).z(2)

# Add the Z_1 term into the system part of the open system
open_system.system_add_operator_product(pp, CalculatorFloat(2.0))
# Add the X_0 Z_2 term into the noise part of the open system
open_system.noise_add_operator_product(
    (dp, dp), CalculatorComplex.from_pair(0.0, 1.0))

print(open_system)
```

## Matrix representation: spin objects only

All spin-objects can be converted into sparse matrices with the following convention.
If \\(M_2\\) corresponds to the matrix acting on spin 2 and \\(M_1\\) corresponds to the matrix acting on spin 1 the total matrix \\(M\\) acting on spins 0 to 2 is given by
\\[
    M = M_2 \otimes M_1 \otimes \mathbb{1}
\\]
For an \\(N\\)-spin operator a term acts on the \\(2^N\\) dimensional space of state vectors.
A superoperator operates on the \\(4^N\\) dimensional space of flattened density-matrices.
struqture uses the convention that density matrices are flattened in row-major order
\\[
    \rho = \begin{pmatrix} a & b \\\\ c & d \end{pmatrix} => \vec{\rho} = \begin{pmatrix} a \\\\ b \\\\ c \\\\ d \end{pmatrix}
\\]
For noiseless objects (`PauliOperator`, `PauliHamiltonian`), sparse operators and sparse superoperators can be constructed, as we can represent the operator as a wavefunction. For operators with noise (`PauliLindbladNoiseOperator`, `PauliLindbladOpenSystem`), however, we can only represent them as density matrices and can therefore only construct sparse superoperators.

Note that the matrix representation functionality exists only for spin objects, and can't be generated for bosonic, fermionic or mixed system objects.

```python
from qoqo_calculator_pyo3 import CalculatorComplex
from struqture_py import spins
from scipy.sparse import coo_matrix

operator = spins.PauliLindbladNoiseOperator()

dp = spins.DecoherenceProduct().x(0).z(2)
operator.add_operator_product((dp, dp), CalculatorComplex.from_pair(1.0, 1.5))
# Using the `sparse_matrix_superoperator_coo` function, you can also
# return the information in scipy coo_matrix form, which can be directly fed in:
python_coo = coo_matrix(operator.sparse_matrix_superoperator_coo(dp.current_number_spins()))
print(python_coo.todense())
```
