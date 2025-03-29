use std::collections::HashMap;
use crate::renderer::TextureHandle;

use std::rc::Rc;

/// エンティティID（ただの整数）
pub type Entity = u32;

/// Entityの2D座標とサイズ情報
#[derive(Clone, Copy)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// テクスチャを参照する Sprite コンポーネント（共有参照）
pub struct Sprite {
    pub texture: Rc<TextureHandle>,
}

/// World は Entity / Component を保持・操作する構造体
pub struct World {
    next_entity: Entity,
    transforms: HashMap<Entity, Transform>,
    sprites: HashMap<Entity, Sprite>,
}

impl World {
    /// 新しいワールドを作成
    pub fn new() -> Self {
        Self {
            next_entity: 0,
            transforms: HashMap::new(),
            sprites: HashMap::new(),
        }
    }

    /// 新しい Entity を生成して返す
    pub fn spawn(&mut self) -> Entity {
        let id = self.next_entity;
        self.next_entity += 1;
        id
    }

    /// Entity に Transform を追加
    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.transforms.insert(entity, transform);
    }

    /// Entity に Sprite を追加
    pub fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.sprites.insert(entity, sprite);
    }

    /// 描画対象のEntityを取得（TransformとSpriteを両方持っているもの）
    pub fn query_drawables(&self) -> Vec<(Transform, &TextureHandle)> {
        self.transforms
            .iter()
            .filter_map(|(&e, t)| {
                self.sprites.get(&e).map(|s| (*t, s.texture.as_ref()))
            })
            .collect()
    }
}
