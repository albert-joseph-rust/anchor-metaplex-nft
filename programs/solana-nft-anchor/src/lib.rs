use anchor_lang::prelude::*;

declare_id!("8id7EM6BhrJgDFnotRyNTYWbEX2pSAhq3WEb76we1kCB");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
