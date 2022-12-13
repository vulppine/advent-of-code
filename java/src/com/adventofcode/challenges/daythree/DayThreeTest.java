package com.adventofcode.challenges.daythree;

import org.junit.Test;
import org.junit.jupiter.api.Assertions;

import java.io.IOException;

public class DayThreeTest {
    @Test
    public void ExampleCase() throws IOException {
        var input = """
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw 
                """;

        var rucksacks = DayThree.parseFromString(input);
        var score = rucksacks.getArrangementScore();

        Assertions.assertEquals(score, 157);
    }
}
