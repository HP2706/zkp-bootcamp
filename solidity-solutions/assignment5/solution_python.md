```python

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
```