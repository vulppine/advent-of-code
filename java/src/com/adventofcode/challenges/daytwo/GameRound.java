package com.adventofcode.challenges.daytwo;

public class GameRound {
    private GameSelection opponentPlay;
    private GameSelection ourPlay;

    public GameRound(GameSelection opponent, GameSelection ours) {
        opponentPlay = opponent;
        ourPlay = ours;
    }

    public int getScore() {
        var result = ourPlay.checkWin(opponentPlay);

        return result.value + ourPlay.value;
    }
}
