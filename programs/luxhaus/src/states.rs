use anchor_lang::{prelude::*, AnchorDeserialize, Discriminator};

#[account]
#[derive(Default, Copy)]
pub struct LuxHaus {
    // Byte offsets:
    // 0
    // Discriminator
    // 8
    pub lux_key: Pubkey,
    // 40
    pub treasury_fee: u16,
}


#[account]
#[derive(Default, Copy)]
pub struct Raffle {

    pub item: Pubkey,
    pub reserve_price: u64,
    pub price: u64,
    pub winner: Pubkey,
    // add the time here
}


