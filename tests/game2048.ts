import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Game2048 } from "../target/types/game2048";
import { expect } from 'chai';

async function play(program: Program<Game2048>, game, player,
  direction, expectedTurn, expectedGameState, expectedBoard) {
  await program.methods
    .play(direction)
    .accounts({
      player: player.publicKey,
      game
    })
    .signers(player instanceof (anchor.Wallet as any) ? [] : [player])
    .rpc();

  const gameState = await program.account.game.fetch(game);
  expect(gameState.turn).to.equal(expectedTurn);
  expect(gameState.state).to.eql(expectedGameState);
  
  expect(gameState.board)
    .to
    .eql(expectedBoard);
}

describe("game2048", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Game2048 as Program<Game2048>;

  it('Setup Game', async () => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.turn).to.equal(1);
    expect(gameState.state).to.eql({ active: {} });
    expect(gameState.board)
      .to
      .eql([[0, 0, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 0]]);
  });

  it('Left move', async () => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.turn).to.equal(1);
    expect(gameState.state).to.eql({ active: {} });
    expect(gameState.board)
      .to
      .eql([[0, 0, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 0]]);

    await play(
      program,
      gameKeypair.publicKey,
      player,
      {left: {}},
      2,
      { active: {}, },
      [
         [0, 2, 0, 0],
         [2, 0, 0, 0],
         [2, 0, 0, 0],
         [0, 0, 0, 0]
      ]
    );

    await play(
      program,
      gameKeypair.publicKey,
      player,
      {left: {}},
      3,
      { active: {}, },
      [
         [2, 0, 0, 2],
         [2, 0, 0, 0],
         [2, 0, 0, 0],
         [0, 0, 0, 0]
      ]
    );

    await play(
      program,
      gameKeypair.publicKey,
      player,
      {left: {}},
      4,
      { active: {}, },
      [
         [4, 0, 0, 0],
         [2, 2, 0, 0],
         [2, 0, 0, 0],
         [0, 0, 0, 0]
      ]
    );
  });


});
