from ecpy.curves import Curve, Point
from ecpy.keys import ECPublicKey, ECPrivateKey
from eth_hash.auto import keccak
import secrets # for random number generation

private_key = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
cv = Curve.get_curve('secp256k1')
pv_key = ECPrivateKey(private_key, cv)
pu_key = pv_key.get_public_key()

m = b'hello world' #the message to sign
hash_m = keccak(m)
# convert to int
hash_m_int = int.from_bytes(hash_m, 'big')

def sign_m(h : int, pv_key : ECPrivateKey) -> int:
    '''
    h : int , the hash of the message
    pv_key : the private key

    Returns:
    (r,s,h,public_key) : the signature
    r : the x coordinate of the curve point
    s : the signature
    h : the hash of the message
    public_key : the public key
    '''


    # The algorithm:
    # G : generator point
    # k : randomly generated nonce
    # curve_point = cv.mul_point(k, G)
    # x_1, y_1 = (curve_point.x, curve_point.y)
    # r = x_1 mod n
    # s = k^-1 * (h+r*pv_key) % n
    k = secrets.randbelow(cv.order)
    curve_point = k*cv.generator
    n = cv.order
    curve_point : Point = cv.mul_point(k, cv.generator)
    x_1 = curve_point.x
    r = x_1 % n
    mod_k_inv = pow(k, -1, n)
    s = (mod_k_inv * (h + r * pv_key.d)) % n
    public_key = pv_key.get_public_key()
    return (r,s,h,public_key)

def verify(r : int, s : int, h : int, public_key : ECPublicKey) -> bool:
    '''
    r : int, the x coordinate of the curve point (R) in tutorial 
    s : int, the signature
    h : int, the hash of the message
    public_key : the public key
    returns:
        bool, True if the signature is valid, False otherwise
    '''
    
    # outline of the algorithm:
    # 1. check public key is not identity element 
    # 2. compute s1 = s^-1 mod n
    # 3. recover the curve point from the signature and the public key
    # 4. check if the x coordinate of curve_point is r

    # 1. check public key is not identity element 
    assert public_key.W != cv.infinity, "Public key is the identity element"
    n = cv.order
    s1 = pow(s, -1, n)
    #recovered curve point
    curve_point = cv.add_point(cv.mul_point(h * s1, cv.generator), cv.mul_point(r * s1, public_key.W))
    r_prime = curve_point.x 
    return r_prime == r



out = sign_m(hash_m_int+2, pv_key)
print(verify(*out))