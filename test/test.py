from ic.principal import Principal
from ic.identity import Identity
from ic.client import Client
from ic.agent import Agent
from ic.candid import encode, decode, Types
from ic.canister import Canister
from eth_account import Account
import secrets
from web3 import Web3


# change these according to the enviremnet 
web3 = Web3(Web3.HTTPProvider("HTTP://127.0.0.1:7545"))
client = Client(url = 'http://localhost:37201')
private_key = "e86158020a0d167a2e59e732e2f111f5e5302bb90c5911cc2cfd7080c52f0f5e"
caister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai"
# constants
p = Principal()
i = Identity()
agent = Agent(i, client=client)





def create_new_eth_account(private_key):
    return web3.eth.account.from_key(private_key)

def deploy_test_erc20(account):
    bytecode = open('./contracts/erc20/build/bytecode.bin', 'r').read()
    abi = open('./contracts/erc20/build/ABI.json', 'r').read()
    erc20 = web3.eth.contract(abi=abi, bytecode=bytecode)
    web3.eth.default_account = account.address
    tx_hash = erc20.constructor().transact()
    tx_receipt = web3.eth.wait_for_transaction_receipt(tx_hash)
    return tx_receipt.contractAddress





print("########## Creating ETH account using private_key ##########")
try:
    account = create_new_eth_account(private_key)
except:
    print ("failed")
    exit
print("using account : " + account.address)
print("########## PASS ##########")








erc20_address = deploy_test_erc20(account)
print("erc20 deployed on " + erc20_address)



params = [
	{'type': Types.Text, 'value': account.address},
]

name = agent.update_raw(caister_id, "create_new_proxy_account", encode(params))

result = (name[0]['value'])



token = result.split(',')[0]
proxy = result.split(',')[1]

print("token: " + token)

print("proxy: " + proxy)


params = [
	{'type': Types.Text, 'value': '5d76327e438eaf3a7c7a0a24edc4019d1b8148a1'},
]

name = agent.query_raw("be2us-64aaa-aaaaa-qaabq-cai", "get_proxies", encode(params))


print(name)

params = [
	{'type': Types.Text, 'value': token},
    {'type': Types.Text, 'value': """
    return "ok"
    """},
    {'type': Types.Text, 'value': """
    print_to_icp(MY_PROXY_ACCOUNT);
    transfer_from_erc20("a","a","a", 20 , MY_PROXY_ACCOUNT);
    return "ok"
    """},
]

name = agent.update_raw("be2us-64aaa-aaaaa-qaabq-cai", "execute_script", encode(params))

print(name)