package com.adventofcode.challenges.dayone;

import java.io.*;
import java.util.ArrayList;
import java.util.List;

public final class DayOne {
    private List<Integer> calories = new ArrayList<>();

    public Integer getTopThreeHighestCalories() {
        calories.sort(Integer::compare);

        var result = 0;
        for (var i = 1; i <= 3; i++) {
            result += calories.get(calories.size() - i);
        }

        return result;
    }

    public Integer getHighestCalories() {
        // lmao, holy shit shio this syntax sugar works deadass
        calories.sort(Integer::compare);

        return calories.get(calories.size() - 1);
    }

    public static DayOne parseCalories(Reader reader) throws IOException {
        var buffered = new BufferedReader(reader);
        var result = new DayOne();
        var current = 0;
        var line = buffered.readLine();

        while (line != null) {
            if (line.isEmpty()) {
                result.calories.add(current);
                current = 0;
            } else {
                current += Integer.parseInt(line);
            }

            line = buffered.readLine();
        }

        return result;
    }

    public static DayOne parseCalories(File file) throws IOException {
        var reader = new FileReader(file);

        return parseCalories(reader);
    }

    public static DayOne parseCalories(String string) throws IOException {
        var reader = new StringReader(string);

        return parseCalories(reader);
    }
}

