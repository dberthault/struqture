# Copyright © 2019-2023 HQS Quantum Simulations GmbH. All Rights Reserved.

"""Struqture python interface

`HQS Quantum Simulations <https://quantumsimulations.de>`_ package for representing physical operators.

Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.

.. autosummary::
    :toctree: generated/

    spins
    bosons
    fermions
    mixed_systems
    openfermion_interface
"""
from .struqture_py import *  # type: ignore
from .spins import *  # type: ignore
from .fermions import *  # type: ignore
from .bosons import *  # type: ignore
from .mixed_systems import *  # type: ignore

try:
    from .openfermion_interface import *  # type: ignore
except ImportError:
    pass  # OpenFermion interface is optional and not available
# from .openfermion_interface import struqture_to_openfermion, openfermion_to_struqture  # type: ignore

__license__ = "Apache-2.0"
