package com.adventofcode.challenges.daytwo;

import org.junit.Test;
import org.junit.jupiter.api.Assertions;

import java.io.IOException;

public class DayTwoTest {
    @Test
    public void ExampleCase() throws IOException {
        var game = DayTwo.parseStrategyGuideFromString("""
                A Y
                B X
                C Z 
                """);

        var result = game.getScore();

        Assertions.assertEquals(result, 15);
    }
}
