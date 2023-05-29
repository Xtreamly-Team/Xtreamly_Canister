from ic.principal import Principal
from ic.identity import Identity
from ic.client import Client
from ic.agent import Agent
from ic.candid import encode, decode, Types
from ic.canister import Canister




p = Principal()

i = Identity()

print(i)

client = Client(url = 'http://localhost:44427')

agent = Agent(i, client=client)

params = [
	{'type': Types.Text, 'value': '5d76327e438eaf3a7c7a0a24edc4019d1b8148a1'},
]

name = agent.update_raw("be2us-64aaa-aaaaa-qaabq-cai", "create_new_proxy_account", encode(params))

result = (name[0]['value'])



token = result.split(',')[0]
proxy = result.split(',')[1]

print("token: " + token)

print("proxy: " + proxy)

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