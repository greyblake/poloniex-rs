use std::fmt;
use std::ascii::AsciiExt;

#[derive(Debug)]
pub enum Currency {
    Amp,
    Ardr,
    Bch,
    Bcn,
    Bcy,
    Bela,
    Blk,
    Btc,
    Btcd,
    Btm,
    Bts,
    Burst,
    Clam,
    Dash,
    Dcr,
    Dgb,
    Doge,
    Emc2,
    Etc,
    Eth,
    Exp,
    Fct,
    Fldc,
    Flo,
    Game,
    Gno,
    Gnt,
    Grc,
    Huc,
    Lbc,
    Lsk,
    Ltc,
    Maid,
    Naut,
    Nav,
    Neos,
    Nmc,
    Note,
    Nxc,
    Nxt,
    Omni,
    Pasc,
    Pink,
    Pot,
    Ppc,
    Rads,
    Rep,
    Ric,
    Sbd,
    Sc,
    Sjcx,
    Steem,
    Str,
    Strat,
    Sys,
    Usdt,
    Via,
    Vrc,
    Vtc,
    Xbc,
    Xcp,
    Xem,
    Xmr,
    Xpm,
    Xrp,
    Xvc,
    Zec
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_ascii_uppercase();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let btc = format!("{}", Currency::Btc);
        assert_eq!(btc, "BTC");

        let eth  = format!("{}", Currency::Eth);
        assert_eq!(eth, "ETH");
    }
}
