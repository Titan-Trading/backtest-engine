function on_bar(bar)

    -- Get the last bar
    -- local last_bar = get_last_bar()

    -- Get the last 10 bars
    -- local last_10_bars = get_last_bars(10)

    -- Get the last 10 bars of the 1 minute timeframe
    -- local last_10_bars_1m = get_last_bars(10, "1m")

    -- Get the last 10 bars of the 1 minute timeframe of the EURUSD symbol
    -- local last_10_bars_1m_eurusd = get_last_bars(10, "1m", "EURUSD")

    -- Get the RSI indicator of the last 10 bars of the 1 minute timeframe of the EURUSD symbol
    local rsi = get_rsi(14, "1m", "EURUSD")

    -- Get the EMAs of the last 10 bars of the 1 minute timeframe of the EURUSD symbol
    -- local emas = get_ema(10, "1m", "EURUSD")

    -- Execute a market order
    -- local order = execute_market_order("EURUSD", 0.01, "buy")

    -- Execute a limit order
    -- local order = execute_limit_order("EURUSD", 0.01, "buy", 1.1)

    -- Get orders on the exchange
    -- local orders = get_orders()

    -- Get orders on the exchange for the EURUSD symbol
    -- local orders = get_orders("EURUSD")

    -- If RSI is below 30 and we don't have a position, buy
    if rsi < 30 and not get_position("EURUSD") then
        execute_market_order("EURUSD", 0.01, "buy")
    end
    -- If RSI is above 70 and we have a position, sell
    if rsi > 70 and get_position("EURUSD") then
        execute_market_order("EURUSD", 0.01, "sell")
    end

    print("on_bar")

end