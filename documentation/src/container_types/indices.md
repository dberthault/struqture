# Products and Indices

The fundamental design of `struqture` uses products of quantum operators acting on single spins or modes to build up all represented objects. For spins those are `SinglePauliOperator` and `SingleDecoherenceOperator` and for Fermions and Bosons those are simply fermionic creation and annihilation operators.

**NOTE**: This section discusses technical aspects of the implementation and design choices of `struqture` products. For details of how to use these products, please use the [How to use struqture](../physical_types/intro.md) section.

Since these operators on single modes or spins form a complete basis of the operator space, each physical object that is represented in `struqture` can be built up from sum over products of these operators, be it an operator, a Hamiltonian or a noise description.

These sum objects can then be represented in a sparse fashion by saving the sum as a HashMap or Dictionary where the values are the prefactors of the operator products in the sum.
The keys of the HashMap are the operator products or for noise objects tuples of operator products.

One of the goals of `struqture` is to avoid introducing unphysical behaviour by encoding guarantees into the types of operators. For operator products that are not always Hermitian, `struqture` provides a Hermitian variant of the operator product. This variant picks by design one of the two hermitian conjugated versions of the operator product.
It can be used to uniquely represent the coefficient in sum objects that are themselves Hermitian (Hamiltonians) where the coefficients of Hermitian conjugated operator products in the sum also need to be Hermitian conjugated.


The operator products in `struqture` are

* `PauliProduct`
* `DecoherenceProduct`
* `FermionProduct`
* `HermitianFermionProduct`
* `BosonProduct`
* `HermitianBosonProdcut`
* `MixedProduct`
* `HermitianMixedProduct`
* `MixedDecoherenceProduct`

For examples showing how to use `PauliProducts` and `DecoherenceProducts`, please see the [the spins section](../physical_types/spins/products.md#examples).
For examples showing how to use `FermionProducts` and `HermitianFermionProducts`, please see the [the fermions section](../physical_types/fermions.md#examples).
For examples showing how to use `BosonProducts` and `HermitianBosonProducts`, please see the [the bosons section](../physical_types/bosons.md#examples).
For examples showing how to use `MixedProducts`, `HermitianMixedProducts` and `MixedDecoherenceProducts`, please see the [the mixed system section](../physical_types/mixed_systems.md#examples).
