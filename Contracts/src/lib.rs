use anchor_lang::prelude::*;

declare_id!("7RHSYqcQQiVksPx6Wgw5zW5JXERSwBnNgpVQEg9afmi1");

pub mod states;
pub mod constants;
use crate::{states::*, constants::*};

#[program]
pub mod beat_folio{
    use super::*;

    pub fn intialize_account(ctx: Context<InitializeAccount>, _name: String, _bio: String, _cover: String, _dp: String) -> Result<()>{
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.name = _name;
        user_profile.bio = _bio;
        user_profile.cover = _cover;
        user_profile.dp = _dp;
        user_profile.song_count = 0;

        Ok(())
    }

    pub fn add_song(ctx: Context<AddSong>, _title: String, _genre: String, _file_url: String, _status: bool) -> Result<()>{
        let user_profile = &mut ctx.accounts.user_profile;
        let song_post = &mut ctx.accounts.song_post;
        song_post.authority = ctx.accounts.authority.key();
        song_post.title = _title;
        song_post.genre = _genre;
        song_post.file_url = _file_url;
        song_post.idx = user_profile.song_count;
        song_post.status = _status;
        user_profile.song_count = user_profile.song_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn edit_status(ctx: Context<EditStatus>, _status: bool) -> Result<()>{
        let song_post = &mut ctx.accounts.song_post;
        song_post.status = _status;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeAccount<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(init,
             seeds=[USER_TAG, authority.key().as_ref()],
             bump,
             payer=authority,
             space=8 + std::mem::size_of::<User>()
             )]
    pub user_profile : Box<Account<'info , User>>,

    pub system_program : Program<'info , System>
}

#[derive(Accounts)]
#[instruction()]
pub struct AddSong<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(mut,
             seeds=[USER_TAG, authority.key().as_ref()],
             bump,
             has_one=authority)]
    pub user_profile : Box<Account<'info, User>>,

    #[account(init,
             seeds=[SONG_TAG, authority.key().as_ref(), &[user_profile.song_count as u8].as_ref()],
             bump,
             payer=authority,
             space=8 + std::mem::size_of::<Song>()
             )]
    pub song_post : Box<Account<'info, Song>>,

    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
#[instruction(song_idx : u8)]
pub struct EditStatus<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(mut,
             seeds=[USER_TAG, authority.key().as_ref()],
             bump,
             has_one=authority)]
    pub user_profile : Box<Account<'info, User>>,

    #[account(mut,
             seeds=[SONG_TAG, authority.key().as_ref(), &[song_idx].as_ref()],
             bump,
             has_one=authority
             )]
    pub song_post : Box<Account<'info, Song>>,

    pub system_program : Program<'info, System>
}