# t_query

## TODO:

* Enable/disable stations

* Concurrency, e.g. the graph can be passed between multiple threads

    - Use Channels

        * Receiver will handle receiving commands, running command, sending
        results back on provided channel.

        * Senders send commands _and_ a receiving channel to receive their
        results on.

    - Will need a queue of operations to perform

        * e.g. `["disable foo", "from bar to baz", "enable foo", ...]`

