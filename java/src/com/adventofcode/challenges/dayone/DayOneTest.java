package com.adventofcode.challenges.dayone;

import org.junit.Test;
import org.junit.jupiter.api.Assertions;

import java.io.File;
import java.io.IOException;

public final class DayOneTest {
    @Test
    public void ExampleCase() throws IOException {
        var input = """
                1000
                2000
                3000
                                
                4000
                                
                5000
                6000
                                
                7000
                8000
                9000
                                
                10000
                """;

        var calories = DayOne.parseCalories(input);
        var highest = calories.getHighestCalories();

        Assertions.assertEquals(highest, 24000);
    }
    @Test
    public void PartOne() throws IOException {
        var file = new File("../input/day_one.input");

        var calories = DayOne.parseCalories(file);
        var highest = calories.getHighestCalories();
        System.out.println(highest);
    }

    @Test
    public void PartTwo() throws IOException {
        var file = new File("../input/day_one.input");

        var calories = DayOne.parseCalories(file);
        var threeHighest = calories.getTopThreeHighestCalories();
        System.out.println(threeHighest);
    }
}
