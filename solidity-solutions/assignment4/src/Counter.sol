// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct ECPoint {
	uint256 x;
	uint256 y;
}

import {console2} from "forge-std/Test.sol";




/* const CURVE_ORDER_ECPY = 115792089237316195423570985008687907852837564279074904382605163141518161494337
const CURVE_ORDER = 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141; 
assert(CURVE_ORDER_ECPY == CURVE_ORDER); //these are the same
*/

contract ECMath {
    uint256 public immutable GENERATOR_POINT_X;
    uint256 public immutable GENERATOR_POINT_Y;
    uint256 public immutable CURVE_ORDER = 115792089237316195423570985008687907852837564279074904382605163141518161494337;

    constructor() {
        GENERATOR_POINT_X = 55066263022277343669578718895168534326250603453777594175500187360389116729240;
        GENERATOR_POINT_Y = 32670510020758816978083085130507043184471273380659243275938904335757337482424;
    }

    function createPoint(uint256 scalar) public view returns (ECPoint memory) {
        (uint256 x, uint256 y) = multiplyPoints(GENERATOR_POINT_X, GENERATOR_POINT_Y, scalar);
        return ECPoint(x, y);
    }

    function addPoints(uint256 x1, uint256 y1, uint256 x2, uint256 y2) public view returns (uint256, uint256) {
        uint256 x = addmod(x1, x2, CURVE_ORDER);
        uint256 y = addmod(y1, y2, CURVE_ORDER);
        return (x, y);
    }

    function multiplyPoints(uint256 x, uint256 y, uint256 s) public view returns (uint256, uint256) {
        uint256 mulmodX = mulmod(x, s, CURVE_ORDER);
        uint256 mulmodY = mulmod(y, s, CURVE_ORDER);
        return (mulmodX, mulmodY);
    }

    function rationalAdd(ECPoint calldata A, ECPoint calldata B, uint256 num, uint256 den) public view returns (bool verified) {
        (uint256 x, uint256 y) = addPoints(A.x, A.y, B.x, B.y);
        uint256 ratio = num / den; 
        (uint256 expectedX, uint256 expectedY) = multiplyPoints(GENERATOR_POINT_X, GENERATOR_POINT_Y, ratio);
        return (x == expectedX && y == expectedY);
    }

    function matmul(uint256[] calldata matrix,
        uint256 n, // n x n for the matrix
        ECPoint[] calldata s, // n elements
        uint256[] calldata o // n elements
    ) public returns (bool verified) {
	// revert if dimensions don't make sense or the matrices are empty
	// return true if Ms == o elementwise. You need to do n equality checks. If you're lazy, you can hardcode n to 3, but it is suggested that you do this with a for loop 


    ECPoint[] memory buffer = new ECPoint[](n);
    for (uint256 i = 0; i < n; i++) {
        ECPoint memory buf = ECPoint(0, 0);
        for (uint256 j = 0; j < n; j++) {
            uint256 scalar = matrix[i * n + j];
            (uint256 intermediate_x, uint256 intermediate_y) = ECMath.multiplyPoints(s[j].x, s[j].y, scalar);
            (buf.x, buf.y) = ECMath.addPoints(buf.x, buf.y, intermediate_x, intermediate_y);
        }
        buffer[i] = buf;
    }

    // check if buffer == o
    for (uint256 i = 0; i < n; i++) {
       bool res = (buffer[i].x == ECMath.createPoint(o[i]).x && buffer[i].y == ECMath.createPoint(o[i]).y);
       assert(res);
    }
    return true;

}

}