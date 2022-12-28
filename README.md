# Binance parser
To run this parser you need docker 
```
docker build -t binance-parser .
docker run --rm --name binance-parser -d -e ASSET=USDT -e PAY_TYPES=LIBERTYBANK -e FIAT=USD -e MIN_VALUE=0.9 -e TG_TOKEN=token -e TG_CHANNEL_ID=-889705140 binance-parser
```
and to stop it
```
docker stop binance-parser
```