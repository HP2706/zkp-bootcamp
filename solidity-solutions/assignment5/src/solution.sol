// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import {Test, console2} from "forge-std/Test.sol";

/// taken from
/// https://github.com/tornadocash/tornado-core/blob/master/contracts/Verifier.sol#L79
library Pairing {
    uint256 constant PRIME_Q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;
    uint256 constant CurveOrder = 21888242871839275222246405745257275088548364400416034343698204186575808495617;

    struct G1Point {
        uint256 X;
        uint256 Y;
    }

    // Encoding of field elements is: X[0] * z + X[1]
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }

    function getG1() public pure returns (G1Point memory) {
        return G1Point(1, 2);
    }

    function getG2() public pure returns (G2Point memory) {
    // https://github.com/ethereum/py_pairing/blob/master/py_ecc/bn128/bn128_curve.py
    // G2 = (
    //    FQ2(
    //        [
    //          10857046999023057135944570762232829481370756359578518086990519993285655852781, 
    //          11559732032986387107991004021392285783925812861821192530917403151452391805634
    //        ],
    //        FQ2(
    //          [
    //          8495653923123431417604973247489272438418190587263600148770280649306958101930, 
    //          4082367875863433681332203403145435568316851327593401208105741076214120093531
    //        ]
    //    )
    // )

        return G2Point(
            [
                10857046999023057135944570762232829481370756359578518086990519993285655852781, 
                11559732032986387107991004021392285783925812861821192530917403151452391805634
            ],
            [
                8495653923123431417604973247489272438418190587263600148770280649306958101930, 
                4082367875863433681332203403145435568316851327593401208105741076214120093531
            ]
        );
    }


    // returns 1* generator of G2
    function get_g2_1() public view returns (G2Point memory) {
        Pairing.G2Point memory g2 = Pairing.G2Point(
            [10857046999023057135944570762232829481370756359578518086990519993285655852781, 11559732032986387107991004021392285783925812861821192530917403151452391805634],
            [8495653923123431417604973247489272438418190587263600148770280649306958101930, 4082367875863433681332203403145435568316851327593401208105741076214120093531]
        );
        return g2;
    }

    function create_G1Point(uint256 scalar) public view returns (G1Point memory) {
        return scalar_mul(getG1(), scalar);
    }

    /*
    * @return The negation of p, i.e. p.plus(p.negate()) should be zero.
    */
    function negate(G1Point memory p) internal pure returns (G1Point memory) {
        // The prime q in the base field F_q for G1
        if (p.X == 0 && p.Y == 0) {
            return G1Point(0, 0);
        } else {
            return G1Point(p.X, PRIME_Q - (p.Y % PRIME_Q));
        }
    }

    /*
    * @return r the sum of two points of G1
    */
    function plus(
        G1Point memory p1,
        G1Point memory p2
    ) internal view returns (G1Point memory r) {
        uint256[4] memory input;
        input[0] = p1.X;
        input[1] = p1.Y;
        input[2] = p2.X;
        input[3] = p2.Y;
        bool success;

        // solium-disable-next-line security/no-inline-assembly
        assembly {
            success := staticcall(sub(gas(), 2000), 6, input, 0xc0, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success case 0 { invalid() }
        }

        require(success, "pairing-add-failed");
    }


    function run_dyn(uint256[] memory input, uint256 len) public view returns (bool) {
        uint256 in_size = len * 32;
        uint256 out_size = 32;
        bool result;

        assembly {
            // Skip the length field of the dynamic array
            let inputPtr := add(input, 32)
            
            // Perform the staticcall
            let success := staticcall(gas(), 8, inputPtr, in_size, 0x00, out_size)
            
            // Load the result
            result := mload(0x00)
            
            // Revert if the call was not successful
            if iszero(success) {
                revert(0, 0)
            }
        }

        return result;
    }

    /*
    * @return r the product of a point on G1 and a scalar, i.e.
    *         p == p.scalar_mul(1) and p.plus(p) == p.scalar_mul(2) for all
    *         points p.
    */
    function scalar_mul(
        G1Point memory p, 
        uint256 s
    ) internal view returns (G1Point memory r) 
    {
        uint256[3] memory input;
        input[0] = p.X;
        input[1] = p.Y;
        input[2] = s;
        bool success;
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            // address 
            success := staticcall(sub(gas(), 2000), 7, input, 0x80, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success case 0 { invalid() }
        }
        require(success, "pairing-mul-failed");
  }

    function pairing(
        G1Point memory a1,
        G2Point memory a2,
        G1Point memory b1,
        G2Point memory b2,
        G1Point memory c1,
        G2Point memory c2,
        G1Point memory d1,
        G2Point memory d2
    ) internal view returns (bool) {
        G1Point[4] memory p1 = [a1, b1, c1, d1];
        G2Point[4] memory p2 = [a2, b2, c2, d2];

        uint256 inputSize = 24;
        uint256[] memory input = new uint256[](inputSize);

        for (uint256 i = 0; i < 4; i++) {
            uint256 j = i * 6;
            input[j + 0] = p1[i].X;
            input[j + 1] = p1[i].Y;
            input[j + 2] = p2[i].X[1];
            input[j + 3] = p2[i].X[0];
            input[j + 4] = p2[i].Y[1];
            input[j + 5] = p2[i].Y[0];
        }

        uint256[1] memory out;
        bool success;


        // solium-disable-next-line security/no-inline-assembly
        assembly {
            success := staticcall(sub(gas(), 2000), 8, add(input, 0x20), mul(inputSize, 0x20), out, 0x20)
            // Use "invalid" to make gas estimation work
            switch success case 0 { invalid() }
        }

        require(success, "pairing-opcode-failed");
        console2.log("out[0]: %s", out[0]);

        return out[0] != 0;
    }
}

contract Verifier {
    struct VerifyingKey {
        Pairing.G1Point alfa1;
        Pairing.G2Point beta2;
        Pairing.G2Point gamma2;
        Pairing.G2Point delta2;
    }

    function verifyingKey() internal view returns (VerifyingKey memory vk) {
        uint256 scalar_alpha1 = 1;
        uint256 scalar_beta2 = 1;
        uint256 scalar_gamma2 = 1;
        uint256 scalar_delta2 = 1;

        Pairing.G1Point memory alpha1 = Pairing.create_G1Point(scalar_alpha1);
        Pairing.G2Point memory g2 = Pairing.get_g2_1();
        Pairing.G2Point memory beta2 = g2;
        Pairing.G2Point memory gamma2 = g2;
        Pairing.G2Point memory delta2 = g2;

        return VerifyingKey(alpha1, beta2, gamma2, delta2);
    }

    function print_G2Point(Pairing.G2Point memory g2, string memory name) internal view {
        
        uint256 x1 = g2.X[0];
        uint256 x2 = g2.X[1];
        uint256 y1 = g2.Y[0];
        uint256 y2 = g2.Y[1];
        console2.log(name, ": xs", x1, x2);
        console2.log(name, ": ys", y1, y2);
    }

    function verify(
        Pairing.G1Point memory A1,
        Pairing.G2Point memory B2,
        Pairing.G1Point memory C1,
        uint256 x1,
        uint256 x2,
        uint256 x3
    ) public view returns (bool) {
        //check each G point is less than the prime q
        require(A1.X < Pairing.PRIME_Q && A1.Y < Pairing.PRIME_Q, "A1 is not a valid G1 point");
        require(B2.X[0] < Pairing.PRIME_Q && B2.X[1] < Pairing.PRIME_Q && B2.Y[0] < Pairing.PRIME_Q && B2.Y[1] < Pairing.PRIME_Q, "B2 is not a valid G2 point");
        require(C1.X < Pairing.PRIME_Q && C1.Y < Pairing.PRIME_Q, "C1 is not a valid G1 point");

        VerifyingKey memory vk = verifyingKey();

        uint256 scalar_x1 = (x1 + x2 + x3) % Pairing.CurveOrder;
        Pairing.G1Point memory x1G1 = Pairing.create_G1Point(scalar_x1);
        console2.log("A1: ", A1.X, A1.Y);

        print_G2Point(B2, 'b2');
        console2.log("vk.alfa1: ", vk.alfa1.X, vk.alfa1.Y);
        print_G2Point(vk.beta2, 'vk.beta2');
        console2.log("x1G1: ", x1G1.X, x1G1.Y);
        print_G2Point(vk.gamma2, 'vk.gamma2');
        console2.log("C1: ", C1.X, C1.Y);
        print_G2Point(vk.delta2, 'vk.delta2');

        return Pairing.pairing(
            Pairing.negate(A1),
            B2,
            vk.alfa1,
            vk.beta2,
            x1G1,
            vk.gamma2,
            C1,
            vk.delta2
        );
    }
}


/* 
python results, pairing returns true
A1: (1368015179489954701390400359078579693043519447331113978918064868415326638035, 9918110051302171585080402603319702774565515993150576347155970296011118125764)
B2: ((10857046999023057135944570762232829481370756359578518086990519993285655852781, 11559732032986387107991004021392285783925812861821192530917403151452391805634), (8495653923123431417604973247489272438418190587263600148770280649306958101930, 4082367875863433681332203403145435568316851327593401208105741076214120093531))
vk.alfa1: (1, 2)
vk.beta2: ((10857046999023057135944570762232829481370756359578518086990519993285655852781, 11559732032986387107991004021392285783925812861821192530917403151452391805634), (8495653923123431417604973247489272438418190587263600148770280649306958101930, 4082367875863433681332203403145435568316851327593401208105741076214120093531))
x1G1: (1, 21888242871839275222246405745257275088696311157297823662689037894645226208581)
vk.gamma2: ((10857046999023057135944570762232829481370756359578518086990519993285655852781, 11559732032986387107991004021392285783925812861821192530917403151452391805634), (8495653923123431417604973247489272438418190587263600148770280649306958101930, 4082367875863433681332203403145435568316851327593401208105741076214120093531))
C1: (1368015179489954701390400359078579693043519447331113978918064868415326638035, 9918110051302171585080402603319702774565515993150576347155970296011118125764)
vk.delta2: ((10857046999023057135944570762232829481370756359578518086990519993285655852781, 11559732032986387107991004021392285783925812861821192530917403151452391805634), (8495653923123431417604973247489272438418190587263600148770280649306958101930, 4082367875863433681332203403145435568316851327593401208105741076214120093531))
 */