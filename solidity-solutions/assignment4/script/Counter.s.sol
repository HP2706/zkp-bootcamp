// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {ECPoint, ECMath} from "../src/Counter.sol";

contract CounterScript is Script {
    ECMath public ecMath;

    function setUp() public {
        ecMath = new ECMath();
    }

    function testrationalAdd() public {
        vm.startBroadcast();
        ECPoint memory A = ecMath.createPoint(1);
        ECPoint memory B = ecMath.createPoint(2);
        bool result = ecMath.rationalAdd(A, B, 6, 2);
        console2.log("result", result);
        vm.stopBroadcast();
    }

    function testmatmul() public {
        vm.startBroadcast();
        ECPoint[] memory s = new ECPoint[](3);
        s[0] = ecMath.createPoint(1);
        s[1] = ecMath.createPoint(2);
        s[2] = ecMath.createPoint(3);

        uint256[] memory o = new uint256[](3);
        o[0] = 14;
        o[1] = 32;
        o[2] = 50;

        uint256[] memory matrix = new uint256[](9);
        for (uint256 i = 0; i < 9; i++) {
            matrix[i] = i + 1;
        }

        ecMath.matmul(
            matrix,
            3,
            s,
            o
        );
        vm.stopBroadcast();
    }

    function run() public {
        //testrationalAdd();
        testmatmul();

    }
}