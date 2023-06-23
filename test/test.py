import json
from ic.principal import Principal
from ic.identity import Identity
from ic.client import Client
from ic.agent import Agent
from ic.candid import encode, decode, Types
from ic.canister import Canister
from eth_account import Account
import secrets
from web3 import Web3
import random


# change these according to the enviremnet
web3 = Web3(Web3.HTTPProvider("https://chainnova-scf-chain.darkube.app"))
client = Client(url = 'http://localhost:33035')
private_key = "0x961e65e45b2e16a13ff7d344dd2ea215035e7cfc0c52e8b3c611a2968d10b296"
caister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai"
# constants
p = Principal()
i = Identity()
agent = Agent(i, client=client)



# functions

def create_new_eth_account(private_key):
    return web3.eth.account.from_key(private_key)

def deploy_test_erc20(account):
    nonce = web3.eth.get_transaction_count(account.address)
    bytecode = open('./contracts/erc20/build/bytecode.bin', 'r').read()
    abi = open('./contracts/erc20/build/ABI.json', 'r').read()
    erc20 = web3.eth.contract(abi=abi, bytecode=bytecode)
    tx = erc20.constructor().build_transaction(    {
        'from': account.address,
        'nonce': nonce,
    }
    )
    tx_create = web3.eth.account.sign_transaction(tx,private_key)
    tx_hash = web3.eth.send_raw_transaction(tx_create.rawTransaction)
    tx_receipt = web3.eth.wait_for_transaction_receipt(tx_hash)
    return tx_receipt.contractAddress









# creating a new account
print("########## Creating ETH account using private_key ##########")
try:
    account = create_new_eth_account(private_key)
except:
    print ("failed")
    exit
print("using account : " + account.address)
print("########## PASS ##########")
###########################






# deploying erc20 contract
erc20_address = deploy_test_erc20(account)
print("erc20 deployed on " + erc20_address)
###########################



# creating a new proxy account
params = [
	{'type': Types.Text, 'value': account.address},
]
name = agent.update_raw(caister_id, "create_new_proxy_account", encode(params))
result = (name[0]['value'])
token = result.split(',')[0]
proxy = result.split(',')[1]

print("token: " + token)

print("proxy: " + proxy)
###########################





# transfer 1 eth for transaction fees cover 
from_address = account.address
amount = 1
nonce = web3.eth.get_transaction_count(account.address)
tx = {
    'nonce': nonce,
    'to': Web3.to_checksum_address(proxy),
    'value': web3.to_wei(1, 'ether'),
    'gas': 2000000,
    'gasPrice': web3.to_wei('5000', 'gwei')
}

signed_tx = web3.eth.account.sign_transaction(tx, private_key)
tx_hash = web3.eth.send_raw_transaction(signed_tx.rawTransaction)
receipt = web3.eth.wait_for_transaction_receipt(tx_hash)
###########################






# reading proxy accounts
params = [
	{'type': Types.Text, 'value': account.address},
]

name = agent.query_raw(caister_id, "get_proxies", encode(params))
result = (name[0]['value'])
print("list of proxy accounts : " + result)






# approving erc20 
from_address = account.address
token_amount = 10000
abi = open('./contracts/erc20/build/ABI.json', 'r').read()
contract = web3.eth.contract(address=(Web3.to_checksum_address(erc20_address)), abi=abi)
approve_data = contract.encodeABI(fn_name='approve', args=[Web3.to_checksum_address(proxy), token_amount])
nonce = web3.eth.get_transaction_count(account.address)

tx = {
    'nonce' :nonce,
    'from': from_address,
    'to':  Web3.to_checksum_address(erc20_address),
    'data': approve_data,
    'gas': 200000,  
    'gasPrice': 2000000
}

signed_tx = web3.eth.account.sign_transaction(tx, private_key)
tx_hash = web3.eth.send_raw_transaction(signed_tx.rawTransaction)
receipt = web3.eth.wait_for_transaction_receipt(tx_hash)
###########################






# proxy account balance before rhia
contract = web3.eth.contract(address=erc20_address, abi=abi)

balance = contract.functions.balanceOf(Web3.to_checksum_address(proxy)).call()
print("balance before rhia : " + str( balance))
###########################



# testing rhia

params = [
	{'type': Types.Text, 'value': token},
    {'type': Types.Text, 'value': """
    return "ok"
    """},
    {'type': Types.Text, 'value': f"""
    print_to_icp(MY_PROXY_ACCOUNT);
    transfer_from_erc20("{erc20_address}","{account.address}", "{proxy}", 20 , MY_PROXY_ACCOUNT);
    return "ok"
    """},
]

name = agent.update_raw(caister_id, "execute_script", encode(params))

print(name)

# balance after rhia
contract = web3.eth.contract(address=erc20_address, abi=abi)

balance = contract.functions.balanceOf(Web3.to_checksum_address(proxy)).call()
print("balance after rhia : " + str (balance))
###########################
