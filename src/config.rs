/// ゲームエンジン全体の設定情報をまとめた構造体。
#[derive(Debug, Clone)]
pub struct GameConfig {
    /// ウィンドウの幅（物理サイズ）
    pub window_width: u32,
    /// ウィンドウの高さ（物理サイズ）
    pub window_height: u32,
    /// ゲームの論理解像度の幅
    pub logical_width: u32,
    /// ゲームの論理解像度の高さ
    pub logical_height: u32,
    /// ウィンドウのタイトル
    pub title: String,
    /// 目標とするFPS
    pub target_fps: u32,
    /// stretch_mode が true の場合、ウィンドウサイズに合わせて描画内容も拡大縮小する
    /// false の場合、論理解像度を固定して表示する（letterbox 状態になる）
    pub stretch_mode: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            logical_width: 800,
            logical_height: 600,
            title: "Mothra Engine".to_string(),
            target_fps: 60,
            stretch_mode: false, // デフォルトは固定（論理解像度維持）
        }
    }
}
