# Interface to Qutip

Struqture has an interface to the [QuTiP](https://qutip.org/) package that can be used to transform struqture spin objects to qutip objects for simulation purposes. 

It is a separate package from struqture that can be installed with:

```bash
pip install struqture-qutip-interface
```

More informations can be found on the [struqture-qutip-interface github page](https://github.com/HQSquantumsimulations/struqture-qutip-interface/tree/main)

# Interface to OpenFermion

Struqture also has an interface to the [OpenFermion](https://quantumai.google/openfermion) package allowing users to switch from one package to the other.

For now only the conversion to and from `PauliHamiltonian` is implemented with the functions `struqture_to_openfermion` and `openfermion_to_struqture`.

