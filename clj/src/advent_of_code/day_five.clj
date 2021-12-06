(ns advent-of-code.day-five
  (:require [advent-of-code.util :as ut])
  (:require [clojure.string :as st]))

; once again, similar to the rust method
(defn new-coordinate [x y] (list x y))

(defn coord-x [c] (first c))
(defn coord-y [c] (second c))

(defn do-on-coords [a b f]
  "Does something to a pair of coordinates.
  Totally isn't a map abstraction."
  (map f a b))

(defn coordinate-delta [a b]
  "Gets the delta between two
  coordinates."
  (do-on-coords b a -))

(defn coordinate-abs-delta [a b]
  "Gets the absolute delta between
  two coordinates."
  (map #(Math/abs %1) (coordinate-delta a b)))

(defn coordinate-dir [a b]
  "Gets a coordinate's direction
  relative to another coordinate.
  Calculated using compares."
  (do-on-coords a b compare))

(defn coordinate-eq? [a b]
  "Checks if two coordinates are equal."
  (reduce #(and %1 %2) (do-on-coords a b =)))

(defn move-coordinate [a b acc]
  "Moves a coordinate a towards some location b
  at some acceleration."
  (let [dir (coordinate-dir a b)]
    (map #(- %1 (* acc %2)) a dir)))

; vectors
(defrecord Vector [head tail acc])

(defn is-diag [v]
  "Gets if a vector is diagonal (i.e.,
  the coordinate deltas are
  gt zero)"
  (reduce #(and %1 (< 0 %2)) true (coordinate-abs-delta (:head v) (:tail v))))

(defn is-horz [v]
  "Gets if a vector is horizontal -
  this is jut an inversion of is-diag"
  (not (is-diag v)))

(defn is-45 [v]
  "Gets if a vector is a 45 degree angle."
  (= (reduce / (coordinate-abs-delta (:head v) (:tail v))) 1))

(defn process-vector
  "Iterate over a vector, until the
  head is reached. Returns a sequence
  of positions touched down on."
  ([v] (if (and (is-diag v)
                (not (is-45 v)))
          nil ; return nil, no exceptions allowed here!!!
          (process-vector v
                   (:tail v)
                   (coordinate-dir (:tail v)
                                   (:head v))
                   (list (:tail v)))))
  ([v pos dir pos-seq]
   (cond (coordinate-eq? pos
                         (:head v))
          pos-seq
         (not (coordinate-eq? (coordinate-dir pos
                                              (:head v))
                              dir))
          (cons (:head v) pos-seq)
         :else
          (let [new-pos (move-coordinate pos (:head v) (:acc v))]
            (recur v
                   new-pos
                   dir
                   (cons new-pos pos-seq))))))

; attempt #1
(defn map-vector-pos
  "Maps a vector to a map of
  positions, based on the vector's
  acceleration."
  ([v] (map-vector-pos (process-vector v) {}))
  ([pos-seq vmap]
   (if (empty? pos-seq)
     vmap
     (recur (rest pos-seq)
            (assoc vmap
                   (first pos-seq)
                   (let [count (get vmap (first pos-seq))]
                     (if (nil? count) 1 (inc count))))))))

(defn vec-seq-to-pos-map
  "Maps a sequence of vectors to
  a map of positions, with overlap."
  ([vseq] (vec-seq-to-pos-map vseq {}))
  ([vseq vmap]
   (if (empty? vseq)
     (do (println vmap) vmap)
     (recur (rest vseq)
            (merge-with + vmap (map-vector-pos (first vseq)))))))

; attempt #2
(defn vec-seq->pos-seq [vseq]
  "Maps a sequence of vectors to a sequence of
  positions."
  (map process-vector vseq))

(defn add-pos-to-map [pos-map pos]
  "Adds a position to a map. If
  the sequence already exists,
  increments the count instead."
  (update pos-map pos #(if (nil? %1) 1 (inc %1))))

(defn pos-seq->pos-map [pseq]
  "Reduces a sequence of positions
  into a map of positions."
  (reduce add-pos-to-map {} pseq))

(defn vec-seq->pos-map
  "Reduces a sequence of vectors
  into a map of positions."
  ([vseq] (vec-seq->pos-map (vec-seq->pos-seq vseq) {}))
  ([pseq pos-map]
   (if (empty? pseq)
     pos-map
     (recur (rest pseq)
            (reduce add-pos-to-map pos-map (first pseq))))))

(defn vec-from-string [s]
  "Parses a vector from a string."
  (let [coords (map ut/string-list-parse
                    (map #(st/split %1 #",")
                         (st/split s #" -> ")))]
    (->Vector (first coords) (second coords) 1)))

(defn run
  "Run a coding challenge!"
  ([] (run "resources/day_five.input"))
  ([& opts]
   (let [input (map vec-from-string (ut/file-as-seq (first opts)))]
     (println (filter is-horz input))
     (println (count (filter #(> %1 1) (vals (vec-seq->pos-map (filter is-horz input))))))
     (println (count (filter #(> %1 1) (vals (vec-seq->pos-map input))))))))
