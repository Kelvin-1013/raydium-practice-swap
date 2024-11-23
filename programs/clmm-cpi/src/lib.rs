use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

declare_id!("8wGnAiSXYK31kdas6qNgfr8N3pzHoBEAaDK824qgYE81");

#[program]
pub mod clmm_cpi {
    use super::*;

    pub fn proxy_swap<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ProxySwap<'info>>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    ) -> Result<()> {
        instructions::proxy_swap(
            ctx,
            amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
        )
    }
}
