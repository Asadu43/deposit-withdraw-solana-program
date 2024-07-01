use anchor_lang::prelude::*;

declare_id!("4K7D7cDzpnGdKKFQhAP6TPLFnKkeVqVPkjRgggNHsd7C");

#[program]
pub mod deposit_withdraw_solana_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, name: String) -> Result<()> {
        let bank = &mut _ctx.accounts.bank;
        bank.name = name;
        bank.balance = 0;
        bank.owner = *_ctx.accounts.user.key;
        Ok(())
    }

    pub fn deposit(_ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let txn = anchor_lang::solana_program::system_instruction::transfer(
            &_ctx.accounts.user.key(),
            &_ctx.accounts.bank.key(),
            amount
        );

        anchor_lang::solana_program::program::invoke(
            &txn,
            &[_ctx.accounts.user.to_account_info(), _ctx.accounts.bank.to_account_info()]
        )?;

        _ctx.accounts.bank.balance += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 32, // Adjusted space to accommodate the struct fields
        seeds = [b"bankaccount", user.key().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Bank {
    pub name: String,
    pub balance: u64,
    pub owner: Pubkey,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
