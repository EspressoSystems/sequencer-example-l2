// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../lib/espresso-sequencer/contracts/src/LightClient.sol";

import "../src/ExampleRollup.sol";

contract ExampleRollupTest is Test {
    LightClient public lightClient;
    ExampleRollup public rollup;

    event StateUpdate(uint256 blockHeight, uint256 stateCommitment);

    function setUp() public {
        lightClient = new LightClient();
        rollup = new ExampleRollup(address(lightClient), 0);
    }

    function testNoblocks() public {
        ExampleRollup.BatchProof memory proof =
            ExampleRollup.BatchProof({firstBlock: 0, lastBlock: 0, oldState: 0, newState: 0x1});
        vm.expectRevert(ExampleRollup.NoBlocks.selector);
        rollup.verifyBlocks(0, 0, proof);
    }

    function testNotYetSequenced() public {
        ExampleRollup.BatchProof memory proof =
            ExampleRollup.BatchProof({firstBlock: 0, lastBlock: 0, oldState: 0, newState: 0x1});
        vm.expectRevert(abi.encodeWithSelector(ExampleRollup.NotYetSequenced.selector, 0, 2, 0));
        rollup.verifyBlocks(2, 0x1, proof);
    }

    function testInvalidProof() public {
        ExampleRollup.BatchProof memory proof =
            ExampleRollup.BatchProof({firstBlock: 0, lastBlock: 0, oldState: 0, newState: 0x2});
        vm.expectRevert(ExampleRollup.InvalidProof.selector);
        rollup.verifyBlocks(1, 0x1, proof);
    }
}
