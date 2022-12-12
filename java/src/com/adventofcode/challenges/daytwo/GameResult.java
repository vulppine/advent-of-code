package com.adventofcode.challenges.daytwo;

public enum GameResult {
    Loss(0),
    Draw(3),
    Win(6);

    public int value;

    GameResult(int value) {
        this.value = value;
    }
}
