from web3 import Web3
import json
import os

infura_goerli_testnet_url_API = str(os.environ['goerliHTTPS_InfuraAPIKey']);
devTestnetPrivateKey = str(os.environ['devTestnetPrivateKey']);
web3 = Web3(Web3.HTTPProvider(infura_goerli_testnet_url_API))

print("Connected to Web3? ")
print(web3.isConnected())

# Read information from the blockchain.
print("Current block? ")
print(web3.eth.blockNumber)

print("Chain ID? ")
print(web3.eth.chain_id)

balance = web3.eth.getBalance("0xc1202e7d42655F23097476f6D48006fE56d38d4f")
print("Balance [Goerli ether]" )
print(web3.fromWei(balance, "ether") )

Contract_At_Address= web3.toChecksumAddress("0xdbaA7dfBd9125B7a43457D979B1f8a1Bd8687f37");
abi_Contract = json.loads('[{"anonymous":false,"inputs":[],"name":"setEvent","type":"event"},{"inputs":[{"internalType":"uint256","name":"x","type":"uint256"}],"name":"set","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"storedData","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]');
contract_Call = web3.eth.contract(address = Contract_At_Address , abi = abi_Contract);
print(contract_Call.functions.storedData().call());

EIP_1559_tx = {
    'nonce':  web3.eth.getTransactionCount("0xc1202e7d42655F23097476f6D48006fE56d38d4f")       ,
    'to': Contract_At_Address, #WORKS WITH REGULAR WALLETS BUT CANNOT SEND TO CONTRACT FOR SOME REASON?
    'gas': 2100000, #GAS LIMIT. REMOVED FIXED GAS PRICE. NOW DYNAMIC.
    # 'gasPrice': web3.toWei('50', 'gwei'), # https://etherscan.io/gastracker
    'maxFeePerGas': web3.toWei(60, 'gwei'),
    'maxPriorityFeePerGas': web3.toWei(50, 'gwei'),
    'chainId' : web3.eth.chain_id,
    'data' : contract_Call.encodeABI(fn_name='set', args=[23131])
}

signed_tx = web3.eth.account.signTransaction(EIP_1559_tx, devTestnetPrivateKey)
print(web3.toHex(web3.eth.sendRawTransaction(signed_tx.rawTransaction)))
