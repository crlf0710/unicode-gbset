mod tables;

#[cfg(test)]
use std::ops::Not;

#[doc(inline)]
pub use tables::UNICODE_VERSION;

#[doc(inline)]
pub use tables::gbset_data::GBSet;

impl GBSet {
    pub fn is_gb0(self) -> bool {
        matches!(
            self,
            GBSet::GB0L1
                | GBSet::GB0L1AndSuppl1
                | GBSet::GB0L1AndSuppl3
                | GBSet::GB0L2
                | GBSet::GB0L2AndSuppl1
        )
    }

    pub fn is_gb0_l1(self) -> bool {
        matches!(
            self,
            GBSet::GB0L1 | GBSet::GB0L1AndSuppl1 | GBSet::GB0L1AndSuppl3
        )
    }

    pub fn is_gb0_l2(self) -> bool {
        matches!(self, GBSet::GB0L2 | GBSet::GB0L2AndSuppl1)
    }

    pub fn is_gbsuppl1(self) -> bool {
        matches!(
            self,
            GBSet::GB0L1AndSuppl1 | GBSet::GB0L2AndSuppl1 | GBSet::GBSuppl1
        )
    }

    pub fn is_gbsuppl3(self) -> bool {
        matches!(self, GBSet::GB0L1AndSuppl3 | GBSet::GBSuppl3)
    }

    pub fn is_gbsuppl5(self) -> bool {
        matches!(self, GBSet::GBSuppl5)
    }

    pub fn is_gbsuppl7(self) -> bool {
        matches!(self, GBSet::GBSuppl7)
    }

    pub fn is_gb(self) -> bool {
        !matches!(self, GBSet::Other)
    }
}

pub trait UnicodeGBSet {
    fn gb_set(&self) -> GBSet;
}

impl UnicodeGBSet for char {
    fn gb_set(&self) -> GBSet {
        tables::gbset_data::gbset_data_lookup(*self)
    }
}

#[cfg(test)]
fn uro_chars() -> impl Iterator<Item = char> {
    (0x4e00u32..=0x9fa5).flat_map(|x| char::try_from(x).ok())
}

#[test]
fn count_gb0_l1() {
    let gb0_l1_chars = uro_chars().filter(|x| x.gb_set().is_gb0_l1());
    assert_eq!(gb0_l1_chars.count(), 3755);
}

#[test]
fn count_gb0_l2() {
    let gb0_l2_chars = uro_chars().filter(|x| x.gb_set().is_gb0_l2());
    assert_eq!(gb0_l2_chars.count(), 3008);
}

#[test]
fn count_gbsupplfictional6() {
    let gbsuppl6_chars = uro_chars().filter(|x| {
        matches!(
            x.gb_set(),
            GBSet::GBSupplFictional6G7
                | GBSet::GBSupplFictional6G8
                | GBSet::GBSupplFictional6GH
                | GBSet::GBSupplFictional6GHZ
                | GBSet::GBSupplFictional6GK
                | GBSet::GBSupplFictional6GT
                | GBSet::GBSupplFictional6GU
        )
    });
    assert_eq!(gbsuppl6_chars.count(), 482);
}

#[test]
fn count_gbsuppl7() {
    let gbsuppl7_chars = uro_chars().filter(|x| x.gb_set().is_gbsuppl7());
    assert_eq!(gbsuppl7_chars.count(), 3778);
}

#[test]
fn count_nongb_uros() {
    let nongb_chars = uro_chars().filter(|x| x.gb_set().is_gb().not());
    assert_eq!(nongb_chars.count(), 0);
}
