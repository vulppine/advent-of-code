package com.adventofcode.challenges.daythree;

import java.io.*;
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

    public int getGroupScores() {
        var groupScores = 0;
        for (var i = 0; i <= rucksacks.size() - 3; i = i + 3) {
            groupScores += sumGroupScore(i, i + 3);
        }

        return groupScores;
    }

    public int sumGroupScore(int start, int end) {
        var resultSet = new HashSet<>(rucksacks.get(start).unionCompartments());
        for (var i = start + 1; i < end; i++) {
            resultSet.retainAll(rucksacks.get(i).unionCompartments()) ;
        }

        var resultScore = 0;
        for (var c : resultSet) {
            resultScore += getPriority(c);
        }

        return resultScore;
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

    public static DayThree parseFromFile(String path) throws IOException {
        return parseFromInput(new FileReader(path));
    }
}

