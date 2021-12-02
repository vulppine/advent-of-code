(ns advent-of-code.util
  (:require [clojure.java.io :as io])
  (:require [clojure.string :as st])
  (:require [clojure.edn :as edn]))

(defn file-as-list [file-name]
  (let [file (io/file file-name)]
    (seq (st/split-lines (slurp file)))))

(defn string-list-parse [string-list]
  (map edn/read-string string-list))
