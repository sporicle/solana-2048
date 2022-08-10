use std::collections::HashMap;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod game2048 {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>) -> Result<()> {
        ctx.accounts.game.start(ctx.accounts.player.key())
    }

    pub fn play(ctx: Context<Play>, direction: Direction) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.play(&direction)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Game {
    player: Pubkey,       // 32
    board: [[u16; 4]; 4], // 2 * 16
    state: GameState,     // 1 + 4
    turn: u8,             // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Win,
    Loss { score: u32 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub const MAXIMUM_SIZE: usize = 32 + (16 * 2) + 1 + 4 + 1;

    pub fn start(&mut self, player: Pubkey) -> Result<()> {
        require_eq!(self.turn, 0, Twenty48error::GameAlreadyStarted);
        self.board = [[0, 0, 0, 0], 
                      [0, 0, 2, 0],
                      [0, 0, 2, 0],
                      [0, 0, 0, 0]];
        self.state = GameState::Active;
        self.player = player;
        self.turn = 1;
        Ok(())
    }

    pub fn play(&mut self, direction: &Direction) -> Result<()> {
        if *direction == Direction::Left {
            for i in 0..4 {
                let row = self.board[i];
                let mut stack = Vec::new();
                let mut result = Vec::new();
                for element in row {
                    if element != 0 {
                        stack.push(element);
                    }
                }
                while stack.len() > 0 {
                    let a = stack.pop().unwrap();
                    let last = stack.last();
                    match last {
                        Some(val) => {
                            if a == *val {
                                stack.pop();
                                result.push(a*2);
                            }
                            else {
                                result.push(a);
                            }
                        }
                        None => {
                            result.push(a);
                        }
                    }
                }
                
                let mut new_row:[u16 ; 4] = [0,0,0,0];
                for ii in 0..4 {
                    match result.pop() {
                        Some(val) => {
                            new_row[ii] = val;
                        }
                        None => {}
                    }
                }

                self.board[i] = new_row;
            }
        } 
        else if  *direction == Direction::Down {

        }

        Self::generate_new_num(&mut self.board, self.turn);

        self.turn += 1;
        Ok(())
    }

    pub fn generate_new_num(board: &mut[[u16; 4]; 4], turn: u8) {
        let mut map = HashMap::new();
        let mut count = 0;
        for i in 0..4 {
            for j in 0..4 {
                if board[i][j] == 0 {
                    map.insert(count, (i, j));
                    count += 1;
                }
            }
        }

        let x = turn % count;
        board[map.get(&x).unwrap().0][map.get(&x).unwrap().1] = 2;
    }

}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

#[error_code]
pub enum Twenty48error {
    InvalidDirection,
    GameAlreadyOver,
    GameAlreadyStarted,
}

