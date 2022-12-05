# Limitations:
# - max account size
# The 10KiB limit for accounts is only for accounts on program-derived addresses. 
# For addresses with a keypair, you can allocate 10MB

# Built with Seahorse v2.4.0
from seahorse.prelude import *


# Updates automatically when you build the project.
declare_id('7n4AUGyiAbCwv3F6GKyJAPAS9KCbDb3QKYsgHcPCPnFP');
# Defining the account which will be stored on-chain for every unique wallet interacting with our program.
class IanaAccount(Account):
    owner: Pubkey
    # asn_keys: Array[Pubkey, 64] # 32 bytes public-key * 65536 (or 2^16) possible ASNs
    count_asn: u8 # Total number of ASNs added
    bump: u8

class AsAccount(Account):
  authority: Pubkey
  owner: Pubkey
  n: u32
  prefix: u8
  mask: u8
  bump: u8

 

# Here we define all our instructions, each of the method below as an RPC end point which can be invoked by clients.
@instruction
def init_iana(owner: Signer, iana: Empty[IanaAccount]):
    # As a new user connects, we create a new IANA account for him and intialize the account.
    iana_acct = iana.init(payer=owner, seeds=["iana-account", owner])
    # Assign the owner or the Signer of the one initialize the accouunt to the user's newly created VoteAccount owner.
    iana_acct.owner = owner.key()
    iana_acct.count_asn = 0
    # asn_keys = []
    # a = Pubkey
    # for i in range(64):
    #     asn_keys.append(a)

    # iana_acct.asn_keys = Array(u32, 64)
    # Retrieve the bump seed used to create the PDA.
    iana_acct.bump = iana.bump()
  
# Called to add a new asn. Allowed if the caller is Iana
# @instruction
# def init_asn(owner: Signer, iana: IanaAccount, _as: Empty[AsAccount], asn: u32):
#     # Check if the public key of the signer is the same as the owner in the iana account.
#     assert owner.key() == iana.owner, "This is not your Iana account!"
#     # Assign ASN
#     iana.asn_keys[iana.count_asn] = owner.key()
#     print("Added ASN #", iana.count_asn)
#     iana.count_asn += 1

#     as_acct = _as.init(payer=owner, seeds=["Asn", owner])
#     # Retrieve the bump seed used to create the PDA.
#     as_acct.n = asn
#     as_acct.bump = _as.bump()
#     owner

# @instruction
# def init_prefix(owner: Signer, iana: IanaAccount, asn: u16}):
#     # Check if the public key of the signer is the same as the owner in the iana account.
#     # assert owner.key() == iana.owner, "This is not your Iana account!"
  
#   @instruction
# def init_asn(owner: Signer, iana: IanaAccount, asn: u16}):
#     # Check if the public key of the signer is the same as the owner in the iana account.
#     # assert owner.key() == iana.owner, "This is not your Iana account!"
  
# To vote smooth
# @instruction
# def add_prefix(owner: Signer, iana: IanaAccount,  ):
#     # Check if the public key of the signer is the same as the owner in the vote account.
#     assert owner.key() == iana.owner, "This is not your Iana account!"
#     # Increment the smooth variable in the user's VoteAccount

  
# class Route(Account):
#   owner: Pubkey
#   # Ip address prefix
#   prefix: u8
#   mask: u8

# class AsnMap:
  
  