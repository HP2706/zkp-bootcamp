from py_ecc.bn128 import (
    G1, 
    G2, 
    multiply, 
    add, 
    pairing, 
    bn128_curve,
    eq,
    neg
)
from dataclasses import dataclass
import numpy as np


from py_ecc.bn128 import (
    G1, 
    G2, 
    multiply, 
    add, 
    pairing, 
    bn128_curve,
    eq,
    neg
)
from dataclasses import dataclass
import numpy as np


def verifier(
    A1 : int, 
    B2 : int, 
    C1 : int, 
    x1 : int, 
    x2 : int, 
    x3 : int
) -> bool:    
    '''
    Checks that:
    0 = pairing(B2, neg(A1)) + pairing(beta_2, alpha_1) + pairing(gamma_2, X1) + pairing(delta_2, C1)
    '''
    _alpha_1 = 1
    _beta_2 = 1
    _gamma_2 = 1
    _delta_2 = 1
    
    alpha_1 = multiply(G1, _alpha_1)
    beta_2 = multiply(G2, _beta_2)
    gamma_2 = multiply(G2, _gamma_2)
    delta_2 = multiply(G2, _delta_2)
    
    _X1 = (x1 + x2 + x3) % bn128_curve.curve_order
    
   
    X1 = multiply(G1, _X1)
    return eq(
        pairing(B2, A1), 
        pairing(beta_2, alpha_1) * pairing(gamma_2, X1) * pairing(delta_2, C1)
    )

def test_discrete_log_sum(
    _A1 : int,
    _B2 : int,
    _C1 : int,
    x1 : int,
    x2 : int,
    x3 : int,
    _beta_2 : int = 1,
    _alpha_1 : int = 1,
    _gamma_2 : int = 1,
    _delta_2 : int = 1,
)-> bool:
    _X1 = (x1 + x2 + x3) % bn128_curve.curve_order
    left_side = (_B2 * (_A1)) % bn128_curve.curve_order
    right_side = ((_beta_2 * _alpha_1) + (_gamma_2 * _X1) + (_delta_2 * _C1)) % bn128_curve.curve_order
    return left_side == right_side

a1 = 2
b2 = 1
c1 = 2
x1 = 1
x2 = 1
x3 = -3

output = verifier(
    multiply(G1, a1),
    multiply(G2, b2),
    multiply(G1, c1),
    x1,
    x2,
    x3
)

print(output)

print(test_discrete_log_sum(
    a1,
    b2,
    c1,
    x1,
    x2,
    x3
))