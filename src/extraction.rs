use crate::*;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::Extract;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;

/// The z depth the stat bar sprites are drawn with.
const DEFAULT_Z_DEPTH: f32 = 990.0;

#[allow(clippy::type_complexity)]
pub(crate) fn extract_stat_bars<V: TypePath>(
    extraction: Extract<(
        Option<Res<StatbarDepth>>,
        Query<(
            Entity,
            &Statbar<V>,
            Option<&StatbarBorder<V>>,
            &GlobalTransform,
            &InheritedVisibility,
        )>,
    )>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    mut commands: Commands,
) {
    let mut new_translation;
    let (depth, query) = &*extraction;
    for (id, bar, border, global_transform, computed_visibility) in query.iter() {
        if bar.hide || !computed_visibility.get() {
            continue;
        }
        let (major_axis, minor_axis) = if bar.vertical {
            (Vec2::Y, Vec2::X)
        } else {
            (Vec2::X, Vec2::Y)
        };

        let value = bar.value;
        let length = bar.length;
        let thickness = bar.thickness;
        new_translation = global_transform.translation();
        let z = depth
            .as_ref()
            .map(|depth| depth.0)
            .unwrap_or(DEFAULT_Z_DEPTH);
        new_translation.z = z;
        new_translation.x += bar.displacement.x;
        new_translation.y += bar.displacement.y;
        let size = length * major_axis + thickness * minor_axis;
        if let Some(border) = border {
            let border_size = vec2(
                size.x + border.left + border.right,
                size.y + border.bottom + border.top,
            );
            extracted_sprites.sprites.insert(
                commands.spawn_empty().id(),
                ExtractedSprite {
                    transform: GlobalTransform::from_translation(new_translation),
                    color: border.color.to_linear(),
                    rect: None,
                    custom_size: Some(border_size),
                    image_handle_id: AssetId::default(),
                    flip_x: false,
                    flip_y: false,
                    anchor: Default::default(),
                    original_entity: Some(id),
                },
            );
        }

        // draw bar back
        if value < 1.0 {
            new_translation.z = z + 1.0;
            extracted_sprites.sprites.insert(
                commands.spawn_empty().id(),
                ExtractedSprite {
                    transform: GlobalTransform::from_translation(new_translation),
                    color: bar.empty_color.to_linear(),
                    rect: None,
                    custom_size: Some(size),
                    image_handle_id: AssetId::default(),
                    flip_x: false,
                    flip_y: false,
                    anchor: Default::default(),
                    original_entity: Some(id),
                },
            );
        }

        // draw bar
        if 0.0 < value {
            let value = value.clamp(0., 1.);
            let bar_size = value * length * major_axis + thickness * minor_axis;
            let direction = if bar.reverse { -1. } else { 1. };
            new_translation += direction * 0.5 * length * (value - 1.) * major_axis.extend(0.);
            new_translation.z = z + 2.0;
            extracted_sprites.sprites.insert(
                commands.spawn_empty().id(),
                ExtractedSprite {
                    transform: GlobalTransform::from_translation(new_translation),
                    color: bar.color.to_linear(),
                    rect: None,
                    custom_size: Some(bar_size),
                    image_handle_id: AssetId::default(),
                    flip_x: false,
                    flip_y: false,
                    anchor: Default::default(),
                    original_entity: Some(id),
                },
            );
        }
    }
}
