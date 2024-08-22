// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {Pairing, Verifier} from "../src/solution.sol";


contract VerifierTest is Test {
    Verifier public verifier;

    function setUp() public {
        verifier = new Verifier();
    }

    function testVerifier() public view {
        /// define points
        uint256 scalar_a1 = 2;
        uint256 scalar_b2 = 1;
        uint256 scalar_c1 = 2;

        uint256 scalar_x1 = 1;
        uint256 scalar_x2 = 1;
        uint256 scalar_x3 = Pairing.PRIME_Q-3; //use additive inverse of 3

        Pairing.G1Point memory A1 = Pairing.create_G1Point(scalar_a1);
        Pairing.G2Point memory B2 = Pairing.create_G2Point(scalar_b2);
        Pairing.G1Point memory C1 = Pairing.create_G1Point(scalar_c1);


        bool result = verifier.verify(
            A1,
            B2,
            C1,
            scalar_x1,
            scalar_x2,
            scalar_x3
        );
        
        console2.log("result: ", result);
        assert(result);


    }
}