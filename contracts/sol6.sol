// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract sol6 {
    function _check_mul(
        uint256 a,
        uint256 b
    ) private pure returns (uint256 res) {
        unchecked {
            res = a * b;
            require(a == 0 || res / a == b);
        }
    }

    function as_wei1(uint256 _value) external pure returns (uint256 ret) {
        return _check_mul(_value, 10 ** 18);
    }

    function as_wei2(uint8 _value) external pure returns (uint256) {
        return _check_mul(_value, 10 ** 18);
    }

    function as_wei3(uint256 _value) external pure returns (uint256) {
        return _value;
    }

    function as_wei4(uint8 _value) external pure returns (uint256) {
        return _value;
    }

    function as_wei5(uint256 _value) external pure returns (uint256) {
        return _check_mul(_value, 10 ** 3);
    }

    function as_wei6(uint8 _value) external pure returns (uint256) {
        return _check_mul(_value, 10 ** 3);
    }

    function as_wei7(uint256 _value) external pure returns (uint256) {
        return _check_mul(_value, 10 ** 3);
    }

    function as_wei8(uint256 _value) external pure returns (uint256) {
        return _check_mul(_value, 10 ** 6);
    }

    function as_wei9(uint8 _value) external pure returns (uint256) {
        return _check_mul(_value, 10 ** 6);
    }

    function as_wei10(uint256 _value) external pure returns (uint256) {
        return _check_mul(_value, 10 ** 6);
    }

    function as_wei11(uint8 _value) external pure returns (uint256) {
        return _check_mul(_value, 10 ** 9);
    }
}
