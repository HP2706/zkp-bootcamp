from ecpy.curves import Curve, Point
from dataclasses import dataclass
import numpy as np

curve = Curve.get_curve('secp256k1')



#function rationalAdd(ECPoint calldata A, ECPoint calldata B, uint256 num, uint256 den) public view returns (bool verified) 
def rationalAdd(A: Point, B: Point, num: int, den: int) -> bool:
    '''
    Claim: “I know two rational numbers that add up to num/den”
    Proof: ([A], [B], num, den)
    #pow(a, -1, curve_order) == pow(a, curve_order - 2, curve_order)
    '''
    assert num % den == 0
    assert A.curve == B.curve
    sum : Point = A.add(B)
    num_dem_point = A.curve.generator.mul(int(num / den))   
    
    assert num_dem_point.x == sum.x
    assert num_dem_point.y == sum.y
    
def verify_rational_add():
    num = 10 
    den = 2
    A = curve.generator * 3  # Point A (3 times the generator point)
    B = curve.generator * 2  # Point B (2 times the generator point)
    rationalAdd(A, B, num, den)

def matmul(matrix: list[int], n: int, s: list[Point], o: list[int]) -> bool:
    """ function matmul(uint256[] calldata matrix,
                    uint256 n, // n x n for the matrix
                    ECPoint[] calldata s, // n elements
                    uint256[] calldata o // n elements
                ) public returns (bool verified) {

        // revert if dimensions don't make sense or the matrices are empty

        // return true if Ms == o elementwise. You need to do n equality checks. If you're lazy, you can hardcode n to 3, but it is suggested that you do this with a for loop 
    }
    """
    assert len(matrix) == n * n
    assert len(s) == n
    assert len(o) == n
    
    buffer = [None] * n 
    for i in range(n):
        buf = None
        for j in range(n):
            intermediate = s[j].mul(matrix[i * n + j])
            if buf is None:
                buf = intermediate
            else:
                buf = buf.add(intermediate)
        buffer[i] = buf

    for a, b in zip(buffer, o):
        assert a.x == b.x, f"got {a.x}, expected {b.x}"
        assert a.y == b.y, f"got {a.y}, expected {b.y}"
    return True

def test_matmul():
    n = 10
    matrix = np.random.randint(0, 100, size=(n, n))
    s_int = np.random.randint(0, 100, size=(n,1))
    s_points = [curve.generator.mul(int(i)) for i in s_int.reshape(-1)]
    o_int = matrix @ s_int
    o_points = [curve.generator.mul(int(i)) for i in o_int.reshape(-1)]
    
    assert matmul(matrix.reshape(-1).tolist(), n, s_points, o_points), "Matrix multiplication verification failed"
    

def test_solution():
    verify_rational_add()
    test_matmul()
    
test_solution()

a = np.array([i for i in range(1, 10)]).reshape(3, 3)
b = np.array([1, 2, 3])
print(a @ b)