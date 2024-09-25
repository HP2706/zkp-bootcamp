from py_ecc.bn128 import curve_order
import galois

#smart optimization for fast curve-point multiplication
GF = galois.GF(curve_order, primitive_element=5, verify=False)