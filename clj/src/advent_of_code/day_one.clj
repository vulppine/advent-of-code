(ns advent-of-code.day-one
  (:require [advent-of-code.util :as ut]))

; this will *attempt* to be more functional
; than the rust end of things, the rust
; end of this is to get a syntax refresher:
; the real challenge is in the Clojure.......

; since i don't know every single clojure tool,
; the entry point is in run

(defn delta-increase
  "Check if the delta between the head of the list
  and the head of the rest has increased. If so,
  increment incr. Returns incr if the next head
  of the list is nil."
  ([coll] (delta-increase coll 0))
  ([coll, incr]
    (if (nil? (second coll))
      incr
      (if (< (first coll) (second coll))
        (recur (rest coll) (+ incr 1))
        (recur (rest coll) incr)))))

(defn delta-3-increase
  "Check the delta of the sum of the first 3 items,
  compared to the next 3 items. If the next set is
  less than three, the increment count is returned."
  ([coll] (delta-3-increase coll 0))
  ([coll, incr]
    (let [a (take 3 coll)
          b (take 3 (rest coll))]
      (if (< (count b) 3)
        incr
        (if (< (reduce + a) (reduce + b))
          (recur (rest coll) (+ incr 1))
          (recur (rest coll) incr))))))

(defn run []
  "Entry point for a code challenge"
  (let [input (ut/string-list-parse (ut/file-as-list "resources/day_one.input"))]
    (println (delta-increase input))
    (println (delta-3-increase input))))
