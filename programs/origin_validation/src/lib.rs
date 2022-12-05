#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("7n4AUGyiAbCwv3F6GKyJAPAS9KCbDb3QKYsgHcPCPnFP");

pub mod seahorse_util {
    use super::*;

    #[cfg(feature = "pyth-sdk-solana")]
    pub use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }
}

#[program]
mod origin_validation {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    # [instruction (ip_prefix : u32 , ip_mask : u8)]
    pub struct InitPrefix<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        #[account(mut)]
        pub iana: Box<Account<'info, dot::program::IanaAccount>>,
        #[account(mut)]
        pub _as: Box<Account<'info, dot::program::AsAccount>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: PrefixAccount > () + 8 , payer = owner , seeds = ["prefix-account" . as_bytes () . as_ref () , owner . key () . as_ref ()] , bump)]
        pub prefix: Box<Account<'info, dot::program::PrefixAccount>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn init_prefix(ctx: Context<InitPrefix>, ip_prefix: u32, ip_mask: u8) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let iana = dot::program::IanaAccount::load(&mut ctx.accounts.iana, &programs_map);
        let _as = dot::program::AsAccount::load(&mut ctx.accounts._as, &programs_map);
        let prefix = Empty {
            account: dot::program::PrefixAccount::load(&mut ctx.accounts.prefix, &programs_map),
            bump: ctx.bumps.get("prefix").map(|bump| *bump),
        };

        init_prefix_handler(
            owner.clone(),
            iana.clone(),
            _as.clone(),
            prefix.clone(),
            ip_prefix,
            ip_mask,
        );

        dot::program::IanaAccount::store(iana);

        dot::program::AsAccount::store(_as);

        dot::program::PrefixAccount::store(prefix.account);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct InitIana<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        # [account (init , space = 4096 as usize , payer = owner , seeds = ["iana-account" . as_bytes () . as_ref () , owner . key () . as_ref ()] , bump)]
        pub iana: Box<Account<'info, dot::program::IanaAccount>>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

    pub fn init_iana(ctx: Context<InitIana>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let iana = Empty {
            account: dot::program::IanaAccount::load(&mut ctx.accounts.iana, &programs_map),
            bump: ctx.bumps.get("iana").map(|bump| *bump),
        };

        init_iana_handler(owner.clone(), iana.clone());

        dot::program::IanaAccount::store(iana.account);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct InitAs<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        #[account(mut)]
        pub iana: Box<Account<'info, dot::program::IanaAccount>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: AsAccount > () + 8 , payer = owner , seeds = ["as-account" . as_bytes () . as_ref () , owner . key () . as_ref ()] , bump)]
        pub _as: Box<Account<'info, dot::program::AsAccount>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn init_as(ctx: Context<InitAs>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let iana = dot::program::IanaAccount::load(&mut ctx.accounts.iana, &programs_map);
        let _as = Empty {
            account: dot::program::AsAccount::load(&mut ctx.accounts._as, &programs_map),
            bump: ctx.bumps.get("_as").map(|bump| *bump),
        };

        init_as_handler(owner.clone(), iana.clone(), _as.clone());

        dot::program::IanaAccount::store(iana);

        dot::program::AsAccount::store(_as.account);

        return Ok(());
    }
}
