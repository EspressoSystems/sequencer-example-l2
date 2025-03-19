pragma solidity ^0.8.13;

import "../lib/espresso-network/contracts/src/LightClient.sol";

contract ExampleRollup {
    LightClient public lightClient;
    uint256 public stateCommitment;
    uint256 public numVerifiedBlocks;

    // An example batch proof of the execution of a chain of blocks.
    //  In a real rollup, this batch proof should be modified based on the requirements.
    struct BatchProof {
        uint256 firstBlock;
        uint256 lastBlock;
        uint256 oldState;
        uint256 newState;
    }

    // Attempted to verify a proof of the blocks from `numVerifiedBlocks` to
    // `numVerifiedBlocks + count`, but the LightClient `blockHeight` is less than
    // `numVerifiedBlocks + count`.
    error NotYetSequenced(uint256 numVerifiedBlocks, uint64 count, uint256 blockHeight);
    // Attempted to verify an empty chain of blocks;
    error NoBlocks();

    // Thrown when the proof is invalid
    error InvalidProof();

    event StateUpdate(uint256 blockHeight, uint256 stateCommitment);

    constructor(address lightClientAddress, uint256 initialState) {
        lightClient = LightClient(lightClientAddress);

        stateCommitment = initialState;
        numVerifiedBlocks = 0;
    }

    function verifyBlocks(uint64 count, uint256 nextStateCommitment, BatchProof memory proof) external {
        if (count == 0) {
            revert NoBlocks();
        }

        if (proof.newState != nextStateCommitment) {
            revert InvalidProof();
        }

        //  Use Light client contract to verify the batch proof and do other validations when in production
        (, uint64 blockHeight,) = lightClient.finalizedState();
        if (numVerifiedBlocks + count > blockHeight) {
            revert NotYetSequenced(numVerifiedBlocks, count, blockHeight);
        }

        numVerifiedBlocks += count;
        stateCommitment = nextStateCommitment;
        emit StateUpdate(numVerifiedBlocks, stateCommitment);
    }
}
