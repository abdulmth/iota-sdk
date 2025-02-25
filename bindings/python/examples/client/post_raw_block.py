from iota_sdk import Client, MnemonicSecretManager

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Create and post a block without payload
block_id = client.build_and_post_block()[0]
blockBytes = client.get_block_raw(block_id)

# Post raw block
result = client.post_block_raw(blockBytes)

# Print block raw
print(f'{result}')
