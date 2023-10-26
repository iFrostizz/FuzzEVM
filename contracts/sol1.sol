// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.14;

contract sol1 {
    // utils
    function max(uint256 a, uint256 b) internal pure returns (uint256) {
        return a > b ? a : b;
    }

    function max(int256 a, int256 b) internal pure returns (int256) {
        return a > b ? a : b;
    }

    function maxuu(uint256 a, uint256 b) internal pure returns (uint256) {
        return max(a, b);
    }

    function maxii(int256 a, int256 b) internal pure returns (int256) {
        return max(a, b);
    }

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    function min(int256 a, int256 b) internal pure returns (int256) {
        return a < b ? a : b;
    }

    function minuu(uint256 a, uint256 b) internal pure returns (uint256) {
        return min(a, b);
    }

    function minii(int256 a, int256 b) internal pure returns (int256) {
        return min(a, b);
    }

    // runtime
    function max1(uint256 a, uint256 b) external pure returns (uint256) {
        return max(a, b);
    }

    function max2(int256 a, int256 b) external pure returns (int256) {
        return max(a, b);
    }

    function min1(uint256 a, uint256 b) external pure returns (uint256) {
        return min(a, b);
    }

    function min2(int256 a, int256 b) external pure returns (int256) {
        return min(a, b);
    }

    // compile
    function comp_max1() external pure returns (uint256) {
        return maxuu(0, 1);
    }

    function comp_max2() external pure returns (int256) {
        return maxii(123456, 1984791749812);
    }

    function comp_max3() external pure returns (int256) {
        return max(-1, 1984791749812);
    }

    function comp_max4() external pure returns (int256) {
        return maxii(123456, 123456);
    }

    function comp_max5() external pure returns (int256) {
        return max(-123456, -123456);
    }

    function comp_max6() external pure returns (int256) {
        return max(-123457, -123456);
    }

    function comp_min1() external pure returns (uint256) {
        return minuu(0, 1);
    }

    function comp_min2() external pure returns (int256) {
        return minii(123456, 1984791749812);
    }

    function comp_min3() external pure returns (int256) {
        return min(-1, 1984791749812);
    }

    function comp_min4() external pure returns (int256) {
        return minii(123456, 123456);
    }

    function comp_min5() external pure returns (int256) {
        return min(-123456, -123456);
    }

    function comp_min6() external pure returns (int256) {
        return min(-123457, -123456);
    }
}
