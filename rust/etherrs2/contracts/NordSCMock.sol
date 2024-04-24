// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract NordSCMock {
    /// @param prevStateHash The summary of the previous state to update from.
    /// @param pendingStateHash The summary of the pending state to update to.
    /// @param daFact The data availability fact that proves the pending state.
    /// @param onchainUpdatesHash The hash of the onchain updates that are applied to the pending state.
    struct StateUpdateFacts {
        uint256 prevStateHash;
        uint256 pendingStateHash;
        uint256 daFact;
        uint256 onchainUpdatesHash;
    }

    /// @notice Denotes the state of a fact in a fact registry.
    enum FactState {
        NOT_INITIATED,
        OPTIMISTIC,
        CHALLENGED,
        REVERTED,
        FINALIZED_TIMEOUT,
        FINALIZED_PROVED
    }

    struct G1Point {
        uint256 X;
        uint256 Y;
    }

    struct BlobHeader {
        G1Point commitment; // the kzg commitment to the blob
        uint32 dataLength; // the length of the blob in coefficients of the polynomial
        QuorumBlobParam[] quorumBlobParams; // the quorumBlobParams for each quorum
    }

    struct QuorumBlobParam {
        uint8 quorumNumber;
        uint8 adversaryThresholdPercentage;
        uint8 quorumThresholdPercentage;
        uint8 quantizationParameter; // the quantization parameter used for determining
            // the precision of the amount of data and the stake that nodes have
    }

    struct BatchHeader {
        bytes32 blobHeadersRoot;
        bytes quorumNumbers; // each byte is a different quorum number
        bytes quorumThresholdPercentages; // every bytes is an amount less than 100 specifying the percentage of stake
            // the must have signed in the corresponding quorum in `quorumNumbers`
        uint32 referenceBlockNumber;
    }

    // Relevant metadata for a given datastore
    struct BatchMetadata {
        BatchHeader batchHeader; // the header of the data store
        bytes32 signatoryRecordHash; // the hash of the signatory record
        uint96 fee; // the amount of paymentToken paid for the datastore
        uint32 confirmationBlockNumber; // the block number at which the batch was confirmed
    }

    struct BlobVerificationProof {
        uint32 batchId;
        uint8 blobIndex;
        BatchMetadata batchMetadata;
        bytes inclusionProof;
        bytes quorumThresholdIndexes;
    }

    struct StateAuthentication {
        bytes seal; // SNARK proof
        bytes32 postStateDigest; // Digest of the zkVM SystemState after execution.
    }

    uint256 internal _proposedBlockId;
    mapping(uint256 => StateUpdateFacts) internal _pendingFacts;
    mapping(uint256 => FactState) internal _registeredDAFact;
    string text;

    /// Constructor ///
    constructor(string memory _text) {
	text = _text;
	_proposedBlockId = 0;
    }

    /// GETTERS ///

    function getProposedBlockId() external view returns (uint256) {
        return _proposedBlockId;
    }

    function getPendingFacts(uint256 blockId) external view returns (StateUpdateFacts memory) {
        return _pendingFacts[blockId];
    }

    /// MOCK FUNCTIONS ///

    function verifyBlobs(
        BlobHeader[] memory blobHeaders,
        BlobVerificationProof[] memory blobVerificationProofs,
        bytes32 dataDigest,
        StateAuthentication memory auth
    ) external {}

    function submitStateUpdate(StateUpdateFacts memory updateFacts) external {}

    /// SETTERS ///

    function setDAFactState(uint256 fact, FactState state) external {
        _registeredDAFact[fact] = state;
    }

    function incrementProposedBlockId() external {
        _proposedBlockId++;
    }

    function setPendingFacts(uint256 blockId, StateUpdateFacts memory updateFacts) external {
        _pendingFacts[blockId] = updateFacts;
    }
}
