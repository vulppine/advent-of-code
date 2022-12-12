package com.adventofcode.challenges.daytwo;

public enum GameResult {
    Loss(0),
    Draw(3),
    Win(6);

    public int value;

    GameResult(int value) {
        this.value = value;
    }

    public static GameResult fromLetter(char letter) {
        return switch (letter) {
            case 'X' -> Loss;
            case 'Y' -> Draw;
            case 'Z' -> Win;
            default -> throw new IllegalArgumentException("Invalid letter passed into GameResult");
        };
    }
}
