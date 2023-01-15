// SPDX-License-Identifier: MIT
pragma solidity 0.8.7;

error cooldown();
error faucetNotFunded();

contract faucetSHM {

    mapping(address => uint) public userPreviousWithdrawTime;

    receive() external payable {}

    function useFaucet() public {
        if(block.timestamp < userPreviousWithdrawTime[msg.sender] + 43200) {revert cooldown();} 
        if(address(this).balance < 50 ether) {revert faucetNotFunded();}
        userPreviousWithdrawTime[msg.sender] = block.timestamp; //Current faucet user address records current UNIX time for cooldown check. 
        payable(msg.sender).transfer(50 ether);             //Send 20 LINK to current faucet user address.0x05b7fBE24C98b81C558Aa2fC1e490f7E83AA62Bc
    }
}
