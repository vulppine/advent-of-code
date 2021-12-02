(ns advent-of-code.day-two
  (:require [advent-of-code.util :as ut])
  (:require [clojure.string :as st]))

; we do a little language features
; it would be fun to just do it strictly
; as a list, ala '(horizontal depth aim)
(defrecord Position [horizontal depth aim])

; really hard to do this via REPL so here's
; a function specifically for it
(defn new-position [] (->Position 0 0 0))

(defn update-position [position cmd]
  "Updates a position according to the command.
  Commands take the form of (command amount)."
  (cond
    (= (first cmd) 'forward)
      (assoc position
             :horizontal
             (+ (:horizontal position) (second cmd)))
    (= (first cmd) 'down)
      (assoc position
             :depth
             (+ (:depth position) (second cmd)))
    (= (first cmd) 'up)
      (assoc position
             :depth
             (- (:depth position) (second cmd)))))

(defn update-position-aim [position cmd]
  "Updates a position similar to update-position,
  but takes into account aim as well."
  (cond
    (= (first cmd) 'forward)
      (assoc position
             :horizontal (+ (:horizontal position) (second cmd))
             :depth (+ (:depth position) (* (:aim position) (second cmd))))
    (= (first cmd) 'down)
      (assoc position
             :aim
             (+ (:aim position) (second cmd)))
    (= (first cmd) 'up)
      (assoc position
             :aim
             (- (:aim position) (second cmd)))))

(defn string-to-cmd [string]
  "Takes a string, converts it into a command."
  (ut/string-list-parse (seq (st/split string #" "))))

(defn file-to-cmds [file-name]
  "Takes a file name, opens it, and
  (attempts) to map everything to
  valid commands used by update-position."
  (let [file (ut/file-as-list file-name)]
    (map string-to-cmd file)))

(defn run []
  "Entry point to a code challenge!"
  (let [cmds (file-to-cmds "resources/day_two.input")]
    (let [position (reduce update-position
                           (new-position)
                           cmds)
          position-aim (reduce update-position-aim
                               (new-position)
                               cmds)]
      (println (* (:horizontal position) (:depth position)))
      (println (* (:horizontal position-aim) (:depth position-aim))))))
