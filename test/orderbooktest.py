import asyncio
import websockets
import json

async def connect_to_exchange(ws_url, subscription_message):
    async with websockets.connect(ws_url) as websocket:
        # Subscribe to the order book channel
        await websocket.send(json.dumps(subscription_message))

        # Handle incoming messages
        while True:
            message = await websocket.recv()
            print(f"Received message: {message}")
binance_ws_url = "wss://stream.binance.com:9443/ws/btcusdt@depth"

coinbase_ws_url = "wss://ws-feed-public.sandbox.exchange.coinbase.com"
coinbase_subscription_message = {
    "type": "subscribe",
    "channels": [{"name": "level2", "product_ids": ["BTC-USD"]}]
}

kraken_ws_url = "wss://ws.kraken.com"
kraken_subscription_message = {
    "event": "subscribe",
    "pair": ["XBT/USD"],
    "subscription": {"name": "book"}
}
bitfinex_ws_url = "wss://api-pub.bitfinex.com/ws/2"
bitfinex_subscription_message = {
    "event": "subscribe",
    "channel": "book",
    "symbol": "tBTCUSD"
}
okx_ws_url = "wss://ws.okx.com:8443/ws/v5/public"
okx_subscription_message = {
    "op": "subscribe",
    "args": [{
        "channel": "books",
        "instId": "BTC-USDT"
    }]
}
huobi_ws_url = "wss://api.huobi.pro/ws"
huobi_subscription_message = {
    "sub": "market.btcusdt.depth.step0",
    "id": "id1"
}
bybit_ws_url = "wss://stream.bybit.com/spot/public/v3"
bybit_subscription_message = {
    "op": "subscribe",
    "args": ["orderbook.40.BTCUSDT"]
}
gateio_ws_url = "wss://api.gateio.ws/ws/v4/"
gateio_subscription_message = {
    "time": 1605171643,
    "channel": "spot.order_book",
    "event": "subscribe",
    "payload": ["BTC_USDT", "20", "100ms"]
}


async def main():
    await asyncio.gather(
        #connect_to_exchange(binance_ws_url, {}),
        #connect_to_exchange(coinbase_ws_url, coinbase_subscription_message),
        connect_to_exchange(kraken_ws_url, kraken_subscription_message)
        #connect_to_exchange(bitfinex_ws_url, bitfinex_subscription_message)
        #connect_to_exchange(okx_ws_url, okx_subscription_message)
        #connect_to_exchange(huobi_ws_url, huobi_subscription_message)
        #connect_to_exchange(bybit_ws_url, bybit_subscription_message)
        #connect_to_exchange(gateio_ws_url, gateio_subscription_message)
    )

asyncio.run(main())
