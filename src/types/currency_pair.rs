use std::fmt;
use case::CaseExt;
use std::ascii::AsciiExt;

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

// TODO: rework to use pattern matching
impl fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_snake().to_ascii_uppercase();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_display() {
        let s = format!("{}", CurrencyPair::BtcEth);
        assert_eq!(s, "BTC_ETH");
    }
}
