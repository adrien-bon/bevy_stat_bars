mod extraction;

use bevy::{prelude::*, reflect::TypePath};
use std::marker::PhantomData;

/// Insert as a resource to set z depth of Statbars
#[derive(Resource)]
pub struct StatbarDepth(pub f32);

/// Implement `StatbarObservable` for a component you want to visualise with a stat bar.
/// Should return a value between 0.0 (= empty) and 1.0 (= full).
/// If the value is larger or smaller it is clamped before rendering.
pub trait StatbarObservable {
    fn get_statbar_value(&self) -> f32;
}

/// Insert this component to observe components from another entity.
/// Does not have a generic parameter for a marker component.
///
/// Overrides any local observeable components.
/// Overriden by StatbarObserveParent.
///
/// Does not have a marker component because I think it would be very confusing to have
/// an entity with three statbars observing components on three other entities.
/// If you really want a many to one capability, it should be trivial to write your own system.
#[derive(Component, Reflect)]
pub struct StatbarObserveEntity(pub Entity);

/// Insert this component to observe components from the entities parent.
/// Overrides any local obversable components and StatbarObservedEntity.
#[derive(Component, Reflect)]
pub struct StatbarObserveParent;

// observe a resource that implements 'StatbarOversable'
#[derive(Reflect)]
pub struct StatbarObserveResource<T>
where
    T: 'static,
{
    #[reflect(ignore)]
    #[doc(hidden)]
    pub _phantom: PhantomData<fn() -> T>,
}

impl<T> Default for StatbarObserveResource<T>
where
    T: 'static,
{
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

/// Insert this component to add a statbar to an entity.
/// Multiple statbars can be inserted on a single entity by using different marker components.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Statbar<T = ()>
where
    T: TypePath + 'static,
{
    /// color of the full part of the bar
    pub color: Color,
    /// color of the empty part of the bar
    pub empty_color: Color,
    /// length of the bar
    pub length: f32,
    /// thickness of the bar
    pub thickness: f32,
    /// absolute displacement from the GlobalTransform's position
    /// not part of the transform hierarchy, won't be scaled or rotated.
    pub displacement: Vec2,
    /// false => horizontally orientated bar,
    /// true => vertically orientated bar,
    pub vertical: bool,
    /// false =>
    /// * horizontal bar increasing from left to right
    /// * vertical bar increasing from bottom to top
    /// true =>
    /// * horizontal bar increasing from right to left
    /// * vertical bar increasing from bottom to top
    pub reverse: bool,
    /// if true, do not draw
    pub hide: bool,
    /// value of bar
    /// * 0.0 => bar entirely colored with empty color
    /// * 0.75 => bar three quarters full color, one quarter empty color
    /// * 1.0 => bar entity colored with full color
    pub value: f32,
    #[reflect(ignore)]
    #[doc(hidden)]
    pub _phantom: PhantomData<fn() -> T>,
}

impl<T: TypePath> Default for Statbar<T> {
    fn default() -> Self {
        Self {
            color: Color::from(bevy::color::palettes::css::YELLOW),
            empty_color: Color::srgb(0.2, 0.2, 0.0),
            length: 100.,
            thickness: 16.,
            displacement: Vec2::ZERO,
            vertical: false,
            reverse: false,
            hide: false,
            value: 0.75,
            _phantom: PhantomData,
        }
    }
}

/// Adds a border around the corresponding Statbar
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct StatbarBorder<T>
where
    T: 'static,
{
    /// color of the border
    pub color: Color,
    /// thickness of the border on the left
    left: f32,
    /// thickness of the border on the right
    right: f32,
    /// thickness of the border on the bottom
    bottom: f32,
    /// thickness of the border on the top
    top: f32,
    #[reflect(ignore)]
    phantom: PhantomData<fn() -> T>,
}

impl<T> StatbarBorder<T>
where
    T: 'static,
{
    /// A StarbarBorder with the same thickness on all four sides
    pub fn all(color: Color, thickness: f32) -> Self {
        Self {
            color,
            left: thickness,
            right: thickness,
            bottom: thickness,
            top: thickness,
            phantom: PhantomData,
        }
    }
}

impl<T> Default for StatbarBorder<T>
where
    T: 'static,
{
    fn default() -> Self {
        Self::all(Color::WHITE, 2.0)
    }
}

/// Linearly interpolate the value of the bar color
/// between `min` and `max` using the value of the Statbar
/// * statbar.value == 0. => statbar.color == min
/// * statbar.value == 1. => statbar.color == max
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct StatbarColorLerp<T>
where
    T: 'static,
{
    /// bar color when value is 0.0
    pub min: Color,
    /// bar color when value is 1.0
    pub max: Color,
    #[reflect(ignore)]
    phantom: PhantomData<fn() -> T>,
}

impl<T> Default for StatbarColorLerp<T>
where
    T: 'static,
{
    fn default() -> Self {
        Self {
            min: Color::from(bevy::color::palettes::css::RED),
            max: Color::from(bevy::color::palettes::css::GREEN),
            phantom: Default::default(),
        }
    }
}

impl<T> StatbarColorLerp<T>
where
    T: 'static,
{
    pub fn new(min: Color, max: Color) -> Self {
        Self {
            min,
            max,
            phantom: Default::default(),
        }
    }
}

/// Change the statbar color depending on the value of the statbar's subject
///
/// Could be used for a health bar that
/// turns to red when the character has less than 25% health remaining.
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct StatbarColorSwitch<T>
where
    T: 'static,
{
    /// * `statbar.value <= pivot`
    ///     => sets bar's color to `low`
    /// * `piviot < statbar.value
    ///     => sets bar's color to `high`
    pub pivot: f32,
    /// statbar color when the statbar's value is less than or equal to pivot
    pub low: Color,
    /// statbar color when the statbar's value is greater than pivot
    pub high: Color,
    #[reflect(ignore)]
    phantom: PhantomData<fn() -> T>,
}

impl<T> Default for StatbarColorSwitch<T>
where
    T: 'static,
{
    fn default() -> Self {
        Self {
            pivot: 0.25,
            low: Color::from(bevy::color::palettes::css::RED),
            high: Color::from(bevy::color::palettes::css::GREEN),
            phantom: Default::default(),
        }
    }
}

impl<T> StatbarColorSwitch<T>
where
    T: 'static,
{
    pub fn new(pivot: f32, low: Color, high: Color) -> Self {
        Self {
            pivot,
            low,
            high,
            phantom: Default::default(),
        }
    }
}

fn switch_stat_bar_colors<T: TypePath>(
    mut color_switch_query: Query<
        (&mut Statbar<T>, &mut StatbarColorSwitch<T>),
        Changed<Statbar<T>>,
    >,
) where
    T: 'static,
{
    color_switch_query
        .iter_mut()
        .for_each(|(mut bar, switcher)| {
            bar.color = if bar.value <= switcher.pivot {
                switcher.low
            } else {
                switcher.high
            };
        });
}

fn lerp_stat_bar_colors<T: TypePath>(
    mut color_lerp_query: Query<(&mut Statbar<T>, &mut StatbarColorLerp<T>), Changed<Statbar<T>>>,
) where
    T: 'static,
{

    color_lerp_query.iter_mut().for_each(|(mut bar, lerper)| {
        bar.color = lerper.min.mix(&lerper.max, bar.value);
    });
}

fn update_statbar_values<T: TypePath>(
    mut statbar_query: Query<
        (&mut Statbar<T>, &T),
        (
            Changed<T>,
            Without<StatbarObserveParent>,
            Without<StatbarObserveEntity>,
        ),
    >,
) where
    T: Component + StatbarObservable,
{
    statbar_query.iter_mut().for_each(|(mut statbar, value)| {
        statbar.value = value.get_statbar_value();
    });
}

fn update_statbar_values_from_parents<T: TypePath>(
    mut statbar_query: Query<
        (&mut Statbar<T>, &Parent),
        (With<StatbarObserveParent>, Without<StatbarObserveEntity>),
    >,
    parent_value_query: Query<&T, Changed<T>>,
) where
    T: Component + StatbarObservable,
{
    statbar_query.iter_mut().for_each(|(mut statbar, parent)| {
        if let Ok(value) = parent_value_query.get(parent.get()) {
            statbar.value = value.get_statbar_value();
        }
    });
}

fn update_statbar_values_from_other<T: TypePath>(
    mut statbar_query: Query<
        (&mut Statbar<T>, &StatbarObserveEntity),
        Without<StatbarObserveParent>,
    >,
    other_value_query: Query<&T, Changed<T>>,
) where
    T: Component + StatbarObservable,
{
    statbar_query
        .iter_mut()
        .for_each(|(mut statbar, &StatbarObserveEntity(target))| {
            if let Ok(value) = other_value_query.get(target) {
                statbar.value = value.get_statbar_value();
            }
        });
}

fn update_statbar_from_resource<T: TypePath + Resource>(
    resource: Res<T>,
    mut statbar_query: Query<&mut Statbar<T>>,
) where
    T: StatbarObservable + 'static + Send + Sync,
{
    if resource.is_changed() {
        statbar_query.iter_mut().for_each(|mut statbar| {
            statbar.value = resource.get_statbar_value();
        });
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum StatbarSystem {
    UpdateValues,
    UpdateColors,
    ExtractSprites,
}

pub trait RegisterStatbarSubject {
    fn add_statbar_component_observer<T: StatbarObservable + Component + TypePath>(
        &mut self,
    ) -> &mut Self;
    fn add_statbar_resource_observer<
        T: TypePath + Resource + StatbarObservable + 'static + Send + Sync,
    >(
        &mut self,
    ) -> &mut Self;
    fn add_standalone_statbar<T: TypePath + 'static>(&mut self) -> &mut Self;
}

impl RegisterStatbarSubject for App {
    fn add_statbar_component_observer<T: StatbarObservable + Component + TypePath>(
        &mut self,
    ) -> &mut Self {
        if let Some(render_app) = self.get_sub_app_mut(bevy::render::RenderApp) {
            render_app.add_systems(
                ExtractSchedule,
                extraction::extract_stat_bars::<T>
                    .after(bevy::sprite::SpriteSystem::ExtractSprites),
            );
        }

        self.register_type::<Statbar<T>>()
            .register_type::<StatbarBorder<T>>()
            .register_type::<StatbarColorLerp<T>>()
            .register_type::<StatbarColorSwitch<T>>()
            .configure_sets(
                PostUpdate,
                (StatbarSystem::UpdateValues, StatbarSystem::UpdateColors).chain(),
            )
            .add_systems(
                PostUpdate,
                (
                    (
                        update_statbar_values::<T>,
                        update_statbar_values_from_other::<T>,
                        update_statbar_values_from_parents::<T>,
                    )
                        .in_set(StatbarSystem::UpdateValues),
                    (switch_stat_bar_colors::<T>, lerp_stat_bar_colors::<T>)
                        .in_set(StatbarSystem::UpdateColors),
                ),
            )
    }

    fn add_statbar_resource_observer<
        T: TypePath + Resource + StatbarObservable + 'static + Send + Sync,
    >(
        &mut self,
    ) -> &mut Self {
        if let Some(render_app) = self.get_sub_app_mut(bevy::render::RenderApp) {
            render_app.add_systems(
                ExtractSchedule,
                extraction::extract_stat_bars::<T>
                    .after(bevy::sprite::SpriteSystem::ExtractSprites),
            );
        }

        self.register_type::<Statbar<T>>()
            .register_type::<StatbarBorder<T>>()
            .register_type::<StatbarColorLerp<T>>()
            .register_type::<StatbarColorSwitch<T>>()
            .configure_sets(
                PostUpdate,
                (StatbarSystem::UpdateValues, StatbarSystem::UpdateColors).chain(),
            )
            .add_systems(
                PostUpdate,
                (
                    (update_statbar_from_resource::<T>,).in_set(StatbarSystem::UpdateValues),
                    (switch_stat_bar_colors::<T>, lerp_stat_bar_colors::<T>)
                        .in_set(StatbarSystem::UpdateColors),
                ),
            )
    }

    fn add_standalone_statbar<T: TypePath + 'static>(&mut self) -> &mut Self {
        if let Some(render_app) = self.get_sub_app_mut(bevy::render::RenderApp) {
            render_app.add_systems(
                ExtractSchedule,
                extraction::extract_stat_bars::<T>
                    .after(bevy::sprite::SpriteSystem::ExtractSprites),
            );
        }

        self.register_type::<Statbar<T>>()
            .register_type::<StatbarBorder<T>>()
            .register_type::<StatbarColorLerp<T>>()
            .register_type::<StatbarColorSwitch<T>>()
            .add_systems(
                PostUpdate,
                ((switch_stat_bar_colors::<T>, lerp_stat_bar_colors::<T>)
                    .in_set(StatbarSystem::UpdateColors),),
            )
    }
}
