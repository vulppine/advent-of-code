(ns advent-of-code.day-three
  (:require [advent-of-code.util :as ut])
  (:require [clojure.string :as st]))

(defn num-to-bits
  "Turns a number into a sequence
  of true-falses. Requires a length."
  ([number len]
   (num-to-bits number len '() 0))
  ([number len coll cur-bit]
    (if (= cur-bit len)
      coll
      (let [bit (bit-and (bit-shift-right number cur-bit) 1)]
        (if (= bit 0)
            (recur number len (cons false coll) (inc cur-bit))
            (recur number len (cons true coll) (inc cur-bit)))))))

(defn bool-to-tseq [bool]
  "Turns a bool into a truth seq
  (i.e., 0/1 table).
  (1, 0) is false,
  (0, 1) is true."
  (if bool
    '(0 1)
    '(1 0)))

(defn coll-nums-to-bits [coll len]
  "Converts a seq of numbers to true-false bits.
  Requires a length to care about."
  (map #(num-to-bits % len) coll))

(defn coll-bits-to-tseqs [coll]
  "Converts collections of true-false bits
  into truth seqs. This could probably
  be implemented without all the abstraction."
  (map #(map bool-to-tseq %) coll))

; these two are NOT lazy eval'd:
; i feel like that there should be some
; way to get 'columns' from a seq without
; doing apply
(defn add-seqs [& seqs]
  "Adds seqs together."
  (apply map + seqs))

(defn count-bits-in-nums [coll len]
  "Counts the number of bits in a collection
  of numbers per field, into a collection that
  represents the number of 0/1 bits per field.
  Requires a length to care about."
  (apply map add-seqs (coll-bits-to-tseqs (coll-nums-to-bits coll len))))

(defn count-bits-in-bitseq [coll]
  "Counts the number of bits in a collection
  of bitseqs per field, into a collection that
  represents the number of 0/1 bits per field.

  Smaller version of count-bits-in-nums."
  (apply map add-seqs (coll-bits-to-tseqs coll)))

(defn get-common-bits [coll len]
  "Gets the most common bit per field,
  represented as false for 0, true for 1.
  Returns a sequence."
  (map #(> (first %) (second %)) (count-bits-in-nums coll len)))

(defn get-lcommon-bits [coll len]
  "Gets the least common bit per field,
  represented as false for 0, true for 1.
  Returns a sequence."
  (map not (get-common-bits coll len)))

(defn bit-seq-to-num [coll]
  "Reduces a true/false bit sequence
  into a number."
  (reduce #(bit-or (bit-shift-left %1 1) (if %2 1 0)) 0 coll))

(defn filter-bit-seqs-by-nth [coll n truth-filter]
  "Filter true/false bit seqs
  based on the nth bit
  (zero-indexed)
  being true or false.

  coll is the collection,
  truth-filter is whether
  to filter true or false into
  the result."
  (filter #(= (nth %1
                   (- (count %1)
                      1
                      n))
              truth-filter)
          coll))


(defn gas-rating
  "AoC challenge, part two.
  I don't know if there's an actual
  name for this, so we're going
  with the challenge context.

  Takes a collection of bit-seqs.
  Returns the last bit-seq left
  in decimal."
  ([coll len gas-type] (gas-rating coll len gas-type 0))
  ([coll len gas-type cur]
    (if (or (<= (count coll) 1) (> cur len))
      (bit-seq-to-num (first coll))
      (let [bit-count (count-bits-in-bitseq coll)]
        (let [nth-bit (nth bit-count cur)]
          (let [bit-truthy (cond (> (first nth-bit) (second nth-bit)) false
                                 (< (first nth-bit) (second nth-bit)) true
                                 (= (first nth-bit) (second nth-bit)) true)]
            (recur (filter-bit-seqs-by-nth coll
                                           (- len 1 cur)
                                           (cond (= gas-type 'o2) bit-truthy
                                                 (= gas-type 'co2) (not bit-truthy)))
                   len
                   gas-type
                   (inc cur))))))))

(defn o2-rating [coll len] (gas-rating coll len 'o2))
(defn co2-rating [coll len] (gas-rating coll len 'co2))

(defn run
  "Entry point to a code challenge!"
  ([] (run "resources/day_three.input"))
  ([& opts]
  (let [file (ut/file-as-list (first opts))]
    (let [len (count (first file))
          input (ut/string-list-parse (map #(st/join (list "2r" %)) file))]
      (println (* (bit-seq-to-num (get-common-bits input len))
                  (bit-seq-to-num (get-lcommon-bits input len))))
      (println (* (o2-rating (coll-nums-to-bits input len) len)
                  (co2-rating (coll-nums-to-bits input len) len)))))))
