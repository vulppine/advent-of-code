package com.adventofcode.challenges.daythree;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Rucksack {
    private Set<Character> firstCompartment = new HashSet<>();
    private Set<Character> secondCompartment = new HashSet<>();

    public List<Character> findEqualCharacters() {
        var copy = new HashSet<>(firstCompartment);
        copy.retainAll(secondCompartment);

        return copy.stream().toList();
    }

    // OOP-y way to do it would be to decorate or w/e tbh
    public void populateCompartments(char[] chars) {
        // it would be NICE if we had SLICES
        for (var i = 0; i < chars.length / 2; i++) {
            firstCompartment.add(chars[i]);
        }

        for (var i = chars.length / 2; i < chars.length; i++) {
            secondCompartment.add(chars[i]);
        }
    }
}
