package com.adventofcode.challenges.daythree;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.Reader;
import java.io.StringReader;
import java.nio.Buffer;
import java.util.*;

public class DayThree {
    private List<Rucksack> rucksacks = new ArrayList<>();

    private static int lowestCapitalLetterValue = 'A';
    private static int lowestLowercaseLetterValue = 'a';

    private static int getPriority(char input) {
        if (input < 65) {
            // throw
        }

        if (input <= 'Z') {
            return (26 * 2) - ('Z' - input);
        }

        if (input <= 'z') {
            return 26 - ('z' - input);
        }

        throw new IllegalArgumentException("Character not valid for priority calculation");
    }

    public int getArrangementScore() {
        var score = 0;
        for (var rucksack : rucksacks) {
            var chars = rucksack.findEqualCharacters();
            for (var c : chars) {
                score += getPriority(c);
            }
        }

        return score;
    }

    public static DayThree parseFromInput(Reader input) throws IOException {
        var result = new DayThree();
        var buffered = new BufferedReader(input);
        var read = buffered.readLine();

        while (read != null) {
            var rucksack = new Rucksack();
            rucksack.populateCompartments(read.toCharArray());
            result.rucksacks.add(rucksack);

            read = buffered.readLine();;
        }

        return result;
    }

    public static DayThree parseFromString(String input) throws IOException {
        return parseFromInput(new StringReader(input));
    }
}

