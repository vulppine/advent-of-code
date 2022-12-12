package com.adventofcode.challenges.daytwo;

import java.io.*;
import java.nio.Buffer;
import java.util.ArrayList;

public class DayTwo {
    private ArrayList<GameRound> rounds = new ArrayList<>();

    public int getScore() {
        var result = 0;
        for (var round : rounds) {
            result += round.getScore();
        }

        return result;
    }

    public static DayTwo parseStrategyGuide(Reader reader, GameStrategyParser parser) throws IOException {
        var buffered = new BufferedReader(reader);
        var line = buffered.readLine();
        var result = new DayTwo();

        while (line != null) {
            if (line.length() != 3) {
                throw new IllegalArgumentException("Too many characters in line");
            }

            var round = parser.getStrategy(line.charAt(0), line.charAt(2));


            result.rounds.add(round);

            line = buffered.readLine();
        }

        return result;
    }

    public static DayTwo parseStrategyGuideFromString(String string, GameStrategyParser parser) throws IOException {
        return parseStrategyGuide(new StringReader(string), parser);
    }

    public static DayTwo parseStrategyGuideFromFile(String path, GameStrategyParser parser) throws IOException {
        var file = new File(path);

        return parseStrategyGuide(new FileReader(file), parser);
    }
}