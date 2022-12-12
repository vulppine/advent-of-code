package com.adventofcode.challenges.daytwo;

public interface GameStrategyParser {
    public GameRound getStrategy(char opponentPlay, char ourPlay);
}
