#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct IanaAccount {
    pub owner: Pubkey,
    pub count_as: u32,
    pub bump: u8,
}

impl<'info, 'entrypoint> IanaAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedIanaAccount<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let count_as = account.count_as;
        let bump = account.bump;

        Mutable::new(LoadedIanaAccount {
            __account__: account,
            __programs__: programs_map,
            owner,
            count_as,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedIanaAccount>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let count_as = loaded.count_as;

        loaded.__account__.count_as = count_as;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedIanaAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, IanaAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub count_as: u32,
    pub bump: u8,
}

#[account]
#[derive(Debug)]
pub struct PrefixAccount {
    pub owner: Pubkey,
    pub prefix: u32,
    pub mask: u8,
}

impl<'info, 'entrypoint> PrefixAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedPrefixAccount<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let prefix = account.prefix;
        let mask = account.mask;

        Mutable::new(LoadedPrefixAccount {
            __account__: account,
            __programs__: programs_map,
            owner,
            prefix,
            mask,
        })
    }

    pub fn store(loaded: Mutable<LoadedPrefixAccount>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let prefix = loaded.prefix;

        loaded.__account__.prefix = prefix;

        let mask = loaded.mask;

        loaded.__account__.mask = mask;
    }
}

#[derive(Debug)]
pub struct LoadedPrefixAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, PrefixAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub prefix: u32,
    pub mask: u8,
}

#[account]
#[derive(Debug)]
pub struct AsAccount {
    pub owner: Pubkey,
    pub n: u32,
    pub bump: u8,
}

impl<'info, 'entrypoint> AsAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedAsAccount<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let n = account.n;
        let bump = account.bump;

        Mutable::new(LoadedAsAccount {
            __account__: account,
            __programs__: programs_map,
            owner,
            n,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedAsAccount>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let n = loaded.n;

        loaded.__account__.n = n;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedAsAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, AsAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub n: u32,
    pub bump: u8,
}

pub fn init_prefix_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut iana: Mutable<LoadedIanaAccount<'info, '_>>,
    mut _as: Mutable<LoadedAsAccount<'info, '_>>,
    mut prefix: Empty<Mutable<LoadedPrefixAccount<'info, '_>>>,
    mut ip_prefix: u32,
    mut ip_mask: u8,
) -> () {
    if !(owner.key() == iana.borrow().owner) {
        panic!("You aren't IANA");
    }

    let mut prefix_acct = prefix.account.clone();

    assign!(
        prefix_acct.borrow_mut().owner,
        _as.borrow().__account__.key()
    );

    assign!(prefix_acct.borrow_mut().prefix, ip_prefix);

    assign!(prefix_acct.borrow_mut().mask, ip_mask);

    solana_program::msg!(
        "{} {} {} {}",
        "Added prefix/mask ",
        prefix_acct.borrow().prefix,
        "/",
        prefix_acct.borrow().mask
    );
}

pub fn init_iana_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut iana: Empty<Mutable<LoadedIanaAccount<'info, '_>>>,
) -> () {
    let mut iana_acct = iana.account.clone();

    assign!(iana_acct.borrow_mut().owner, owner.key());

    assign!(iana_acct.borrow_mut().count_as, 0);

    assign!(iana_acct.borrow_mut().bump, iana.bump.unwrap());
}

pub fn init_as_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut iana: Mutable<LoadedIanaAccount<'info, '_>>,
    mut _as: Empty<Mutable<LoadedAsAccount<'info, '_>>>,
) -> () {
    if !(owner.key() == iana.borrow().owner) {
        panic!("You aren't IANA");
    }

    let mut as_acct = _as.account.clone();

    assign!(as_acct.borrow_mut().n, iana.borrow().count_as);

    assign!(as_acct.borrow_mut().owner, owner.key());

    assign!(as_acct.borrow_mut().bump, _as.bump.unwrap());

    solana_program::msg!("{} {}", "Added ASN #", iana.borrow().count_as);

    assign!(iana.borrow_mut().count_as, iana.borrow().count_as + 1);
}
