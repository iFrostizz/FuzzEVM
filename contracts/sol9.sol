// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract sol9 {
    function cak1(bytes memory value) external pure returns (bytes32) {
        return keccak256(value);
    }

    function cak2(string memory value) external pure returns (bytes32) {
        return keccak256(bytes(value));
    }

    function cak3(bytes32 value) external pure returns (bytes32) {
        return keccak256(abi.encode(value));
    }

    function cha1(bytes memory value) external pure returns (bytes32) {
        return sha256(value);
    }

    function cha2(string memory value) external pure returns (bytes32) {
        return sha256(bytes(value));
    }

    function cha3(bytes32 value) external pure returns (bytes32) {
        return sha256(abi.encode(value));
    }
}
