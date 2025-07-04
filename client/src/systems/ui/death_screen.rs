use crate::components::network::GameData;
use crate::net::NetworkClient;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DeathState {
    pub is_dead: bool,
}

#[derive(Resource, Default)]
pub struct DamageOverlayState {
    pub show_overlay: bool,
    pub timer: Timer,
}

impl DamageOverlayState {
    pub fn trigger_damage_flash(&mut self) {
        self.show_overlay = true;
        self.timer = Timer::from_seconds(0.2, TimerMode::Once); // 200ms flash
    }
}

#[derive(Component)]
pub struct DeathScreenUI;

#[derive(Component)]
pub struct DeathText;

#[derive(Component)]
pub struct DamageOverlay;

pub fn setup_death_screen(mut commands: Commands) {
    // Create death screen overlay (initially hidden)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Dark overlay
            Visibility::Hidden,                                // Start hidden
            DeathScreenUI,
        ))
        .with_children(|parent| {
            // "YOU DIED" text
            parent.spawn((
                Text::new("YOU DIED"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)), // Red text
                DeathText,
            ));

            // Instructions text
            parent.spawn((
                Text::new("Press R to respawn"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)), // Gray text
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(100.0),
                    ..default()
                },
            ));
        });

    // Create damage overlay (initially hidden)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            // No z_index needed, will be above other elements
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.3)), // Bright red with transparency
        Visibility::Hidden,                                // Start hidden
        DamageOverlay,
    ));
}

pub fn handle_death_screen(
    death_state: Res<DeathState>,
    mut death_screen_query: Query<&mut Visibility, With<DeathScreenUI>>,
) {
    for mut visibility in death_screen_query.iter_mut() {
        *visibility = if death_state.is_dead {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn handle_damage_overlay(
    time: Res<Time>,
    mut damage_state: ResMut<DamageOverlayState>,
    mut damage_overlay_query: Query<&mut Visibility, With<DamageOverlay>>,
) {
    if damage_state.show_overlay {
        damage_state.timer.tick(time.delta());

        if damage_state.timer.finished() {
            damage_state.show_overlay = false;
            // Hide the overlay
            for mut visibility in damage_overlay_query.iter_mut() {
                *visibility = Visibility::Hidden;
            }
        } else {
            // Show the overlay
            for mut visibility in damage_overlay_query.iter_mut() {
                *visibility = Visibility::Visible;
            }
        }
    }
}

pub fn update_death_state(game_data: Res<GameData>, mut death_state: ResMut<DeathState>) {
    // Update death state based on local player status
    if let Some(my_id) = &game_data.my_id {
        if let Some(my_player) = game_data.players.get(my_id) {
            death_state.is_dead = !my_player.is_alive;
        }
    }
}

pub fn handle_manual_respawn(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    death_state: Res<DeathState>,
    network: Res<NetworkClient>,
    _game_data: Res<GameData>,
) {
    // Only allow respawn when dead and R key is pressed
    if death_state.is_dead && keyboard_input.just_pressed(KeyCode::KeyR) {
        network.send_respawn();
        println!("Manual respawn requested - sending to server...");
    }
}

pub fn disable_movement_when_dead(
    death_state: Res<DeathState>,
    mut player_query: Query<&mut Transform, With<crate::components::player::Player>>,
) {
    if death_state.is_dead {
        // Keep player at current position when dead
        // Movement systems will be blocked by checking death state
        for mut _transform in player_query.iter_mut() {
            // Transform is frozen by not updating it
        }
    }
}
