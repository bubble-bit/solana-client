# SOLANA CLIENT

The program creates a new keypair each run, 
save the value from `new key secret:` to a key.txt file
the program wont run to completion until you do this.

this will also allow you to use the airdrop


## Example output

```
new key secret: "3NjpmykQjjrB8STYANPE394EvrNnvK68PkdL2aMzDqNHUfxcnt3B2T89hSQm5SjaX4ewadeUnVy3edNH8NfgXgrs"
Recovered: Keypair(Keypair { secret: SecretKey: [157, 33, 59, 245, 217, 66, 7, 149, 96, 2, 23, 139, 208, 0, 247, 91, 202, 214, 81, 248, 109, 2, 156, 94, 102, 161, 144, 138, 250, 175, 204, 239], public: PublicKey(CompressedEdwardsY: [209, 123, 23, 153, 86, 48, 73, 248, 110, 170, 54, 244, 157, 248, 52, 16, 143, 78, 119, 175, 198, 59, 239, 33, 201, 64, 15, 46, 158, 102, 80, 148]), EdwardsPoint{
	X: FieldElement51([1590647362272195, 1641373663702173, 1368405769229322, 653828792382979, 1665986285600837]),
	Y: FieldElement51([334623470484433, 1053911243677449, 1650617072276450, 110573792387927, 357368825307380]),
	Z: FieldElement51([1, 0, 0, 0, 0]),
	T: FieldElement51([885499967500077, 847755217495851, 1028702037285634, 191621525497255, 1452679189238249])
}) })
Recovered: "49D6tdiBajwQzZXPuzNVuo1f4Mq6rr2vrG34XgYj5Ws1rcx8LLRkcgy8ovwzk6GE6S13KvGiYzaQrdnzvLMTtvbM"

==============================

get_account: Ok(Account { lamports: 3500000000, data.len: 0, owner: 11111111111111111111111111111111, executable: false, rent_epoch: 0 })

==============================

get_account_data: Ok([])

==============================


==============================

epock_info: Ok(EpochInfo { epoch: 381, slot_index: 219594, slots_in_epoch: 432000, absolute_slot: 164811594, block_height: 155040842, transaction_count: Some(7109406400) })

get_slot: 164811600
get_block_production: Ok(Response { context: RpcResponseContext { slot: 164811601, api_version: Some(RpcApiVersion(Version { major: 1, minor: 10, patch: 39 })) }, value: RpcBlockProduction { by_identity: {"dv1ZAGvdsz5hHLwWXsVnM94hWf1pjbKVau1QVkaMJ92": (55250, 55250), "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr": (8, 4), "dv3qDFk1DTF36Z62bNvrCXe9sKATA6xvVy6A798xxAS": (54920, 54920), "dv4ACNkpYPcE3aKmYDqZm9G5EB3J4MRoeE7WNDRBVJB": (54716, 54716), "7LH3HCmvnJRvHvzinbDerTNQ2GvLvdnukdx1dQ26aCFt": (20, 20), "dv2eQHeP4RFrJZ6UeiZWoc3XTtmtZCUKxxCApCDcRNV": (54688, 54688)}, range: RpcBlockProductionRange { first_slot: 164592000, last_slot: 164811601 } } })

```