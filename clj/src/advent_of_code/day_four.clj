(ns advent-of-code.day-four
  (:require [advent-of-code.util :as ut])
  (:require [clojure.string :as st]))

;;; directionals
(defn new-position
  ([] (list 0 0))
  ([x y] (list x y)))

(defn pos-x [pos] (first pos))
(defn pos-y [pos] (second pos))

(defn move-position [position direction]
  "Moves a position according to the
  given direction."
  (cond (= direction 'north)
          (list (first position) (dec (second position)))
        (= direction 'south)
          (list (first position) (inc (second position)))
        (= direction 'west)
          (list (dec (first position)) (second position))
        (= direction 'east)
          (list (inc (first position)) (second position))))

(defn get-xy-from-direction [position direction]
  "Gives the x or y based on the direction
  given. north/south give y, west/east give x."
  (cond (or (= direction 'north) (= direction 'south)) (pos-y position)
        (or (= direction 'west) (= direction 'east)) (pos-x position)))

;;; bingo cells
(def BingoCell '(0 false))

(defn cell-number [cell] (first cell))
(defn cell-mark [cell] (second cell))

(defn new-cell [number]
  "Create a cell with a number."
  (cons number (list false)))

(defn toggle-cell [cell]
  "Toggle a cell for it being marked or not."
  (list (first cell) (not (second cell))))

(defn vec-to-bingocells [coll]
  "Converts a vector of numbers to BingoCells."
  (mapv new-cell coll))

;;; bingo tables

; similar approach to the rust method,
; as i still want to do some things in O(1)
; time

; this means we're using vectors and maps baby!!!
; as well as records!!!
(defrecord BingoTable [rows search size has-bingo])

(defn new-bingo-table []
  (map->BingoTable {:rows []
                    :size nil
                    :search {}
                    :has-bingo false}))

(defn build-search-from-row
  "Build the search table based on
  the current row. Consumes the row."
  ([table row]
   (build-search-from-row table
                          row
                          (count (:rows table))
                          0))
  ([table row cur-row cur-pos]
    (if (= (count row) 0)
      table
      (recur (assoc table
                    :search
                    (assoc (:search table)
                           (cell-number (first row))
                           (new-position cur-pos cur-row)))
             (rest row)
             cur-row
             (inc cur-pos)))))

(defn insert-row-into-table [table row]
  "Insert a row into a bingo table."
  (if (or (not (= (count row) (:size table)))
          (= (count (:rows table)) (:size table)))
    (println "incorrect size")
    (assoc (build-search-from-row table row)
           :rows (conj (:rows table)
                       row))))

(defn get-cell-from-table-position [table position]
  "Get a cell from a position on the table."
  (get (get (:rows table)
            (pos-y position))
       (pos-x position)))

(defn get-cell-from-table-number [table number]
  "Get a cell from a number on the table."
  (let [position (get (:search table) number)]
    (get-cell-from-table-position table position)))

(defn get-position-from-table-number [table number]
  "Get a cell's position from a number on the table."
  (get (:search table) number))

(defn mark-cell-at-position [table position]
  (assoc table
         :rows
         (assoc-in (:rows table)
                   [(pos-y position) (pos-x position)]
                   (toggle-cell (get-cell-from-table-position table
                                                              position)))))

(defn mark-cell-at-number [table number]
  (let [position (get-position-from-table-number table number)]
    (if (nil? position)
      table
      (mark-cell-at-position table (get-position-from-table-number table number)))))

(defn check-bingo-amount
  "Checks bingo along the given direction.
  Continues until an edge is reached,
  or the current position is not marked.
  Returns the amount of tiles counted
  once one of the base cases is reached."
  ([table cur-pos direction]
   (check-bingo-amount table cur-pos direction 0))
  ([table cur-pos direction amount]
    (if (or (< (get-xy-from-direction cur-pos direction)
               0)
            (> (get-xy-from-direction cur-pos direction)
               (dec (:size table))))
      amount
      (if (not (cell-mark (get-cell-from-table-position table cur-pos)))
        amount
        (check-bingo-amount table (move-position cur-pos direction) direction (inc amount))))))

(defn check-bingo-from-position [table position]
  "Checks if a bingo exists from a specific
  position. Returns the table, updated with
  has-bingo set to true if it is, otherwise
  just returns the table."
  (if (not (cell-mark (get-cell-from-table-position table position)))
    table
    (let [horz (+ (dec (check-bingo-amount table position 'west))
                  (dec (check-bingo-amount table position 'east))
                  1)
          vert (+ (dec (check-bingo-amount table position 'north))
                  (dec (check-bingo-amount table position 'south))
                  1)
          size (:size table)]
      (if (or (= horz size) (= vert size))
        (assoc table :has-bingo true)
        table))))

(defn tables-from-coll
  "Creates a collection of bingo tables
  from a collection. split-by determines
  when to create a new bingo table. Assumes
  that collection is a set of rows split
  by some object. Takes the size of the
  new table based on the first row it
  collects."
  ([coll split-by]
   (tables-from-coll coll split-by (list (new-bingo-table))))
  ([coll split-by tables]
    (if (empty? coll)
      tables
      (if (= (first coll) split-by)
        (recur (rest coll) split-by (cons (new-bingo-table) tables))
        (if (nil? (:size (first tables)))
          (recur coll
                 split-by
                 (cons (assoc (first tables)
                              :size
                              (count (first coll)))
                       (rest tables)))
          (recur (rest coll)
                 split-by
                 (cons (insert-row-into-table (first tables)
                                              (vec-to-bingocells (vec (first coll))))
                       (rest tables))))))))

; bingo game
; now we're in the end zone
(defrecord Bingo [draw-order tables])

(defn parse-from-file [file]
  "Parses a bingo game from file."
  (let [input (ut/file-as-seq file)]
    (->Bingo (ut/string-list-parse (st/split (first input) #","))
             (tables-from-coll
               (map #(if (= %1 "")
                     '()
                     (ut/string-list-parse (st/split (st/trim %1) #"\s+")))
                     (rest input))
               '() '()))))

(defn check-tables-for-bingo [tables]
  "Checks all tables if there is a bingo.
  Returns nil if no tables have a bingo."
  (if (empty? tables)
    nil
    (if (true? (:has-bingo (first tables)))
      (first tables)
      (recur (rest tables)))))

(defn print-bingo-table [rows]
  (if (empty? rows)
    (do (println))
    (do (println (first rows))
        (recur (rest rows)))))

(defn print-bingo-tables [tables]
  (if (empty? tables)
    (do (println "-----"))
    (do (print-bingo-table (:rows (first tables)))
        (recur (rest tables)))))

(defn play-bingo
  "Plays bingo. Bingo!"
  ([bingo] (play-bingo (:draw-order bingo) (:tables bingo)))
  ([draw-order tables]
   (let [marked-tables (map #(check-bingo-from-position %1
                                                        (get-position-from-table-number %1
                                                                                        (first draw-order)))
                            (map #(mark-cell-at-number %1 (first draw-order)) tables))]
     (let [winning-table (check-tables-for-bingo marked-tables)]
      (if (nil? winning-table)
        (recur (rest draw-order) marked-tables)
        (list winning-table (first draw-order)))))))

(defn play-least-bingo
  "Plays least bingo. Bingo...?"
  ([bingo] (play-least-bingo (:draw-order bingo) (:tables bingo)))
  ([draw-order tables]
   (let [marked-tables (map #(check-bingo-from-position %1
                                                        (get-position-from-table-number %1
                                                                                        (first draw-order)))
                            (map #(mark-cell-at-number %1 (first draw-order)) tables))]
     (let [filtered-tables (filter #(not (:has-bingo %1)) marked-tables)]
       (if (or (empty? filtered-tables))
         (list (last marked-tables) (first draw-order))
         (recur (rest draw-order) filtered-tables))))))


(defn get-unmarked-from-table-rows
  "Gets the unmarked spots from a table."
  ([rows] (get-unmarked-from-table-rows rows '()))
  ([rows spots]
   (if (empty? rows)
     (flatten spots)
     (recur (rest rows)
            (cons (map cell-number (filter #(not (cell-mark %1)) (first rows)))
             spots)))))

(defn calculate-table-score
  "Calculate a table's score."
  ([results] (calculate-table-score (first results) (second results)))
  ([table last-number]
    (* (reduce + (get-unmarked-from-table-rows (:rows table))) last-number)))

(defn run
  "Run a coding challenge!"
  ([] (run "resources/day_four.input"))
  ([& opts]
   (let [input (parse-from-file (first opts))]
     (println (calculate-table-score (play-bingo input)))
     (println (calculate-table-score (play-least-bingo input))))))
