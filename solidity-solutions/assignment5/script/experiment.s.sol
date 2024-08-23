// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Pairing, Verifier} from "../src/solution.sol";
import {Pairings} from "../src/mini_version.sol";

contract Main is Script {
    Verifier verifier;
    function setUp() public {
        verifier = new Verifier();
    }

    function bla() public {
        
        /* 
        a = 4
        b = 3
        c = 6
        d = 2
        */

        Pairing.G1Point memory a1 = Pairing.create_G1Point(uint256(4));

        //3
        Pairing.G2Point memory b2 = Pairing.G2Point(
            [uint256(2725019753478801796453339367788033689375851816420509565303521482350756874229), uint256(7273165102799931111715871471550377909735733521218303035754523677688038059653)],
            [uint256(2512659008974376214222774206987427162027254181373325676825515531566330959255), uint256(957874124722006818841961785324909313781880061366718538693995380805373202866)]
        );
       
        //6
        Pairing.G1Point memory c1 = Pairing.create_G1Point(uint256(7));

        //2
        Pairing.G2Point memory d2 = Pairing.G2Point(
            [uint256(18029695676650738226693292988307914797657423701064905010927197838374790804409), uint256(14583779054894525174450323658765874724019480979794335525732096752006891875705)],
            [uint256(2140229616977736810657479771656733941598412651537078903776637920509952744750), uint256(11474861747383700316476719153975578001603231366361248090558603872215261634898)]
        );

        Pairing.G1Point memory neg_a1 = Pairing.negate(a1);
        Pairing.G1Point memory neg_c1 = Pairing.negate(c1);

        /* Pairing.G1Point[4] memory p1 = [neg_a1, neg_c1, a1, c1];
        Pairing.G2Point[4] memory p2 = [b2, d2, b2, d2];

        uint256 inputSize = 24;
        uint256[] memory points = new uint256[](inputSize);

        for (uint256 i = 0; i < 4; i++) {
            uint256 j = i * 6;
            points[j + 0] = p1[i].X;
            points[j + 1] = p1[i].Y;
            points[j + 2] = p2[i].X[1];
            points[j + 3] = p2[i].X[0];
            points[j + 4] = p2[i].Y[1];
            points[j + 5] = p2[i].Y[0];
        }

        for (uint256 i = 0; i < 24; i++) {
            console2.log("points[", i, "]: ", points[i]);
        } 

        bool result = Pairing.run_dyn(points, 24);
        console2.log("result: ", result);
        */

        bool result2 = Pairing.pairing(neg_a1, b2, neg_c1, d2, a1, b2, c1, d2);
        console2.log("result2: ", result2);

    }

    function run() public {
        vm.startBroadcast();
        
        bla();
        vm.stopBroadcast();
    }
}

