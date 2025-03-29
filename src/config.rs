/// ゲームエンジンの設定情報をまとめた構造体。
/// 利用者はこれを通してウィンドウサイズやタイトル、FPS などの設定を行います。
#[derive(Debug, Clone)]
pub struct GameConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub title: String,
    pub target_fps: u32,
    // 将来的にフルスクリーン設定や音量設定なども追加可能
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            title: "Mothra Engine".to_string(),
            target_fps: 60,
        }
    }
}
