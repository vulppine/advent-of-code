package com.adventofcode.challenges.daytwo;

public class GameStrategyBothSelectParser implements GameStrategyParser {
    @Override
    public GameRound getStrategy(char opponentPlay, char ourPlay) {
        return new GameRound(
                GameSelection.fromLetter(opponentPlay),
                GameSelection.fromLetter(ourPlay)
        );
    }
}
