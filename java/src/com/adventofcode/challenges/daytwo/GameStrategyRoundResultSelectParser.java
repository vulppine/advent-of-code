package com.adventofcode.challenges.daytwo;

public class GameStrategyRoundResultSelectParser implements GameStrategyParser {
    @Override
    public GameRound getStrategy(char opponentPlay, char ourPlay) {
        var opponent = GameSelection.fromLetter(opponentPlay);

        var roundResult = GameResult.fromLetter(ourPlay);

        var ours = GameSelection.getResult(opponent, roundResult);

        return new GameRound(opponent, ours);
    }
}
