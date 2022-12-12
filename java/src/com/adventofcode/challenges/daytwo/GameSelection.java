package com.adventofcode.challenges.daytwo;

public enum GameSelection {
    Rock(1),
    Paper(2),
    Scissors(3);

    public int value;

    GameSelection(int value) {
        this.value = value;
    }

    public static GameSelection fromLetter(char letter) {
        return switch (letter) {
            case 'A', 'X' -> Rock;
            case 'B', 'Y' -> Paper;
            case 'C', 'Z' -> Scissors;
            default -> throw new IllegalArgumentException("Incorrect letter passed into selection parse");
        };
    }

    // could this be done by just calculating the numbers?
    public GameResult checkWin(GameSelection other) {
        return switch (this) {
            case Rock -> switch (other) {

                case Rock -> GameResult.Draw;
                case Paper -> GameResult.Loss;
                case Scissors -> GameResult.Win;
            };
            case Paper -> switch (other) {
                case Rock -> GameResult.Win;
                case Paper -> GameResult.Draw;
                case Scissors -> GameResult.Loss;
            };
            case Scissors -> switch (other) {
                case Rock -> GameResult.Loss;
                case Paper -> GameResult.Win;
                case Scissors -> GameResult.Draw;
            };
        };
    }

    public static GameSelection getResult(GameSelection selection, GameResult intendedResult) {
        return switch (intendedResult) {
            case Win -> switch (selection) {
                case Rock -> Paper;
                case Paper -> Scissors;
                case Scissors -> Rock;
            };
            case Draw -> selection;
            case Loss -> switch (selection) {
                case Rock -> Scissors;
                case Paper -> Rock;
                case Scissors -> Paper;
            };
        };
    }
}
