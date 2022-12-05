# Limitations:
# - max account size
# The 10KiB limit for accounts is only for accounts on program-derived addresses. 
# For addresses with a keypair, you can allocate 10MB

# Built with Seahorse v2.4.0
from seahorse.prelude import *


# Updates automatically when you build the project.
declare_id('7n4AUGyiAbCwv3F6GKyJAPAS9KCbDb3QKYsgHcPCPnFP');


# class Prefix:
#     owner: Pubkey
#     prefix: u32
#     mask: u8
#     def __init__(self, owner: Pubkey, prefix: u32, mask: u8):
#         self.owner = owner
#         self.prefix = prefix
#         self.mask = mask

# Defining the account which will be stored on-chain for every unique wallet interacting with our program.
class IanaAccount(Account):
    owner: Pubkey
    count_as: u32
    as_keys: Array[Pubkey, 4]
    # prefix_as_map: Array[Prefix, 1]
    bump: u8

class AsAccount(Account):
    owner: Pubkey
    n: u32
    bump: u8

class PrefixAccount(Account):
    owner: Pubkey
    prefix: u32
    mask: u8


# Here we define all our instructions, each of the method below as an RPC end point which can be invoked by clients.
@instruction
def init_iana(owner: Signer, iana: Empty[IanaAccount]):
    # As a new user connects, we create a new IANA account for him and intialize the account.
    iana_acct = iana.init(payer=owner, seeds=["iana-account", owner], space=4096)
    # Assign the owner or the Signer of the one initialize the accouunt to the user's newly created VoteAccount owner.
    iana_acct.owner = owner.key()
    iana_acct.count_as = 0
    iana_acct.as_keys = Array([owner.key() for i in range(4)], 4)
    # Retrieve the bump seed used to create the PDA.
    iana_acct.bump = iana.bump()

# Called to add a new asn. Allowed if the caller is Iana
@instruction
def init_as(owner: Signer, iana: IanaAccount, _as: Empty[AsAccount]):
    assert owner.key() == iana.owner, "You aren't IANA"
    # Assign ASN
    as_acct = _as.init(payer=owner, seeds=["as-account", owner])
    as_acct.n = iana.count_as
    as_acct.owner = owner.key()
    as_acct.bump = _as.bump()
    iana.as_keys[iana.count_as] = owner.key()
    print("Added ASN #", iana.count_as)
    iana.count_as += 1

# Only Iana can add prefix -
# TODO: should be a msg signed by both iana and AS
@instruction
def init_prefix(owner: Signer, iana: IanaAccount, _as: AsAccount, prefix: Empty[PrefixAccount], 
ip_prefix: u32, ip_mask: u8):
    assert owner.key() == iana.owner, "You aren't IANA"
    prefix_acct = prefix.init(payer=owner, seeds=["prefix-account", owner])
    prefix_acct.owner = _as.key()
    prefix_acct.prefix = ip_prefix
    prefix_acct.mask= ip_mask
    # p = Prefix(_as.key(), ip_prefix, ip_mask)
    # iana.prefix_as_map[_as.n] = p
    print("Added prefix/mask ", prefix_acct.prefix, "/", prefix_acct.mask)

# @instruction
# def get_prefix_owner(owner: Signer, iana: IanaAccount, ip_prefix: u32, ip_mask: u8):
#     iana.
