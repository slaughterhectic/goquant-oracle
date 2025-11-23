CREATE TABLE IF NOT EXISTS price_history (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    price DECIMAL(20, 10) NOT NULL,
    confidence DECIMAL(20, 10) NOT NULL,
    source VARCHAR(20) NOT NULL, -- 'pyth', 'switchboard', 'aggregated'
    timestamp BIGINT NOT NULL
);

CREATE INDEX idx_symbol_time ON price_history(symbol, timestamp DESC);