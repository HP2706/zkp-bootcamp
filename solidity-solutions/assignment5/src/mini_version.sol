// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Pairings {
    /** 
     *  returns true if == 0,
     *  returns false if != 0,
     *  reverts with "Wrong pairing" if invalid pairing
     */
     function run(uint256[18] memory input) public view returns (bool) {
        assembly {
            let success := staticcall(gas(), 8, input, mul(18, 32), input, 32)
            if success {
                return(input, 32)
            }
        }
        revert("Wrong pairing");
    }
}