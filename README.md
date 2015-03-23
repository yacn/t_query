# t_query

## TODO:

* Partial station name matching / disambiguation, ex:

    - `Airport` finds `Airport Station`

    - `Center` requires disambiguating between `Quincy Center Station`,
    `Government Center Station`, `Tufts Medical Center Station`,
    and `Malden Center Station`

* Concurrency, e.g. the graph can be passed between multiple threads

* Enable/disable stations

    - Will need a queue of operations to perform

        * e.g. `["disable foo", "from bar to baz", "enable foo", ...]`
