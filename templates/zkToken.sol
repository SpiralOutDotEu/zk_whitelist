// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// import the verifier that the program created
import "./verifier.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

/// @title ZKToken Contract
/// @notice This contract represents an ERC20 token with minting only for ZK proven accounts.
/// @notice Requires a verifier circuit contracts
contract ZKToken is ERC20 {
    Groth16Verifier public verifier;
    mapping(address => bool) public claimed;

    constructor() ERC20("YourToken", "YTK") {
        verifier = new Groth16Verifier();
    }

    /*
    * @notice Mints new tokens after verifying a provided proof.
    * @param pA, pB, pC, pubSignals  The ZK proofs from proofs file.
    * @return A boolean value indicating whether the function executed successfully. Reverts otherwise.
    */
    function mint(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[2] calldata _pubSignals  ) public returns (bool) {
        // Convert msg.sender address to decimal
        uint256 senderDecimalAddress = uint256(uint160(msg.sender));
        
        // Ensure the proof is for sender
        require(senderDecimalAddress == _pubSignals[1], "Not your proof or invalid input");

        // Ensure the tokens haven't been claimed yet
        require(!claimed[msg.sender], "Tokens already claimed");

        // Verify the proof
        require(verifier.verifyProof(_pA, _pB, _pC, _pubSignals), "Invalid proof");

        // Mark as claimed and mint the tokens
        claimed[msg.sender] = true;
        _mint(msg.sender, 10 * 10 ** decimals());
        return true;
    }
}