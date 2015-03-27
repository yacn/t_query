# Homework 7: Running the T

## Files and Folders

* `src/'
  - `subway/'
    * `data.rs' - data related subway functions
    * `mod.rs' - main file for subway module, contains Subway struct & defs
    * `route.rs' - route/path related subway functions
  - `lib.rs' - main library file for `t_query'
  - `main.rs' - executable entrypoint for `t_query'
  - `server.rs' - TCP server module for `t_query'
* `blue.dat' - data file for Blue line
* `Cargo.lock' - Contains specific versions required to build `t_query'
* `Cargo.toml' - Project definition file
* `green.dat' - data file for Green line
* `orange.dat' - data file for Orange line
* `red.dat' - data file for Red line
* `run.sh' - convenience script to run `t_query' with included subway line data
