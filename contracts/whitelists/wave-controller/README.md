# Wave Controller

A wave controller manages all the whitelists for a collection launch. It manages all types of whitelists including smart whitelists. Whitelists can be added, removed from, and updated by the wave controller.

Note: The public (open) mint is also a type of smart whitelist.

A wave controller maintains all the accounting for the mint, like how many times an address has minted. This enables tracking the mint count for an address that spans multiple whitelists.

The process for minting is as follows:

Minters are instantiated with a wave controller. This is the sequence for a mint:

- `PreMint()` is called on the wave controller to check if the address is allowed to mint
- `Mint()` is called on the minter to mint the NFT(s)
- `PostMint()` is called on the wave controller to update the accounting for the address

A wave controller is instantiated with an `owner`, usually the creator, responsible for managing the whitelists. The owner can add, remove, and update whitelists. They can also transfer ownership to another address or renounce ownership entirely. If an owner renounces ownership, then the wave is immutable and no updates can be made.
