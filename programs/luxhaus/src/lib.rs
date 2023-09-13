use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use anchor_lang::prelude::*;

use crate::states::*;
use {
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, SetAuthority, Token, TokenAccount},
    }
};

pub const PREFIX: &str = "luxhaus";
pub const ESCROW: &str = "escrow";

pub mod states;

declare_id!("DcPTwPTmFrmmVEH8nb9VzaU8NX5fAJZWBhCBD4dxBCPi");

#[program]
pub mod luxhaus {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, treasury_fee: u16) -> Result<()> {
        let authority = &ctx.accounts.wallet;
        let lux_haus = &mut ctx.accounts.lux_haus;

        lux_haus.authority = authority.key();
        lux_haus.treasury_fee = treasury_fee;



        OK(())
    }

    pub fn CreateRaffle(ctx: Context<Initialize>,
                        entry_price: u64,
                        reserve_price: u64,
                        start: i64,
                        expiry: i64) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let owner = &ctx.accounts.owner;
        let token_mint = &ctx.accounts.token_mint;
        let src_account = &ctx.accounts.src_account;
        let token_escrow = &ctx.accounts.token_escrow;
        let escrow_sol_account = &ctx.accounts.escrow_sol_account;


        let raffle = &ctx.accounts.raffle.to_account_info();

        // checks


        // assignments
        raffle.entry_price = entry_price;
        raffle.reserve_price = reserve_price;
        raffle.start = start;
        raffle.end = end;

        OK(())


    }

    pub fn PickWinner(ctx: Context<PickWinner>,
                        winner,
    ){

    }

}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    init,
    payer = authority,
    seeds = [
    PREFIX.as_bytes(),
    ],
    space =::LEN,
    bump)]
    lux_haus: Account<'info, LuxHaus>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateRaffle <'info> {
    #[account(
    init,
    payer = authority,
    seeds = [
        PREFIX.as_bytes(),
    ],
    space =::LEN,
    bump)]
    raffle: Account<'info, Raffle>,
    #[account(mut)]
    authority: Signer<'info>,
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        constraint = token_mint.supply == 1 @ ErrorCode::InvalidTokenMint,
        constraint = token_mint.decimals == 0 @ ErrorCode::InvalidTokenMint,
    )]
    token_mint: Account<'info, Mint>,
    #[account(
        constraint = src_nft_account.mint == token_mint.key()
        constraint = src_nft_account.
    )]
    src_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            ESCROW.as_bytes(),
            token_mint.key().as_ref(),
        ]
        constraint =
    )]
    #[account(
        seeds = [

        ]

    )]
    escrow_ata: Account<'info, TokenAccount>,
    escrow_payment_acc: Account<'info, TokenAccount> ,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyEntry <'info> {
    #[account(
    init,
    payer = authority,
    seeds = [
    PREFIX.as_bytes(),
    ],
    space =::LEN,
    bump)]
    raffle: Account<'info, Raffle>,
    #[account(mut)]
    authority: Signer<'info>,
    nft_owner: Signer<'info>,
    token_mint, // nft
    src_nft_account,
    nft_escrow_account,
    escrow_payment_account,
    mint_metadata, // verify
    token_program,
    system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct PickWinner<'info> {
    lux_haus: Account<'info, LuxHaus>,
    #[account(mut)

    ]
    raffle,
    #[account(mut)
        constraint = lux_haus.authority
    ]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    raffle,
    winner,
    token_mint,
    escrow,
    winner_token_acc,
}










