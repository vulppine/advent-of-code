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

    @Test
    public void PartOne() throws IOException {
        var rucksacks = DayThree.parseFromFile("../input/day_three.input");
        var score = rucksacks.getArrangementScore();

        System.out.println(score);
    }

    @Test
    public void ExampleCasePartTwo() throws IOException {
        var input = """
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw 
                """;

        var rucksacks = DayThree.parseFromString(input);
        var score = rucksacks.getGroupScores();

        Assertions.assertEquals(score, 70);
    }

    @Test
    public void PartTwo() throws IOException {
        var rucksacks = DayThree.parseFromFile("../input/day_three.input");
        var score = rucksacks.getGroupScores();

        System.out.println(score);
    }
}
