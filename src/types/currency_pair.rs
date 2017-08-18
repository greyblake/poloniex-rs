use std::fmt;
use super::Currency;
use super::Currency::*;
use self::CurrencyPair::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurrencyPair {
    BtcAmp,
    BtcArdr,
    BtcBch,
    BtcBcn,
    BtcBcy,
    BtcBela,
    BtcBlk,
    BtcBtcd,
    BtcBtm,
    BtcBts,
    BtcBurst,
    BtcClam,
    BtcDash,
    BtcDcr,
    BtcDgb,
    BtcDoge,
    BtcEmc2,
    BtcEtc,
    BtcEth,
    BtcExp,
    BtcFct,
    BtcFldc,
    BtcFlo,
    BtcGame,
    BtcGno,
    BtcGnt,
    BtcGrc,
    BtcHuc,
    BtcLbc,
    BtcLsk,
    BtcLtc,
    BtcMaid,
    BtcNaut,
    BtcNav,
    BtcNeos,
    BtcNmc,
    BtcNote,
    BtcNxc,
    BtcNxt,
    BtcOmni,
    BtcPasc,
    BtcPink,
    BtcPot,
    BtcPpc,
    BtcRads,
    BtcRep,
    BtcRic,
    BtcSbd,
    BtcSc,
    BtcSjcx,
    BtcSteem,
    BtcStr,
    BtcStrat,
    BtcSys,
    BtcVia,
    BtcVrc,
    BtcVtc,
    BtcXbc,
    BtcXcp,
    BtcXem,
    BtcXmr,
    BtcXpm,
    BtcXrp,
    BtcXvc,
    BtcZec,
    EthBch,
    EthEtc,
    EthGno,
    EthGnt,
    EthLsk,
    EthRep,
    EthSteem,
    EthZec,
    UsdtBch,
    UsdtBtc,
    UsdtDash,
    UsdtEtc,
    UsdtEth,
    UsdtLtc,
    UsdtNxt,
    UsdtRep,
    UsdtStr,
    UsdtXmr,
    UsdtXrp,
    UsdtZec,
    XmrBcn,
    XmrBlk,
    XmrBtcd,
    XmrDash,
    XmrLtc,
    XmrMaid,
    XmrNxt,
    XmrZec
}

impl CurrencyPair {
    pub fn primary_currency(&self) -> Currency {
        self.currencies().0
    }

    pub fn secondary_currency(&self) -> Currency {
        self.currencies().1
    }

    pub fn currencies(&self) -> (Currency, Currency) {
        match *self {
            BtcAmp => (Btc, Amp),
            BtcArdr => (Btc, Ardr),
            BtcBch => (Btc, Bch),
            BtcBcn => (Btc, Bcn),
            BtcBcy => (Btc, Bcy),
            BtcBela => (Btc, Bela),
            BtcBlk => (Btc, Blk),
            BtcBtcd => (Btc, Btcd),
            BtcBtm => (Btc, Btm),
            BtcBts => (Btc, Bts),
            BtcBurst => (Btc, Burst),
            BtcClam => (Btc, Clam),
            BtcDash => (Btc, Dash),
            BtcDcr => (Btc, Dcr),
            BtcDgb => (Btc, Dgb),
            BtcDoge => (Btc, Doge),
            BtcEmc2 => (Btc, Emc2),
            BtcEtc => (Btc, Etc),
            BtcEth => (Btc, Eth),
            BtcExp => (Btc, Exp),
            BtcFct => (Btc, Fct),
            BtcFldc => (Btc, Fldc),
            BtcFlo => (Btc, Flo),
            BtcGame => (Btc, Game),
            BtcGno => (Btc, Gno),
            BtcGnt => (Btc, Gnt),
            BtcGrc => (Btc, Grc),
            BtcHuc => (Btc, Huc),
            BtcLbc => (Btc, Lbc),
            BtcLsk => (Btc, Lsk),
            BtcLtc => (Btc, Ltc),
            BtcMaid => (Btc, Maid),
            BtcNaut => (Btc, Naut),
            BtcNav => (Btc, Nav),
            BtcNeos => (Btc, Neos),
            BtcNmc => (Btc, Nmc),
            BtcNote => (Btc, Note),
            BtcNxc => (Btc, Nxc),
            BtcNxt => (Btc, Nxt),
            BtcOmni => (Btc, Omni),
            BtcPasc => (Btc, Pasc),
            BtcPink => (Btc, Pink),
            BtcPot => (Btc, Pot),
            BtcPpc => (Btc, Ppc),
            BtcRads => (Btc, Rads),
            BtcRep => (Btc, Rep),
            BtcRic => (Btc, Ric),
            BtcSbd => (Btc, Sbd),
            BtcSc => (Btc, Sc),
            BtcSjcx => (Btc, Sjcx),
            BtcSteem => (Btc, Steem),
            BtcStr => (Btc, Str),
            BtcStrat => (Btc, Strat),
            BtcSys => (Btc, Sys),
            BtcVia => (Btc, Via),
            BtcVrc => (Btc, Vrc),
            BtcVtc => (Btc, Vtc),
            BtcXbc => (Btc, Xbc),
            BtcXcp => (Btc, Xcp),
            BtcXem => (Btc, Xem),
            BtcXmr => (Btc, Xmr),
            BtcXpm => (Btc, Xpm),
            BtcXrp => (Btc, Xrp),
            BtcXvc => (Btc, Xvc),
            BtcZec => (Btc, Zec),
            EthBch => (Eth, Bch),
            EthEtc => (Eth, Etc),
            EthGno => (Eth, Gno),
            EthGnt => (Eth, Gnt),
            EthLsk => (Eth, Lsk),
            EthRep => (Eth, Rep),
            EthSteem => (Eth, Steem),
            EthZec => (Eth, Zec),
            UsdtBch => (Usdt, Bch),
            UsdtBtc => (Usdt, Btc),
            UsdtDash => (Usdt, Dash),
            UsdtEtc => (Usdt, Etc),
            UsdtEth => (Usdt, Eth),
            UsdtLtc => (Usdt, Ltc),
            UsdtNxt => (Usdt, Nxt),
            UsdtRep => (Usdt, Rep),
            UsdtStr => (Usdt, Str),
            UsdtXmr => (Usdt, Xmr),
            UsdtXrp => (Usdt, Xrp),
            UsdtZec => (Usdt, Zec),
            XmrBcn => (Xmr, Bcn),
            XmrBlk => (Xmr, Blk),
            XmrBtcd => (Xmr, Btcd),
            XmrDash => (Xmr, Dash),
            XmrLtc => (Xmr, Ltc),
            XmrMaid => (Xmr, Maid),
            XmrNxt => (Xmr, Nxt),
            XmrZec => (Xmr, Zec)
        }
    }
}

impl fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (c1, c2) = self.currencies();
        write!(f, "{}_{}", c1, c2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let s = format!("{}", CurrencyPair::BtcEth);
        assert_eq!(s, "BTC_ETH");
    }

    #[test]
    fn test_primary_currency() {
        assert_eq!(XmrLtc.primary_currency(), Xmr);
        assert_eq!(EthGno.primary_currency(), Eth);
    }

    #[test]
    fn test_secondary_currency() {
        assert_eq!(XmrLtc.secondary_currency(), Ltc);
        assert_eq!(EthGno.secondary_currency(), Gno);
    }
}
