package com.adventofcode.challenges.daytwo;

import org.junit.Test;
import org.junit.jupiter.api.Assertions;

import java.io.IOException;

public class DayTwoTest {
    @Test
    public void ExampleCase() throws IOException {
        /*
        var game = DayTwo.parseStrategyGuideFromString("""
                A Y
                B X
                C Z 
                """);
         */

        var game = DayTwo.parseStrategyGuideFromFile("../input/day_two.test");

        var result = game.getScore();

        Assertions.assertEquals(result, 15);
    }

    @Test
    public void PartOne() throws IOException {
        var game = DayTwo.parseStrategyGuideFromFile("../input/day_two.input");

        var result = game.getScore();

        System.out.println(result);
    }
}
