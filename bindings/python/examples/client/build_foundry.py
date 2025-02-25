from iota_sdk import *
import json

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Configure foundry output
# TODO: replace with your own values
serial_number = 1
token_scheme = TokenScheme(0, 32, 64)
unlock_conditions = [
    ImmutableAliasAddressUnlockCondition(
        AliasAddress('0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa')
    )
]

# Configure and build and foundry output
output = client.build_foundry_output(
    1,
    token_scheme,
    unlock_conditions,
)

# Print the output
print(json.dumps(output, indent=4))

