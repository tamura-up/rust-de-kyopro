///! 状態を N 進数で管理するためのヘルパークラスです。

/// 状態を N 進数で管理するためのヘルパークラスです。
/// bit 全探索のように状態を管理をしたいが、状態数が 3 以上ある場合に使用します。
///
/// # Example
///
/// ```
/// use kyopro_base_state::BaseStateBuilder;
/// // 3 状態をもつ 5 要素の状態を操作する builder
/// let builder = BaseStateBuilder::new(5, 3);
/// // 最初はすべての要素の状態が 0
/// let state = 0;
///
/// // 0 番目を 2 に変更
/// let state = builder.modify(state, 0, 2);
/// assert_eq!(builder.decode(state), vec![2, 0, 0, 0, 0]);
/// assert_eq!(state, 2);
///
/// // 3 番目を 1 に変更
/// let state = builder.modify(state, 3, 1);
/// assert_eq!(builder.decode(state), vec![2, 0, 0, 1, 0]);
/// assert_eq!(state, 2 + (3 * 3 * 3) * 1);
///```
pub struct BaseStateBuilder {
    MX_SIZE: usize,
    B: usize,
    powb: Vec<usize>,
}
impl BaseStateBuilder {
    /// + `mx_size`: 管理対象の最大要素数
    /// + `base`: とりうる状態数 `> 1`
    pub fn new(mx_size: usize, base: usize) -> Self {
        assert!(base > 1);
        let mut powb = vec![1];
        for i in 0..mx_size - 1 {
            powb.push(powb[i] * base);
        }
        Self { MX_SIZE: mx_size, B: base, powb }
    }
    /// state の i 番目要素の値を変更します
    pub fn modify(&self, mut state: usize, i: usize, v: usize) -> usize {
        let b = self.powb[i];
        let current = (state / b) % self.B;
        state -= b * current;
        state += b * v;
        state
    }
    /// vals を state にエンコードします
    pub fn encode(&self, vals: &[usize]) -> usize {
        vals.iter().enumerate().fold(0usize, |s, (i, &v)| s + self.powb[i] * v)
    }
    /// state を vector にデコードします
    pub fn decode(&self, state: usize) -> Vec<usize> {
        (0..self.MX_SIZE).map(|i| (state / self.powb[i]) % self.B).collect::<Vec<_>>()
    }
    /// state から i 番目要素の値を取得します
    pub fn get_val(&self, state: usize, i: usize) -> usize {
        (state / self.powb[i]) % self.B
    }
    /// state のとりうる最大値を返します
    pub fn get_max_state_value(&self) -> usize {
        self.powb.iter().fold(0usize, |s, &v| s + v * (self.B - 1))
    }
}

#[test]
fn test_modify() {
    let bui = BaseStateBuilder::new(5, 5);
    let s = bui.modify(0, 0, 2);
    assert_eq!(s, 2);
    assert_eq!(0, bui.modify(2, 0, 0));
    let s = bui.modify(0, 1, 2);
    assert_eq!(s, 5 * 2);
    assert_eq!(0, bui.modify(5 * 2, 1, 0));
    assert_eq!(2, bui.modify(2 + 5 * 2, 1, 0));
    assert_eq!(2 + 5 * 1 + 25 * 1, bui.modify(2 + 5 * 2 + 25 * 1, 1, 1));
}
