// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract sol7 {
    function len1(bytes memory a) external pure returns (uint256 len) {
        assembly {
            len := mload(a)
        }
    }

    function len2(string memory a) external pure returns (uint256 len) {
        len = bytes(a).length;
    }

    function len3(uint256[] memory a) external pure returns (uint256 len) {
        assembly {
            len := mload(a)
        }
    }

    function len4(uint256[][] memory a) external pure returns (uint256 len) {
        assembly {
            len := mload(a)
        }
    }

    function len5(bytes[][] memory a) external pure returns (uint256 len) {
        assembly {
            len := mload(a)
        }
    }
}
