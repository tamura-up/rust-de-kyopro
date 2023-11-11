use std::collections::BTreeSet;

/// 区間を管理するデータ構造
/// TODO: 名称要検討 (というか、すでに名前ついててもおかしくないと思う)
///
/// 以下、追加された区間を"保有"、保有する区間を削除することを"消費"と呼びます
///
/// ## Example
/// ```
/// use kyopro_data_stractures::range_space::RangeSpace;
/// let mut rs = RangeSpace::new(1001000100i32);
/// // [10, 100) を追加
/// rs.add_range(10, 100);
/// // 2以上の最小保有値を探す
/// assert_eq!(rs.find_first_point(2), Some(10));
///
/// // 100以上の最小保有値を探すが、存在しないので None
/// assert_eq!(rs.find_first_point(100), None);
///
/// // [45, 55) を消費
/// let _ = rs.use_range(45, 55);
///
/// // 50以上の最小保有値を探す、55 が見つかる
/// assert_eq!(rs.find_first_point(50), Some(55));
/// assert_eq!(rs.find_first_range(50), Some((55, 100)));
/// ```
///
pub struct RangeSpace<T>
where
    T: Copy + Ord,
{
    inf: T,
    st: BTreeSet<(T, T)>,
}

impl<T> RangeSpace<T>
where
    T: Copy + Ord,
{
    /// `inf`: 適当な無限を表す値
    /// new 直後は保有する区間はありません
    pub fn new(inf: T) -> Self {
        RangeSpace {
            inf,
            st: BTreeSet::new(),
        }
    }

    /// 保有する x 以上の最小 range を返します
    pub fn find_first_range(&self, x: T) -> Option<(T, T)> {
        let mut range = self.st.range((
            std::ops::Bound::Included((x, self.inf)),
            std::ops::Bound::Unbounded,
        ));

        if let Some(&(r, l)) = range.next() {
            Some((l, r))
        } else {
            None
        }
    }
    /// 保有する x 以上の最小値を返します
    pub fn find_first_point(&self, x: T) -> Option<T> {
        if let Some((l, _)) = self.find_first_range(x) {
            if x <= l {
                Some(l)
            } else {
                Some(x)
            }
        } else {
            None
        }
    }

    /// 区間 [l, r) を追加します
    pub fn add_range(&mut self, l: T, r: T) -> Result<(), String> {
        if r > self.inf {
            return Err("r が inf を超えています".to_string());
        }
        let mut after_range = self.st.range((
            std::ops::Bound::Included((l, self.inf)),
            std::ops::Bound::Unbounded,
        ));
        if let Some(&(nxt_r, nxt_l)) = after_range.next() {
            // 区間が重複していない
            if r > nxt_l {
                return Err("区間が重複しています".to_string());
            }

            if r == nxt_l {
                // merge
                self.st.remove(&(nxt_r, nxt_l));
                return self.add_range(l, nxt_r);
            }
        }

        let mut before_range = self.st.range((
            std::ops::Bound::Unbounded,
            std::ops::Bound::Included((l, self.inf)),
        ));
        if let Some(&(pre_r, pre_l)) = before_range.next_back() {
            if pre_r > l {
                return Err("区間が重複しています".to_string());
            }
            if pre_r == l {
                // merge
                self.st.remove(&(pre_r, pre_l));
                return self.add_range(pre_l, r);
            }
        }
        self.st.insert((r, l));
        return Ok(());
    }

    /// 区間 [l, r) を消費します
    pub fn use_range(&mut self, l: T, r: T) -> Result<(), String> {
        let mut range = self.st.range((
            std::ops::Bound::Included((l, self.inf)),
            std::ops::Bound::Unbounded,
        ));
        if let Some(&(sr, sl)) = range.next() {
            if sl <= l && r <= sr {
                self.st.remove(&(sr, sl));
                let mut _add = |l: T, r: T| {
                    if l < r {
                        self.st.insert((r, l));
                    }
                };
                _add(sl, l);
                _add(r, sr);
                return Ok(());
            }
        }
        Err("not found space".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::RangeSpace;

    #[test]
    fn test_add_space() {
        let mut rs = RangeSpace::new(10000i32);
        let _ = rs.add_range(1, 3);
        assert!(rs.st.contains(&(3, 1)));
        assert_eq!(rs.st.len(), 1);

        // 右側隣接区間に追加。マージされる
        let _ = rs.add_range(3, 5);
        assert!(rs.st.contains(&(5, 1)));
        assert_eq!(rs.st.len(), 1);

        let _ = rs.add_range(8, 10);
        assert!(rs.st.contains(&(10, 8)));
        assert_eq!(rs.st.len(), 2);

        // 左側隣接区間に追加。マージされる
        let _ = rs.add_range(6, 8);
        assert!(rs.st.contains(&(5, 1)));
        assert!(rs.st.contains(&(10, 6)));
        assert_eq!(rs.st.len(), 2);

        // 中央空き区間に追加。全体がマージされる
        let _ = rs.add_range(5, 6);
        assert!(rs.st.contains(&(10, 1)));
        assert_eq!(rs.st.len(), 1);
    }
    #[test]
    fn test_add_space_err() {
        let mut rs = RangeSpace::new(10000i32);
        let _ = rs.add_range(10, 20);
        assert!(rs.add_range(10, 11).is_err());
        assert!(rs.add_range(19, 20).is_err());

        assert!(rs.add_range(9, 10).is_ok());
        assert!(rs.add_range(20, 21).is_ok());
    }

    #[test]
    fn test_use_range() {
        let mut rs = RangeSpace::new(10000i32);
        let _ = rs.add_range(1, 10);
        assert!(rs.use_range(1, 2).is_ok());
        assert!(rs.st.contains(&(10, 2)));
        assert_eq!(rs.st.len(), 1);

        assert!(rs.use_range(3, 5).is_ok());
        assert!(rs.st.contains(&(10, 5)));
        assert!(rs.st.contains(&(3, 2)));
        assert_eq!(rs.st.len(), 2);

        assert!(rs.use_range(9, 10).is_ok());
        assert!(rs.st.contains(&(9, 5)));
        assert!(rs.st.contains(&(3, 2)));
        assert_eq!(rs.st.len(), 2);

        // 使用済み区間を指定した場合は err
        assert!(rs.use_range(1, 2).is_err());
    }
    #[test]
    fn test_first_point() {
        let mut rs = RangeSpace::new(10000i32);
        let _ = rs.add_range(3, 5);
        let _ = rs.add_range(8, 15);
        assert_eq!(rs.find_first_point(2), Some(3));
        assert_eq!(rs.find_first_point(3), Some(3));
        assert_eq!(rs.find_first_point(5), Some(8));
        assert_eq!(rs.find_first_point(10), Some(10));
        assert_eq!(rs.find_first_point(15), None);
    }
}
