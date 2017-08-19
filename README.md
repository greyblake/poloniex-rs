# Poloniex

Rust client for [Poloniex API](https://poloniex.com/support/api/).

## Roadmap

TODO:
* Add public api methods to Client (use macro?)


Public API:
* [x] returnTicker
* [x] returnOrderBook
* [x] returnTradeHistory
* [x] returnChartData
* [x] returnCurrencies
* [x] returnLoanOrders
* [ ] return24Volume

Trading API:
* [x] returnBalances
* [x] buy
* [x] sell
* [x] returnOpenOrders
* [ ] **cancelOrder**
* [ ] **moveOrder**
* [ ] **returnCompleteBalances**
* [ ] returnDepositAddresses
* [ ] generateNewAddress
* [ ] returnDepositsWithdrawals
* [ ] returnTradeHistory
* [ ] returnOrderTrades
* [ ] withdraw
* [ ] returnFeeInfo
* [ ] returnAvailableAccountBalances
* [ ] returnTradableBalances
* [ ] transferBalance
* [ ] returnMarginAccountSummary
* [ ] marginBuy
* [ ] marginSell
* [ ] getMarginPosition
* [ ] closeMarginPosition
* [ ] createLoanOffer
* [ ] cancelLoanOffer
* [ ] returnOpenLoanOffers
* [ ] returnActiveLoans
* [ ] returnLendingHistory
* [ ] toggleAutoRenew

## License

[MIT](https://github.com/greyblake/whatlang-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)


## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
