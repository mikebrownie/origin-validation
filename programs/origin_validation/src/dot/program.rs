#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct AsAccount {
    pub authority: Pubkey,
    pub owner: Pubkey,
    pub n: u32,
    pub prefix: u8,
    pub mask: u8,
    pub bump: u8,
}

impl<'info, 'entrypoint> AsAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedAsAccount<'info, 'entrypoint>> {
        let authority = account.authority.clone();
        let owner = account.owner.clone();
        let n = account.n;
        let prefix = account.prefix;
        let mask = account.mask;
        let bump = account.bump;

        Mutable::new(LoadedAsAccount {
            __account__: account,
            __programs__: programs_map,
            authority,
            owner,
            n,
            prefix,
            mask,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedAsAccount>) {
        let mut loaded = loaded.borrow_mut();
        let authority = loaded.authority.clone();

        loaded.__account__.authority = authority;

        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let n = loaded.n;

        loaded.__account__.n = n;

        let prefix = loaded.prefix;

        loaded.__account__.prefix = prefix;

        let mask = loaded.mask;

        loaded.__account__.mask = mask;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedAsAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, AsAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub authority: Pubkey,
    pub owner: Pubkey,
    pub n: u32,
    pub prefix: u8,
    pub mask: u8,
    pub bump: u8,
}

#[account]
#[derive(Debug)]
pub struct IanaAccount {
    pub owner: Pubkey,
    pub count_asn: u8,
    pub bump: u8,
}

impl<'info, 'entrypoint> IanaAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedIanaAccount<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let count_asn = account.count_asn;
        let bump = account.bump;

        Mutable::new(LoadedIanaAccount {
            __account__: account,
            __programs__: programs_map,
            owner,
            count_asn,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedIanaAccount>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let count_asn = loaded.count_asn;

        loaded.__account__.count_asn = count_asn;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedIanaAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, IanaAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub count_asn: u8,
    pub bump: u8,
}

pub fn init_iana_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut iana: Empty<Mutable<LoadedIanaAccount<'info, '_>>>,
) -> () {
    let mut iana_acct = iana.account.clone();

    assign!(iana_acct.borrow_mut().owner, owner.key());

    assign!(iana_acct.borrow_mut().count_asn, 0);

    assign!(iana_acct.borrow_mut().bump, iana.bump.unwrap());
}
