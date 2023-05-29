
# Simple Trader Backtest Engine

## Description

This is an all-in-one command line interface tool for fast prototyping of automated trading strategies. It is designed to be a one-stop shop for all the tools you might need to quickly test a trading strategy on historic data. It is built in Rust and is designed to be fast and efficient, both for developing the strategy and for running it on large datasets. It has a built-in backtester, provides a simple way for using Lua scripts to define strategies and indicators.

## Features

- Built-in backtester
- Pre-built indicators
- JITLua (JIT compilation)
- Highly optimized data storage engine (STMDB - Simple Trader Market Data Base)
- Integrated data retrieval from a number of sources (easily extendable)

## How it works

1. We retrieve data from a data source (e.g. Yahoo Finance)
2. We ingest the data in a database (STMDB)
3. We run a strategy on the data from our database
4. We dynamically load and indicators or plugins that we need
5. We run the strategy and output the results

FUTURE: adding result plotting

## Commands

### Reload plugins

We can reload all the Lua plugins (indicators, strategies, etc.) by running the following command:

    ```bash
    /reloadplugins
    ``` 

## References

## License  

This project is under the MIT license. See the [LICENSE](LICENSE) file for more details.

## Author

[**Ryan Coble**](
    https://www.linkedin.com/in/rcoble
)

[**Personal Website**](
    https://www.ryanmcoble.com
)
