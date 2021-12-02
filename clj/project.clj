(defproject advent-of-code "2021"
  :description "Advent of Code 2021 - Flipp Syder"
  :url "http://adventofcode.com"
  :license {:name "MIT"}
  :dependencies [[org.clojure/clojure "1.10.3"]]
  :main ^:skip-aot advent-of-code.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
